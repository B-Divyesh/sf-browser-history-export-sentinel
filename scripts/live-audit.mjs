import AxeBuilder from "@axe-core/playwright";
import { chromium } from "@playwright/test";
import { mkdirSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";
import assert from "node:assert/strict";

const base = process.argv[2] ?? "https://browser-history-export-sentinel.sociobot.in";
const evidence = resolve(process.argv[3] ?? ".factory/evidence/live-polish-3");
mkdirSync(evidence, { recursive: true });

const expected = {
  "/": ["History Export Sentinel — verify browser history exports", "Export and verify browser history"],
  "/demo/": ["Demo — History Export Sentinel", "Try a verified history export"],
  "/privacy/": ["Privacy — History Export Sentinel", "Privacy"],
  "/terms/": ["Terms — History Export Sentinel", "Terms"]
};
const report = { base, checkedAt: new Date().toISOString(), routes: {}, requests: [], checks: [] };
const browser = await chromium.launch();

async function routeAudit(viewport, suffix) {
  const context = await browser.newContext({ viewport });
  const page = await context.newPage();
  const consoleErrors = [];
  page.on("console", (message) => { if (message.type() === "error") consoleErrors.push(message.text()); });
  page.on("pageerror", (error) => consoleErrors.push(String(error)));
  page.on("request", (request) => report.requests.push(request.url()));

  for (const [path, [title, heading]] of Object.entries(expected)) {
    const response = await page.goto(`${base}${path}`, { waitUntil: "networkidle" });
    assert.equal(response?.status(), 200, `${path} did not return 200`);
    assert.equal(await page.title(), title, `${path} title differs`);
    assert.equal(await page.locator("h1").count(), 1, `${path} needs one h1`);
    assert.equal((await page.locator("h1").textContent())?.trim(), heading, `${path} h1 differs`);
    assert.equal(await page.locator("main").count(), 1, `${path} needs one main`);
    assert.equal(await page.locator("img:not([alt])").count(), 0, `${path} image alt missing`);
    assert.ok((await page.locator("meta[name='description']").getAttribute("content"))?.length <= 155, `${path} description too long`);
    assert.ok((await page.locator("link[rel='canonical']").getAttribute("href"))?.startsWith(base), `${path} canonical differs`);
    assert.equal(await page.getByRole("link", { name: "Privacy", exact: true }).last().getAttribute("href"), "/privacy/");
    assert.equal(await page.getByRole("link", { name: "Terms", exact: true }).last().getAttribute("href"), "/terms/");
    const violations = (await new AxeBuilder({ page }).analyze()).violations.filter((item) => ["serious", "critical"].includes(item.impact ?? ""));
    assert.deepEqual(violations, [], `${path} has serious Axe violations`);
    assert.ok(await page.evaluate(() => document.documentElement.scrollWidth - document.documentElement.clientWidth <= 1), `${path} overflows`);
    if (viewport.width === 390) {
      const undersized = await page.locator("a:visible, button:visible").evaluateAll((nodes) => nodes.map((node) => {
        const box = node.getBoundingClientRect();
        return { text: node.textContent?.trim(), width: box.width, height: box.height };
      }).filter((item) => item.width < 44 || item.height < 44));
      assert.deepEqual(undersized, [], `${path} has undersized controls`);
    }
    report.routes[`${suffix}:${path}`] = { status: response?.status(), title, heading, seriousAxeViolations: violations.length };
  }

  assert.deepEqual(consoleErrors, [], `${suffix} route console errors`);
  assert.deepEqual(await context.cookies(), [], `${suffix} cookies found`);
  await context.close();
}

await routeAudit({ width: 1440, height: 900 }, "desktop");
await routeAudit({ width: 390, height: 844 }, "mobile");

{
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 } });
  const page = await context.newPage();
  await page.goto(`${base}/`, { waitUntil: "networkidle" });
  const action = await page.getByRole("link", { name: "Try it with sample data" }).boundingBox();
  const facts = await page.locator(".proof-strip").boundingBox();
  assert.ok(action && action.y + action.height <= 900, "sample action is below the desktop fold");
  assert.ok(facts && facts.y + facts.height <= 900, "three facts are below the desktop fold");
  await page.screenshot({ path: `${evidence}/home-desktop.png` });
  report.checks.push("desktop first screen action and three facts fit at 1440x900");

  await page.evaluate(() => localStorage.setItem("real:sentinel:marker", "keep"));
  await page.getByRole("link", { name: "Try it with sample data" }).click();
  await page.waitForURL(`${base}/?demo=1`);
  assert.equal(page.url(), `${base}/?demo=1`);
  assert.equal(await page.title(), "Demo — History Export Sentinel");
  assert.equal((await page.locator("h1").textContent())?.trim(), "Try a verified history export");
  assert.ok(await page.locator("#demo-banner").isVisible(), "demo banner is hidden");
  assert.match(await page.locator("#terminal-output").innerText(), /6 visits/);
  await page.getByRole("tab", { name: "Empty history" }).click();
  assert.deepEqual(await page.evaluate(() => Object.keys(localStorage).filter((key) => key.startsWith("demo:"))), ["demo:sentinel:state"]);
  await page.getByRole("button", { name: "Reset demo" }).click();
  assert.equal(await page.getByRole("tab", { name: "Verified export" }).getAttribute("aria-selected"), "true");
  assert.deepEqual(await page.evaluate(() => Object.keys(localStorage).filter((key) => key.startsWith("demo:"))), []);
  assert.equal(await page.evaluate(() => localStorage.getItem("real:sentinel:marker")), "keep");
  await page.getByRole("tab", { name: "Blocked profile" }).click();
  await page.getByRole("link", { name: "Start for real" }).click();
  await page.waitForURL(`${base}/#install`);
  assert.deepEqual(await page.evaluate(() => Object.keys(localStorage).filter((key) => key.startsWith("demo:"))), []);
  assert.equal(await page.evaluate(() => localStorage.getItem("real:sentinel:marker")), "keep");
  report.checks.push("query demo, reset, exit, and real-key isolation passed");

  await page.getByRole("link", { name: "Privacy", exact: true }).first().click();
  await page.waitForURL(`${base}/privacy/`);
  await page.waitForFunction(() => document.title === "Privacy — History Export Sentinel");
  assert.equal(await page.title(), "Privacy — History Export Sentinel");
  assert.ok(await page.locator("h1").evaluate((node) => node === document.activeElement), "privacy h1 was not focused");
  await page.goBack();
  await page.waitForFunction(() => document.title === "History Export Sentinel — verify browser history exports");
  assert.ok(await page.locator("h1").evaluate((node) => node === document.activeElement), "back did not focus the home h1");
  report.checks.push("History API title, focus, announcement, and Back behavior passed");
  await context.close();
}

{
  const context = await browser.newContext({ viewport: { width: 390, height: 844 } });
  const page = await context.newPage();
  await page.goto(`${base}/?demo=1`, { waitUntil: "networkidle" });
  await page.screenshot({ path: `${evidence}/demo-query-mobile.png` });
  assert.ok(await page.locator("#demo-panel").evaluate((node) => {
    const box = node.getBoundingClientRect();
    return box.top < innerHeight && box.bottom > 0;
  }), "sample output is absent from the first mobile screen");
  await page.evaluate(async () => { await navigator.serviceWorker.ready; });
  await page.reload({ waitUntil: "networkidle" });
  assert.equal(await page.evaluate(() => Boolean(navigator.serviceWorker.controller)), true, "service worker did not control demo");
  await context.setOffline(true);
  await page.reload({ waitUntil: "domcontentloaded" });
  assert.equal((await page.locator("h1").textContent())?.trim(), "Try a verified history export");
  assert.match(await page.locator("#terminal-output").innerText(), /6 visits/);
  await context.setOffline(false);
  report.checks.push("query demo reloads offline with sample content");
  await context.close();
}

{
  const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, reducedMotion: "reduce" });
  const page = await context.newPage();
  const response = await page.goto(`${base}/definitely-not-a-real-polish-3-route`, { waitUntil: "networkidle" });
  assert.equal(response?.status(), 404, "unknown route did not preserve HTTP 404");
  assert.equal(await page.title(), "Page not found — History Export Sentinel");
  assert.equal((await page.locator("h1").textContent())?.trim(), "This page was not found");
  assert.match((await response?.headerValue("content-security-policy")) ?? "", /frame-ancestors 'none'/);
  assert.equal(await response?.headerValue("referrer-policy"), "no-referrer");
  await page.screenshot({ path: `${evidence}/404-desktop.png` });
  report.checks.push("designed same-origin 404 keeps status and security headers");
  await context.close();
}

const foreign = report.requests.filter((url) => new URL(url).origin !== new URL(base).origin);
assert.deepEqual(foreign, [], `foreign requests observed: ${foreign.join(", ")}`);
report.checks.push("all observed product requests stayed same-origin");
writeFileSync(`${evidence}/live-audit.json`, `${JSON.stringify(report, null, 2)}\n`);
await browser.close();
console.log(`LIVE AUDIT PASS: ${report.checks.length} checks, ${report.requests.length} same-origin requests`);
