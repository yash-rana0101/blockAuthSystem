import { fileURLToPath, URL } from "url";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";
import environment from "vite-plugin-environment";
import dotenv from "dotenv";

dotenv.config({ path: "../../.env" });

export default defineConfig({
  define: {
    "process.env.DFX_NETWORK": JSON.stringify(process.env.DFX_NETWORK),
    "process.env.INTERNET_IDENTITY_CANISTER_ID": JSON.stringify(
      process.env.INTERNET_IDENTITY_CANISTER_ID
    ),
    "process.env.CANISTER_ID_AUTH_BACKEND": JSON.stringify(
      process.env.CANISTER_ID_AUTH_BACKEND
    ),
  },
  build: {
    outDir: "../auth_backend/www",
    emptyOutDir: true,
    rollupOptions: {
      input: "src/main.jsx",
    },
  },
  optimizeDeps: {
    esbuildOptions: {
      define: {
        global: "globalThis",
      },
    },
    include: ["@dfinity/agent"],
  },
  server: {
    hmr: {
      host: "localhost",
      port: 3000,
    },
    proxy: {
      "/api": {
        target: "http://127.0.0.1:4943",
        changeOrigin: true,
      },
    },
  },
  plugins: [react(), environment("all", { prefix: "" })],
  resolve: {
    alias: [
      {
        find: "declarations",
        replacement: fileURLToPath(new URL("../declarations", import.meta.url)),
      },
      {
        find: "src",
        replacement: fileURLToPath(new URL("./src", import.meta.url)),
      },
      {
        find: "components",
        replacement: fileURLToPath(
          new URL("./src/components", import.meta.url)
        ),
      },
    ],
  },
  assetsInclude: ["**/*.svg", "**/*.png", "**/*.jpg", "**/*.gif"],
});
