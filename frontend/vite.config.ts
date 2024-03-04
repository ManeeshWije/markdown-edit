import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
    base: "/",
    plugins: [react()],
    define: {
        VITE_SERVER_URL: process.env.VITE_SERVER_URL
    },
    preview: {
        port: 3000,
        strictPort: true,
        host: true
    },
    server: {
        port: 3000,
        strictPort: true,
        host: true,
        origin: "http://0.0.0.0:3000"
    }
});
