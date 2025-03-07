import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import svgr from "vite-plugin-svgr";
import path from "path";
// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  root: "src",
  plugins: [svgr(), react()],
  clearScreen: false,
  resolve: {
    alias: {
      "@": path.resolve("./src"),
      "@root": path.resolve("."),
    },
  },
  define: {
    OS_PLATFORM: `"${process.platform}"`,
  },
  server: {
    port: 3000,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 3000,
        }
      : undefined,
    watch: {
      ignored: ["**/src-tauri/**"],
    },
  },
}));
