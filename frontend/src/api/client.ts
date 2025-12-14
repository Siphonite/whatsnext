import axios from "axios";

const BACKEND_URL = import.meta.env.VITE_BACKEND_URL;

if (!BACKEND_URL) {
  console.error("VITE_BACKEND_URL is not defined");
} else {
  console.log("Backend URL:", BACKEND_URL);
}

export const apiClient = axios.create({
  baseURL: BACKEND_URL,
  timeout: 15000,
});
