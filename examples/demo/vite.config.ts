import { defineConfig } from 'vite'
import wasm from "vite-plugin-wasm";
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), wasm()],
})
