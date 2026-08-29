import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";

const routes = ["/", "/demo/", "/privacy/", "/terms/", "/404.html"];

test("every route has its own metadata, shell, and accessible structure", async ({ page }) => {
  for (const route of routes) {
    const errors: string[] = [];
    page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
    await page.goto(route);
    await expect(page.locator("html")).toHaveAttribute("lang", "en");
    await expect(page.locator("main")).toHaveCount(1);
    await expect(page.locator("h1")).toHaveCount(1);
    await expect(page.locator("link[rel='canonical']")).toHaveCount(1);
    await expect(page.locator("meta[property='og:image']")).toHaveAttribute("content", /og-history-sentinel\.webp$/);
    await expect(page.locator("meta[name='twitter:card']")).toHaveAttribute("content", "summary_large_image");
    await expect(page.getByRole("link", { name: "History Export Sentinel home" }).first()).toBeVisible();
    await expect(page.getByRole("link", { name: "Privacy", exact: true }).last()).toBeVisible();
    await expect(page.getByRole("link", { name: "Terms", exact: true }).last()).toBeVisible();
    const serious = (await new AxeBuilder({ page }).analyze()).violations.filter((item) => ["serious", "critical"].includes(item.impact ?? ""));
    expect(serious, `${route} axe results`).toEqual([]);
    expect(errors, `${route} console errors`).toEqual([]);
  }
});

test("first screen states the job, audience, action, and three facts", async ({ page }) => {
  await page.goto("/");
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Export and verify browser history");
  await expect(page.getByText("For people archiving Firefox or Chromium history before changing a computer or account.")).toBeVisible();
  await expect(page.getByRole("link", { name: "Try it with sample data" })).toBeVisible();
  await expect(page.locator(".proof-strip li")).toHaveCount(3);
});

test("@claim:demo-sandbox opens in one click, isolates state, resets, and exits", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("link", { name: "Try it with sample data" }).click();
  await expect(page).toHaveURL(/\/demo\/$/);
  await expect(page.locator("#demo-banner")).toBeVisible();
  await expect(page.locator("#terminal-output")).toContainText("6 visits");
  await page.getByRole("tab", { name: "Empty history" }).click();
  expect(await page.evaluate(() => Object.keys(localStorage))).toEqual(["demo:sentinel:state"]);
  await page.getByRole("button", { name: "Reset demo" }).click();
  await expect(page.getByRole("tab", { name: "Verified export" })).toHaveAttribute("aria-selected", "true");
  await page.getByRole("tab", { name: "Blocked profile" }).click();
  await page.getByRole("link", { name: "Start for real" }).click();
  await expect(page).toHaveURL(/\/#install$/);
  expect(await page.evaluate(() => Object.keys(localStorage).filter((key) => key.startsWith("demo:")))).toEqual([]);
});

test("direct query demo path shows the isolated sample immediately", async ({ page }) => {
  await page.goto("/?demo=1");
  await expect(page).toHaveTitle("Demo — History Export Sentinel");
  await expect(page.locator("#demo-banner")).toBeVisible();
  await expect(page.locator("#terminal-output")).toContainText("DEMO EXPORT VERIFIED");
});

test("demo tabs support arrow, Home, and End keys", async ({ page }) => {
  await page.goto("/demo/");
  const verified = page.getByRole("tab", { name: "Verified export" });
  await verified.focus();
  await page.keyboard.press("ArrowRight");
  await expect(page.getByRole("tab", { name: "Empty history" })).toHaveAttribute("aria-selected", "true");
  await page.keyboard.press("End");
  await expect(page.getByRole("tab", { name: "Blocked profile" })).toHaveAttribute("aria-selected", "true");
  await page.keyboard.press("Home");
  await expect(verified).toHaveAttribute("aria-selected", "true");
});

test("internal routing updates history, title, focus, and announcement", async ({ page }) => {
  await page.goto("/");
  await page.getByRole("link", { name: "Privacy", exact: true }).first().click();
  await expect(page).toHaveURL(/\/privacy\/$/);
  await expect(page).toHaveTitle("Privacy — History Export Sentinel");
  await expect(page.locator("h1")).toBeFocused();
  await expect(page.locator(".route-announcer")).toHaveText("Privacy");
  await page.goBack();
  await expect(page).toHaveURL(/\/$/);
  await expect(page.locator("h1")).toBeFocused();
});

test("@claim:site-privacy all routes stay same-origin and set no cookies", async ({ page, context }) => {
  const remote: string[] = [];
  page.on("request", (request) => { if (new URL(request.url()).origin !== "http://127.0.0.1:4173") remote.push(request.url()); });
  for (const route of routes) await page.goto(route);
  expect(remote).toEqual([]);
  expect(await context.cookies()).toEqual([]);
  await page.goto("/");
  expect(await page.evaluate(() => Object.keys(localStorage))).toEqual(["demo:sentinel:state"]);
});

test("@claim:offline-reload precaches and reloads the demo offline", async ({ page, context }) => {
  await page.goto("/demo/");
  await page.evaluate(async () => { await navigator.serviceWorker.ready; });
  await page.reload();
  await expect.poll(() => page.evaluate(() => Boolean(navigator.serviceWorker.controller))).toBe(true);
  await context.setOffline(true);
  await page.reload();
  await expect(page.getByRole("heading", { level: 1 })).toHaveText("Try a verified history export");
  await expect(page.getByRole("heading", { name: "Six sample visits" })).toBeVisible();
  await context.setOffline(false);
});

test("offline event reveals a plain status message", async ({ page }) => {
  await page.goto("/demo/");
  await page.evaluate(() => window.dispatchEvent(new Event("offline")));
  await expect(page.locator("#offline-banner")).toBeVisible();
});

test("mobile layout has no overflow and every visible control is at least 44px", async ({ page }, testInfo) => {
  test.skip(testInfo.project.name !== "mobile-390", "mobile-only measurement");
  for (const route of routes) {
    await page.goto(route);
    expect(await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth), `${route} overflow`).toBeLessThanOrEqual(1);
    const undersized = await page.locator("a:visible, button:visible").evaluateAll((nodes) => nodes.map((node) => {
      const rect = node.getBoundingClientRect();
      return { text: node.textContent?.trim(), width: rect.width, height: rect.height };
    }).filter((item) => item.width < 44 || item.height < 44));
    expect(undersized, `${route} undersized controls`).toEqual([]);
  }
});

test("static configuration supplies a first-party 404 and security policy", async ({ request }) => {
  const config = await (await request.get("/staticwebapp.config.json")).json();
  expect(config.responseOverrides["404"].rewrite).toBe("/404.html");
  expect(config.globalHeaders["content-security-policy"]).toContain("frame-ancestors 'none'");
  expect(config.globalHeaders["referrer-policy"]).toBe("no-referrer");
});
