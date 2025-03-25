<script setup lang="ts">
// import { ref } from "vue";
// import { formatDate } from "@/utils/date-utils";
// import { getSnowData } from "@/services/weather-service";
// import type { SnowData } from "@/models/snow-data";

// const location = ref("");
// const snowData = ref<SnowData>({} as SnowData);

// const checkSnow = async () => {
//   try {
//     const response = await getSnowData(location.value);
//     snowData.value = response;
//   } catch (error) {
//     alert("Failed to fetch snow data!");
//   }
// };
import { onMounted, ref } from "vue";
import maplibregl from "maplibre-gl";
import "maplibre-gl/dist/maplibre-gl.css";
import { getCityCoordinates } from "@/services/weather-service";

const mapContainer = ref<HTMLElement | null>(null);
const map = ref<maplibregl.Map | null>(null);
const location = ref("");
const snowfallPoints = ref<any[]>([]); // Store snowfall data for the heatmap

const initMap = () => {
  if (!mapContainer.value) return;

  map.value = new maplibregl.Map({
    container: mapContainer.value,
    style:
      "https://api.maptiler.com/maps/bright-v2/style.json?key=P8Wm1QyEFk8ioA1IkksY",
    center: [4.899, 52.372], // Default to Amsterdam
    zoom: 5,
  });

  map.value.on("moveend", async () => {
    console.log("Map moved!");
    const bounds = getMapBounds();
    if (!bounds) return;

    // Call Open-Meteo API
    const lat = bounds.map((b: { lat: any }) => b.lat).join(",");
    const lon = bounds.map((b: { lng: any }) => b.lng).join(",");
    const openMeteoURL = `https://api.open-meteo.com/v1/forecast?latitude=${lat}&longitude=${lon}&daily=snowfall_sum&forecast_days=1`;

    const response = await fetch(openMeteoURL);
    const data = await response.json();
    console.log("Snowfall Data:", data);
    snowfallPoints.value = Array.isArray(data)
      ? data.filter((d: any) => d !== null)
      : [data]; // Remove nulls
    updateHeatmap();
  });

  new maplibregl.NavigationControl();
};

const searchCity = async () => {
  if (!location.value) return;

  const data = await getCityCoordinates(location.value);
  console.log(map);
  if (data && map.value) {
    map.value.flyTo({ center: [data.longitude, data.latitude], zoom: 10 });
  } else {
    alert("City not found!");
  }
};

const getMapBounds = () => {
  if (!map.value) return;

  const bounds = map.value.getBounds();
  const coordinates = {
    northEast: bounds.getNorthEast(),
    southWest: bounds.getSouthWest(),
  };

  console.log("Map bounds:", coordinates);
  const sampleCoordinates = generateSampleCoordinates(
    coordinates.northEast,
    coordinates.southWest
  );
  console.log("Sample coordinates:", sampleCoordinates);
  return sampleCoordinates;
};

const generateSampleCoordinates = (
  ne: maplibregl.LngLat,
  sw: maplibregl.LngLat
) => {
  const step = 5; // Adjust step size (degrees) for more/fewer requests
  const points = [];

  for (let lat = sw.lat; lat <= ne.lat; lat += step) {
    for (let lng = sw.lng; lng <= ne.lng; lng += step) {
      points.push({ lat, lng });
    }
  }

  return points;
};

const updateHeatmap = () => {
  if (!map.value) return;

  // Remove old layer and source if they exist
  if (map.value.getLayer("snowfall-heatmap")) {
    map.value.removeLayer("snowfall-heatmap");
    map.value.removeSource("snowfall-data");
  }

  map.value.addSource("snowfall-data", {
    type: "geojson",
    data: {
      type: "FeatureCollection",
      features: snowfallPoints.value.map((point) => ({
        type: "Feature",
        geometry: {
          type: "Point",
          coordinates: [point.longitude, point.latitude],
        },
        properties: { snowfall: point.snowfall_sum },
      })),
    },
  });

  map.value.addLayer({
    id: "snowfall-heatmap",
    type: "heatmap",
    source: "snowfall-data",
    paint: {
      "heatmap-weight": [
        "interpolate",
        ["linear"],
        ["get", "snowfall"],
        0,
        0,
        50,
        1,
      ],
      "heatmap-intensity": 1,
      "heatmap-color": [
        "interpolate",
        ["linear"],
        ["heatmap-density"],
        0,
        "rgba(255, 255, 255, 0)",
        0.2,
        "rgba(173, 216, 230, 0.5)",
        0.4,
        "rgba(135, 206, 250, 0.7)",
        0.6,
        "rgba(30, 144, 255, 0.8)",
        0.8,
        "rgba(0, 0, 255, 0.9)",
      ],
      "heatmap-radius": 50,
      "heatmap-opacity": 1,
    },
  });
};

onMounted(initMap);
</script>

<template>
  <div>
    <input v-model="location" placeholder="Enter city" />
    <button @click="searchCity">Check Snow</button>

    <div class="map" ref="mapContainer"></div>
    <!-- <div v-if="snowData">
      <table>
        <thead>
          <tr>
            <th>Date</th>
            <th>Snowfall (cm)</th>
            <th>Temperature Min</th>
            <th>Temperature Max</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="(snowfall, index) in snowData.snowfall_sum" :key="index">
            <td>{{ formatDate(new Date(snowData.time[index])) }}</td>
            <td>{{ snowfall }}</td>
            <td>{{ snowData.temperature_2m_min[index] }}</td>
            <td>{{ snowData.temperature_2m_max[index] }}</td>
          </tr>
        </tbody>
      </table>
      <p v-if="snowData.snowfall_sum[0] > 0">
        It is snowing today {{ location }}! ❄️
      </p>
      <p v-else>It is not snowing today in {{ location }}.</p>
    </div> -->
  </div>
</template>
