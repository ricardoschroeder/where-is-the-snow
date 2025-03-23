import axios from "axios";

export async function getSnowData(city: string) {
  try {
    const response = await axios.get("/api/snowforecast", {
      params: { city },
    });
    return response.data;
  } catch (error) {
    console.error("Failed to fetch snow data:", error);
    throw error;
  }
}
