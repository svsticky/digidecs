import { defineConfig } from 'vitest/config';
import vue from "@vitejs/plugin-vue";
import viteTsconfigPaths from 'vite-tsconfig-paths';

export default defineConfig({
  test: {
    globals: true,
    environment: 'jsdom',
    coverage: {
      reporter: ['text', 'json', 'html'],
    },
    include: ["src/tests/*.test.js"],
  },
  plugins: [vue(),viteTsconfigPaths()], 
});