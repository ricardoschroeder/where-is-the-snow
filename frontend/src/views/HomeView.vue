<script setup lang="ts">
import { ref } from "vue";
import { formatDate } from "@/utils/date-utils";
import { getSnowData } from "@/services/weather-service";
import type { SnowData } from "@/models/snow-data";

const location = ref("");
const snowData = ref<SnowData>({} as SnowData);

const checkSnow = async () => {
  try {
    const response = await getSnowData(location.value);
    snowData.value = response;
  } catch (error) {
    alert("Failed to fetch snow data!");
  }
};
</script>

<template>
  <div>
    <input v-model="location" placeholder="Enter location" />
    <button @click="checkSnow">Check Snow</button>

    <div v-if="snowData">
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
    </div>
  </div>
</template>
