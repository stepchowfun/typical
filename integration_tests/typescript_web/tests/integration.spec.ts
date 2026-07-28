import { expect, test } from '@playwright/test';

// Verify that all integration scenarios finish successfully in a browser.
test('runs the browser integration tests', async ({ page }) => {
  const pageErrors: Error[] = [];

  // Preserve uncaught browser errors so the test reports them after the page finishes.
  page.on('pageerror', (error) => {
    pageErrors.push(error);
  });

  // Load the test harness and wait for its success status.
  await page.goto('/');
  await expect(page.locator('#app')).toHaveText('Integration tests passed.');

  // Ensure no uncaught error was hidden by an otherwise successful status update.
  expect(pageErrors).toEqual([]);
});
