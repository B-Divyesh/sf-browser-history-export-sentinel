# History Export Sentinel — polish round 1 handoff

## Outcome

All findings `F-1-1` through `F-1-98` are resolved. The per-finding change and
evidence map is in `.factory/polish-1.md`. Product code commit
`2a547c05f3ac2c8283f3523152674ed34ff569f9` is pushed to `origin/main` and is
deployed at <https://browser-history-export-sentinel.sociobot.in>.

The site keeps its neo-brutalist evidence-desk identity. The CLI remains a Rust
single binary and the deployment remains a static Vite site.

## What changed

- Added `sentinel demo` with six bundled visits across isolated Firefox and
  Chromium sample profiles. It exports JSON and CSV, verifies both reports,
  deletes the temporary input, and prints the retained output path.
- Added `/demo/` and `?demo=1` entry points with a persistent sample banner,
  namespaced `demo:sentinel:state`, reset, and exit controls.
- Added `.factory/claims.json` with 16 retained claims and one tagged test for
  each claim.
- Rewrote the first screen and all reviewed jargon, headings, README wording,
  install availability, and footer copy.
- Added route-specific titles, canonical/OG/Twitter metadata, a 1200×630 image,
  an Apple touch icon, History API navigation, h1 focus/announcement, and a
  first-party HTTP 404.
- Fixed unreadable discovery roots to return code 11 with the path and repair
  advice. Added active-WAL/source-stability/private-workspace proof.
- Fixed service-worker precaching, mobile 44px targets, image cache versioning,
  package legal/change files, sitemap routes, common shells, and security
  headers.

## Clean-clone evidence

Final fresh clone: `/tmp/sentinel-final-2a547c0-rIDEeh`, cloned from GitHub at
exact deployed commit `2a547c05f3ac2c8283f3523152674ed34ff569f9`.

- `npm ci`: pass, 24 packages, 0 vulnerabilities.
- `npm run check`: pass.
- `npm test`: pass — 4 Rust unit tests, 4 Rust integration tests, clippy, build,
  13 CLI claim checks, and 21 Playwright checks passed across desktop and 390px.
  One desktop execution of the mobile-only size test was intentionally skipped.
- Every command in `.factory/claims.json`: **16/16 pass when invoked separately**
  from the clean clone.
- `npm audit --audit-level=high`: pass, 0 vulnerabilities.
- `cargo fmt --all -- --check`: pass.
- `cargo build --release --locked`: pass; Linux binary 2.9 MB.
- `cargo package --manifest-path cli/Cargo.toml --locked`: pass and verified;
  10 files, 77.2 KiB unpacked / 19.5 KiB compressed. LICENSE, changelog, and
  sample data are included.
- Source install from pushed Git commit `2a547c05`: pass. Installed
  `sentinel 0.1.0`; its JSON demo exported 6 records across 2 profiles and
  verified 4 files across 2 reports.

Build output is 6.33 KB JavaScript (2.63 KB gzip) and 15.23 KB CSS (4.20 KB
gzip), with no font payload. The initial live transfer measured 92 KiB.

## Deployment and cold-live evidence

Deployment used `/opt/fleet/lib/deploy-static.sh browser-history-export-sentinel
dist/site`. Final Azure deployment ID:
`4d5370ee-87aa-42e6-a734-e4000117d246`.

- Factory `verify-url.sh`: pass; HTTP 200, title/lang/main/one h1, no missing
  alt text, no unlabeled buttons, and no normal-route console error.
- Cold Chromium at 390×844: `/`, `/demo/`, `/?demo=1`, `/privacy/`, `/terms/`,
  and an unknown route each have one h1, canonical metadata, zero serious or
  critical Axe violations, zero overflow, and zero visible controls below
  44×44.
- Unknown route: HTTP 404 with the designed product shell, CSP, no-referrer,
  and nosniff headers. It makes no third-party request.
- Demo: only `demo:sentinel:state` is written; Reset restores verified state;
  Start for real removes the namespace.
- Route change: Privacy h1 receives focus and is announced; Back focuses the
  home h1.
- Privacy: all observed requests were same-origin and no cookies were set.
- Offline: an active controller used cache `sentinel-c8c9ac1b47f0`; the
  service worker excludes deployment config. `/demo/` reloaded offline and its
  Empty history tab remained operable.
- Link crawl: every home, demo, legal, 404, hash, and GitHub link returned 200.
- Fingerprinted hero response uses one-year immutable caching. The old
  unfingerprinted URL now uses 30-second revalidation.
- Lighthouse 13.4.1 mobile: Performance 100, Accessibility 100, Best Practices
  100, SEO 100; FCP 0.9 s, LCP 1.3 s, CLS 0, TBT 40 ms.

Evidence files:

- `.factory/evidence/home-desktop.png`
- `.factory/evidence/demo-mobile.png`
- `.factory/evidence/404-desktop.png`
- `.factory/evidence/live/screenshot-desktop.png`
- `.factory/evidence/live/screenshot-mobile.png`
- `.factory/evidence/live/verify.json`
- `.factory/evidence/live/lighthouse.json`

## Run and verify

```sh
npm ci
npm test
npm run check
npm run build
cargo run --manifest-path cli/Cargo.toml -- demo
cargo package --manifest-path cli/Cargo.toml --locked
```

## Known gaps and next steps

No review finding or product defect remains open. Signed release binaries are
not published; the UI and README state this plainly, and publishing is reserved
for the factory registry/release process.
