import "./style.css";

type DemoState = "verified" | "empty" | "locked";

const demoContent: Record<DemoState, { output: string; label: string; action: string }> = {
  verified: {
    output: `$ sentinel demo

SAMPLE     Firefox / Archive research.default / 3 visits
SAMPLE     Chromium / Research profile / 3 visits
COPY       2 history databases → private temporary folder
EXPORT     6 visits → JSON + CSV
HASH       every exported file → report.json

✓ DEMO EXPORT VERIFIED
  Profiles  2
  Records   6
  Domains   archive.example, docs.example, library.example
  Output    /tmp/sentinel-demo-export-…

No installed browser profile was read.`,
    label: "Exit 0 / verified",
    action: "The export matches both reports"
  },
  empty: {
    output: `$ sentinel export --profile ./sample/empty.default --output ./archive

COPY       places.sqlite → private temporary folder
READ       Firefox history
EXPORT     0 visits

∅ EXPORT STOPPED
  No exported file or report was kept.

sentinel: places.sqlite contains no history visits;
confirm the selected profile, then run sentinel scan`,
    label: "Exit 10 / no history",
    action: "Check the selected profile"
  },
  locked: {
    output: `$ sentinel export --profile ./sample/Default --output ./archive

COPY       History → private temporary folder
! STOPPED  operating system denied access

sentinel: cannot read/copy History: permission denied;
close the browser or fix OS permissions, then retry

No partial export folder was kept.`,
    label: "Exit 11 / access blocked",
    action: "Close the browser or fix permissions"
  }
};

const demoKey = "demo:sentinel:state";

function inDemoMode(): boolean {
  return document.body.hasAttribute("data-demo-page") || new URLSearchParams(location.search).get("demo") === "1";
}

function readDemoState(): DemoState {
  if (!inDemoMode()) return "verified";
  try {
    const value = localStorage.getItem(demoKey);
    return value === "empty" || value === "locked" ? value : "verified";
  } catch {
    return "verified";
  }
}

function writeDemoState(state: DemoState): void {
  if (!inDemoMode()) return;
  try { localStorage.setItem(demoKey, state); } catch { /* Demo still works without storage. */ }
}

function clearDemoStorage(): void {
  try {
    for (const key of Object.keys(localStorage)) if (key.startsWith("demo:")) localStorage.removeItem(key);
  } catch { /* Nothing else is required to leave demo mode. */ }
}

function applyQueryDemoMetadata(): void {
  const title = "Demo — History Export Sentinel";
  const description = "Try History Export Sentinel with isolated Firefox and Chromium sample data.";
  document.title = title;
  document.querySelector<HTMLMetaElement>("meta[name='description']")?.setAttribute("content", description);
  document.querySelector<HTMLLinkElement>("link[rel='canonical']")?.setAttribute("href", "https://browser-history-export-sentinel.sociobot.in/demo/");
  document.querySelector<HTMLMetaElement>("meta[property='og:title']")?.setAttribute("content", title);
  document.querySelector<HTMLMetaElement>("meta[property='og:description']")?.setAttribute("content", description);
  document.querySelector<HTMLMetaElement>("meta[name='twitter:title']")?.setAttribute("content", title);
  document.querySelector<HTMLMetaElement>("meta[name='twitter:description']")?.setAttribute("content", description);
}

function initializePage(): void {
  const demoMode = inDemoMode();
  const demoBanner = document.querySelector<HTMLElement>("#demo-banner");
  if (demoBanner) demoBanner.hidden = !demoMode;
  if (!demoMode) clearDemoStorage();
  if (demoMode) {
    writeDemoState(readDemoState());
    if (new URLSearchParams(location.search).get("demo") === "1") {
      applyQueryDemoMetadata();
      requestAnimationFrame(() => document.querySelector("#sample")?.scrollIntoView());
    }
  }

  const tabs = Array.from(document.querySelectorAll<HTMLButtonElement>("[role='tab'][data-state]"));
  const output = document.querySelector<HTMLElement>("#terminal-output code");
  const panel = document.querySelector<HTMLElement>("#demo-panel");
  const result = document.querySelector<HTMLElement>("#terminal-result");
  const resultLabel = document.querySelector<HTMLElement>("#result-label");
  const resultAction = document.querySelector<HTMLElement>("#result-action");

  const activateTab = (tab: HTMLButtonElement, focus = false, persist = true): void => {
    const state = tab.dataset.state as DemoState;
    tabs.forEach((candidate) => {
      const selected = candidate === tab;
      candidate.setAttribute("aria-selected", String(selected));
      candidate.tabIndex = selected ? 0 : -1;
    });
    if (panel && output && result && resultLabel && resultAction) {
      panel.setAttribute("aria-labelledby", tab.id);
      output.textContent = demoContent[state].output;
      resultLabel.textContent = demoContent[state].label;
      resultAction.textContent = demoContent[state].action;
      result.className = `terminal-result result-${state}`;
      panel.dataset.state = state;
    }
    if (persist) writeDemoState(state);
    if (focus) tab.focus();
  };

  tabs.forEach((tab, index) => {
    tab.addEventListener("click", () => activateTab(tab));
    tab.addEventListener("keydown", (event) => {
      let targetIndex = index;
      if (event.key === "ArrowRight" || event.key === "ArrowDown") targetIndex = (index + 1) % tabs.length;
      else if (event.key === "ArrowLeft" || event.key === "ArrowUp") targetIndex = (index - 1 + tabs.length) % tabs.length;
      else if (event.key === "Home") targetIndex = 0;
      else if (event.key === "End") targetIndex = tabs.length - 1;
      else return;
      event.preventDefault();
      activateTab(tabs[targetIndex], true);
    });
  });
  const initial = tabs.find((tab) => tab.dataset.state === readDemoState()) ?? tabs[0];
  if (initial) activateTab(initial);

  document.querySelectorAll<HTMLButtonElement>("[data-reset-demo]").forEach((button) => {
    button.addEventListener("click", () => {
      clearDemoStorage();
      const verified = tabs.find((tab) => tab.dataset.state === "verified");
      if (verified) activateTab(verified, true, false);
    });
  });
  document.querySelectorAll<HTMLElement>("[data-start-real]").forEach((link) => link.addEventListener("click", clearDemoStorage));

  const copyStatus = document.querySelector<HTMLElement>("#copy-status");
  document.querySelectorAll<HTMLButtonElement>("[data-copy]").forEach((button) => {
    button.addEventListener("click", async () => {
      const target = document.getElementById(button.dataset.copy ?? "");
      try {
        await navigator.clipboard.writeText(target?.textContent ?? "");
        button.childNodes[0].textContent = "Copied ✓";
        if (copyStatus) copyStatus.textContent = "Command copied to the clipboard.";
      } catch {
        if (copyStatus) copyStatus.textContent = "Clipboard access was blocked. Select and copy the command manually.";
      }
    });
  });

  const offlineBanner = document.querySelector<HTMLElement>("#offline-banner");
  if (offlineBanner) offlineBanner.hidden = navigator.onLine;
}

async function loadRoute(url: URL, addHistory: boolean): Promise<void> {
  if (addHistory) history.replaceState({ scrollY: window.scrollY }, "");
  const queryDemo = url.searchParams.get("demo") === "1";
  const response = await fetch(queryDemo ? "/demo/" : `${url.pathname}${url.search}`, { headers: { Accept: "text/html" } });
  const html = await response.text();
  const next = new DOMParser().parseFromString(html, "text/html");
  if (!next.querySelector("main h1")) throw new Error("Route has no page heading");
  document.title = next.title;
  for (const selector of ["meta[name='description']", "link[rel='canonical']", "meta[property^='og:']", "meta[name^='twitter:']"]) {
    document.head.querySelectorAll(selector).forEach((node) => node.remove());
    next.head.querySelectorAll(selector).forEach((node) => document.head.append(document.importNode(node, true)));
  }
  document.body.replaceWith(document.importNode(next.body, true));
  if (addHistory) history.pushState({ scrollY: 0 }, "", url);
  initializePage();
  const heading = document.querySelector<HTMLElement>("main h1");
  if (heading) {
    heading.tabIndex = -1;
    heading.focus();
    document.querySelector<HTMLElement>(".route-announcer")!.textContent = heading.textContent ?? document.title;
  }
  window.scrollTo(0, Number(history.state?.scrollY ?? 0));
}

document.addEventListener("click", (event) => {
  const link = (event.target as Element).closest<HTMLAnchorElement>("a[href]");
  if (!link || event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
  const url = new URL(link.href, location.href);
  if (url.origin !== location.origin || link.hasAttribute("download") || url.hash || url.pathname === location.pathname && url.search === location.search) return;
  event.preventDefault();
  loadRoute(url, true).catch(() => { location.href = url.href; });
});

window.addEventListener("popstate", () => {
  loadRoute(new URL(location.href), false).catch(() => location.reload());
});
window.addEventListener("online", () => { const banner = document.querySelector<HTMLElement>("#offline-banner"); if (banner) banner.hidden = true; });
window.addEventListener("offline", () => { const banner = document.querySelector<HTMLElement>("#offline-banner"); if (banner) banner.hidden = false; });

if (new URLSearchParams(location.search).get("demo") === "1" && !document.body.hasAttribute("data-demo-page")) {
  loadRoute(new URL(location.href), false).catch(initializePage);
} else {
  initializePage();
}

if ("serviceWorker" in navigator) window.addEventListener("load", () => navigator.serviceWorker.register("/sw.js"));
