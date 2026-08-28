import "./style.css";

type DemoState = "verified" | "empty" | "locked";

const demoContent: Record<DemoState, { output: string; label: string; action: string }> = {
  verified: {
    output: `$ sentinel export --profile ./fixture/Default --output ./archive

COPY       History + History-wal → private snapshot
READ       chromium schema v131
NORMALIZE  12,841 visits → JSON + CSV
HASH       history.json  7a920e…e914
HASH       history.csv   f839cb…8a20

✓ EXPORT COMPLETE
  Records   12,841
  Range     2017-04-08 → 2026-08-27
  Report    ./archive/chromium-default/report.json

$ sentinel verify ./archive
✓ VERIFIED — hashes, count, and date bounds match`,
    label: "Exit 0 / verified",
    action: "Safe to copy the archive"
  },
  empty: {
    output: `$ sentinel export --profile ./fixture/empty.default --output ./archive

COPY       places.sqlite → private snapshot
READ       firefox schema
NORMALIZE  0 visits

∅ EXPORT STOPPED
  No artifact or report was kept.

sentinel: places.sqlite contains no history visits;
no export was written

Suggested check: confirm this is the active profile, then run
$ sentinel scan`,
    label: "Exit 10 / no history",
    action: "Check the selected profile"
  },
  locked: {
    output: `$ sentinel export --profile ./Default --output ./archive

COPY       History → private snapshot
! STOPPED  operating system denied access

sentinel: cannot read/copy History: permission denied;
close the browser or fix OS permissions, then retry

No live database was opened.
No partial archive was kept.`,
    label: "Exit 11 / access blocked",
    action: "Close the browser, then retry"
  }
};

const tabs = Array.from(document.querySelectorAll<HTMLButtonElement>("[role='tab'][data-state]"));
const output = document.querySelector<HTMLElement>("#terminal-output code");
const panel = document.querySelector<HTMLElement>("#demo-panel");
const result = document.querySelector<HTMLElement>("#terminal-result");
const resultLabel = document.querySelector<HTMLElement>("#result-label");
const resultAction = document.querySelector<HTMLElement>("#result-action");

function activateTab(tab: HTMLButtonElement, focus = false): void {
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
  if (focus) tab.focus();
}

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

const copyStatus = document.querySelector<HTMLElement>("#copy-status");
document.querySelectorAll<HTMLButtonElement>("[data-copy]").forEach((button) => {
  button.addEventListener("click", async () => {
    const target = document.getElementById(button.dataset.copy ?? "");
    const text = target?.textContent ?? "";
    try {
      await navigator.clipboard.writeText(text);
      button.textContent = "Copied ✓";
      if (copyStatus) copyStatus.textContent = "Command copied to the clipboard.";
      window.setTimeout(() => { button.textContent = "Copy"; }, 1800);
    } catch {
      if (copyStatus) copyStatus.textContent = "Clipboard access was blocked. Select and copy the command manually.";
    }
  });
});

const offlineBanner = document.querySelector<HTMLElement>("#offline-banner");
if (offlineBanner) {
  offlineBanner.hidden = navigator.onLine;
  window.addEventListener("online", () => { offlineBanner.hidden = true; });
  window.addEventListener("offline", () => { offlineBanner.hidden = false; });
}

if ("serviceWorker" in navigator) {
  window.addEventListener("load", () => {
    navigator.serviceWorker.register("/sw.js").catch(() => {
      // Offline caching is an enhancement; the product and docs remain usable.
    });
  });
}
