import wasm from "vite-plugin-wasm";
import { defineConfig } from 'vite';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    wasm(),
  ],
  build: {
    target: 'esnext',
  },
});
