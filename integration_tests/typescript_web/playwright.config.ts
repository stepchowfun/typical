import { defineConfig } from '@playwright/test';

// Run the production build in the same way locally and in continuous integration.
export default defineConfig({
  use: {
    baseURL: 'http://127.0.0.1:5173',
  },
  webServer: {
    command: 'npm run preview -- --host 127.0.0.1 --port 5173',
    reuseExistingServer: false,
    url: 'http://127.0.0.1:5173',
  },
});
