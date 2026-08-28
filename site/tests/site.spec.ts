import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

test("home has one clear hierarchy and no serious accessibility violations", async ({ page }) => {
  const errors: string[] = [];
  page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
  await page.goto("/");
  await expect(page).toHaveTitle(/History Export Sentinel/);
  await expect(page.locator("h1")).toHaveCount(1);
  await expect(page.locator("main")).toHaveCount(1);
  await expect(page.getByRole("img")).toHaveAttribute("alt", /browser database cartridge/i);
  const results = await new AxeBuilder({ page }).analyze();
  const serious = results.violations.filter((violation) => ["serious", "critical"].includes(violation.impact ?? ""));
  expect(serious).toEqual([]);
  expect(errors).toEqual([]);
});

test("recorded demo supports pointer and arrow-key state changes", async ({ page }) => {
  await page.goto("/#demo");
  const first = page.getByRole("tab", { name: "Verified export" });
  const empty = page.getByRole("tab", { name: "Empty history" });
  await first.focus();
  await page.keyboard.press("ArrowRight");
  await expect(empty).toHaveAttribute("aria-selected", "true");
  await expect(page.locator("#result-label")).toHaveText("Exit 10 / no history");
  await page.getByRole("tab", { name: "Locked profile" }).click();
  await expect(page.locator("#terminal-output")).toContainText("No partial archive was kept");
});

test("offline state is explicit and legal routes render", async ({ page }) => {
  await page.goto("/");
  await page.evaluate(() => window.dispatchEvent(new Event("offline")));
  await expect(page.locator("#offline-banner")).toBeVisible();
  for (const route of ["/privacy/", "/terms/"]) {
    await page.goto(route);
    await expect(page.locator("h1")).toHaveCount(1);
    const results = await new AxeBuilder({ page }).analyze();
    expect(results.violations.filter((item) => ["serious", "critical"].includes(item.impact ?? ""))).toEqual([]);
  }
});

test("390px layout has no horizontal overflow", async ({ page }) => {
  await page.goto("/");
  const overflow = await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth);
  expect(overflow).toBeLessThanOrEqual(1);
  await expect(page.getByRole("link", { name: "Install Sentinel" })).toBeVisible();
});
