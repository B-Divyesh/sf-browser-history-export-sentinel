# History Export Sentinel — build handoff

Work order: `browser-history-export-sentinel-build-1`

Version: `0.1.0`

Completed: 2026-08-28

## What shipped

- A Rust/Clap `sentinel` single binary with:
  - `scan` discovery for Firefox, Chrome, Chromium, Edge, Brave, and Vivaldi
    profile layouts on Linux, macOS, and Windows;
  - `export` from auto-detected profiles or an explicit profile/database;
  - a copy-first SQLite snapshot including present WAL/SHM sidecars, with
    whole-set before/after metadata checks and three bounded retries if the
    browser changes the set during the copy;
  - normalized JSON and/or RFC 4180 CSV records;
  - a `report.json` containing count, earliest/latest visit, copied source DB
    hash, and exported artifact SHA-256 hashes;
  - `verify` to recompute hashes, parse artifacts, and compare record counts and
    date bounds;
  - `--json` command output, useful help, no prompts, and documented exit codes.
- Safety behavior: the live database is never opened by SQLite; empty history,
  unreadable/changing profiles, unsupported schemas, and tampered exports stop
  with distinct non-zero codes. Failed output directories are removed instead
  of leaving a plausible partial archive.
- A Vite/vanilla TypeScript documentation site with an original neo-brutalist
  evidence-desk visual system, responsive 390px layout, keyboard-operable
  recorded states (verified/empty/locked), offline banner and service worker,
  install/usage guidance, privacy and terms pages, sitemap/robots, and no
  analytics, cookies, external fonts, or third-party runtime code.
- Product documentation: expanded README, MIT license, changelog, brief, visual
  thesis/provenance, and this handoff.

## Build and verification

From a clean clone with stable Rust, Node 20+, and npm:

```sh
npm ci
npm test
npm run build:site
```

The factory build command is exactly `npm run build:site`; deploy
`dist/site/`, whose root contains `index.html`.

Verified locally:

- `npm test` — pass: 7 Rust tests and 8 Playwright tests across desktop
  Chromium and a 390×844 viewport. The browser tests include axe serious/
  critical checks, semantic structure, console errors, keyboard tab behavior,
  recorded empty/locked states, offline messaging, legal routes, and horizontal
  overflow.
- `cargo clippy --workspace --all-targets -- -D warnings` — pass.
- `cargo build --release --locked` — pass; Linux binary is 2.8 MB.
- `cargo package --manifest-path cli/Cargo.toml --locked` — pass; package is
  16.8 KiB compressed. The factory may publish it later; no publishing was done.
- `npm audit --audit-level=high` — 0 vulnerabilities.
- `/opt/fleet/lib/verify-url.sh http://127.0.0.1:4173 ...` — HTTP 200, 542 ms
  network-idle load in the local runner, no console errors, one h1, `lang=en`,
  main landmark present, no missing image alt, no unlabeled buttons.
- Lighthouse 13.4.1 mobile: Performance **98**, Accessibility **100**, Best
  Practices **100**, SEO **100**; FCP 1.1 s, LCP 1.7 s, CLS 0, TBT 150 ms. INP
  was not available from a no-interaction lab navigation; TBT remains below the
  200 ms interaction budget proxy.
- Production payloads: initial JS 3.9 KB, CSS 13.0 KB, hero WebP 113.8 KB; no
  font payload. All are below the 200/50/300 KB budgets.

## Asset provenance

The 1200×800 `site/public/evidence-desk.webp` was generated specifically for
this product with the required factory image deployment, inspected visually,
and optimized from PNG to 112 KB WebP. The exact prompt, deployment, requested
size, and quality are recorded in
`.factory/assets/evidence-desk-generation.json`; the design rationale and prompt
are also in `.factory/design.md`. No stock or third-party visual assets ship.

## Known gaps / next steps

- Release binaries are not signed or attached to GitHub Releases in this work
  order. The crate is package-verified and the source install works; CI should
  build/sign Windows, macOS, and Linux binaries before announcing downloads.
- Real browser schema behavior is covered with representative Firefox and
  Chromium SQLite fixtures; profile discovery paths are unit-tested for all
  three operating systems, but this container only executed the binary on
  Linux. Run the same fixture suite natively in Windows/macOS release CI.
- Sentinel detects source changes during the copy and asks the user to close the
  browser after three unstable attempts. It intentionally does not suspend a
  browser process, elevate OS permissions, bypass encryption, archive pages, or
  upload history.
