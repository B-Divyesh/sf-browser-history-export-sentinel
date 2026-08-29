# Polish round 3 — complete finding closure map

Product commits: `98efe56f995da51acd0e117105d90e7669fe2f73` and
`a6cbb2c6dbddee242d96604751494aef5e2b7279`. Deployed URL:
<https://browser-history-export-sentinel.sociobot.in>.

Evidence shorthand:

- **C** — all 16 `.factory/claims.json` commands passed independently in clean
  clone `/tmp/history-sentinel-polish3-final-TV1XdT/repo` at `a6cbb2c`.
- **S** — clean-clone `npm test`: 8 Rust tests, clippy, 13 CLI claim checks,
  and 25 Playwright checks passed; one non-mobile measurement was skipped.
- **L** — `.factory/evidence/live-polish-3/live-audit.json`; cold desktop and
  390 px checks, 0 serious/critical Axe findings, 26 same-origin requests.
- **V** — `.factory/evidence/live-polish-3/verify.json`; no console errors,
  one h1/main, title/lang/alt/button checks passed.
- **Shots** — `.factory/evidence/live-polish-3/home-desktop.png`,
  `demo-query-mobile.png`, and `404-desktop.png`.

## Review 1

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Kept the exact job-first h1 and audience sentence; tightened the desktop hero so the action and facts fit. | `first screen states…above the desktop fold`; **L**; home Shot |
| F-1-2 | Kept the isolated CLI demo and made the hero open the dedicated sample at `/?demo=1`; banner, Reset, and exit are immediate. | `@claim:demo-isolation`; `@claim:demo-sandbox`; demo Shot; live `/?demo=1` |
| F-1-3 | Retained the 16-entry claim registry and made every listed command independently runnable. | **C** |
| F-1-4 | Kept the short export/report copy and observable count, date, hash, and empty-result checks. | `@claim:end-to-end-export`; `@claim:actionable-errors` |
| F-1-5 | Numeric network-call copy remains removed; retained privacy copy is tested. | `@claim:cli-privacy`; `@claim:site-privacy` |
| F-1-6 | Browser support is named and exercised with Firefox and Chromium samples. | `@claim:end-to-end-export` |
| F-1-7 | Every demo exported-file digest is recomputed. | `@claim:end-to-end-export` |
| F-1-8 | Copy-before-read wording and source stability remain covered. | `@claim:copy-safety` |
| F-1-9 | Exported records are parsed rather than merely checking for a control. | `@claim:end-to-end-export` |
| F-1-10 | Verification runs after export and detects changed files. | `@claim:verification` |
| F-1-11 | Local processing is retained only with source/dependency and live request evidence. | `@claim:cli-privacy`; **L** |
| F-1-12 | No-account wording remains under the isolated CLI privacy test. | `@claim:cli-privacy` |
| F-1-13 | The vague “No cloud” fragment remains removed. | `.factory/copy-audit.md`; `@claim:cli-privacy` |
| F-1-14 | Exact no-telemetry wording remains tested. | `@claim:cli-privacy` |
| F-1-15 | Copy remains an instruction to keep and rerun the verification report. | `.factory/copy-audit.md`; `@claim:verification` |
| F-1-16 | Supported families and operating-system layouts remain explicit and tested. | `@claim:profile-discovery` |
| F-1-17 | Database and SQLite side-file copies use a private temporary folder. | `@claim:copy-safety` |
| F-1-18 | SQLite implementation jargon remains absent; source bytes and copied read paths are tested. | `@claim:copy-safety` |
| F-1-19 | Hash, record count, and date range are recomputed and checked. | `@claim:end-to-end-export`; `@claim:verification` |
| F-1-20 | A changed file returns code 13 and its exact path. | `@claim:verification` |
| F-1-21 | Both demos use shipped sample data and reserved `.example` domains. | `@claim:sample-domains`; demo Shot |
| F-1-22 | CLI offline/local behavior and isolated web behavior remain tested. | `@claim:cli-privacy`; `@claim:demo-sandbox` |
| F-1-23 | Generated reports are scanned for source and temporary absolute paths. | `@claim:end-to-end-export` |
| F-1-24 | Free/MIT/version facts remain registered. | `@claim:license-version-package` |
| F-1-25 | Install copy remains source-only and the package builds from a clean clone. | clean-clone `cargo package --locked`; **S** |
| F-1-26 | Future factory/release promises remain removed. | `.factory/copy-audit.md` |
| F-1-27 | Page-archive exclusion remains literal. | `@claim:history-fields-only` |
| F-1-28 | Only history records appear in demo exports. | `@claim:history-fields-only` |
| F-1-29 | Encryption-bypass jargon remains replaced by exact excluded data. | `@claim:history-fields-only` |
| F-1-30 | Every sample record is restricted to the six documented history fields. | `@claim:history-fields-only` |
| F-1-31 | Empty, malformed, and permission cases keep exact codes, advice, and no partial output. | `@claim:actionable-errors` |
| F-1-32 | README local-CLI wording remains covered by isolated execution and dependency inspection. | `@claim:cli-privacy` |
| F-1-33 | README uses short sentences; all stated export stages and outputs are checked. | `.factory/copy-audit.md`; `@claim:end-to-end-export` |
| F-1-34 | Source hashes remain stable and empty exports fail without output. | `@claim:copy-safety`; `@claim:actionable-errors` |
| F-1-35 | False binary-download instructions remain absent; installation is from source. | README; live `/#install` |
| F-1-36 | Version 0.1.0 is checked from package metadata. | `@claim:license-version-package` |
| F-1-37 | Internal publishing-process copy remains removed. | `.factory/copy-audit.md` |
| F-1-38 | All named browser/OS layout fixtures remain covered. | `@claim:profile-discovery` |
| F-1-39 | Automatic and explicit profile export remain documented and exercised. | Rust integration tests; `@claim:end-to-end-export` |
| F-1-40 | Existing output contents remain unchanged on refusal. | `@claim:no-overwrite` |
| F-1-41 | Each sample profile produces JSON, CSV, and a report. | `@claim:end-to-end-export` |
| F-1-42 | Intact verification passes and changed verification fails. | `@claim:json-mode`; `@claim:verification` |
| F-1-43 | The documented record example matches the enforced six-field shape. | `@claim:history-fields-only` |
| F-1-44 | Demo and verify JSON output are parsed as machine-readable JSON. | `@claim:json-mode` |
| F-1-45 | JSON control output does not change exported files. | `@claim:json-mode` |
| F-1-46 | Removed the remaining untested “never prompts” phrase; network/telemetry wording stays tested. | README diff; `@claim:cli-privacy`; `.factory/copy-audit.md` |
| F-1-47 | Password/encrypted-data exclusion remains plain and exact. | `@claim:history-fields-only` |
| F-1-48 | Allowed history fields are enforced for every sample row. | `@claim:history-fields-only` |
| F-1-49 | Removed the unregistered prose claim about `npm test`; the command itself passes all stages. | README diff; **S** |
| F-1-50 | Build output remains registered and checks every required route plus the service worker. | `@claim:site-build` |
| F-1-51 | Removed the redundant unregistered deployment sentence. | README diff; live root 200 check |
| F-1-52 | Every public route remains same-origin and cookie-free. | `@claim:site-privacy`; **L** |
| F-1-53 | CLI demo runs under isolated HOME/TMPDIR and writes only its selected output. | `@claim:demo-isolation`; `@claim:cli-privacy` |
| F-1-54 | MIT materials remain in the repository and packaged crate. | `@claim:license-version-package` |
| F-1-55 | Privacy copy states local processing and is backed by no-network inspection. | `@claim:cli-privacy` |
| F-1-56 | Runtime dependencies/source and execution remain free of network/telemetry paths. | `@claim:cli-privacy` |
| F-1-57 | Active WAL and SQLite side-file copying remain tested. | `@claim:copy-safety` |
| F-1-58 | Output permits only URL, title, visit time/count, browser, and profile. | `@claim:history-fields-only` |
| F-1-59 | Sample and snapshot temporary folders are removed. | `@claim:demo-isolation`; `@claim:copy-safety` |
| F-1-60 | Persistent CLI writes remain confined to the explicit output folder. | `@claim:demo-isolation` |
| F-1-61 | Reports omit full source and temporary paths. | `@claim:end-to-end-export` |
| F-1-62 | Home, demo, legal, and first-party error flows use no cookies or third parties. | `@claim:site-privacy`; **L** |
| F-1-63 | Service worker installation and the exact `/?demo=1` offline reload now verify populated cache entries and sample content. | `@claim:offline-reload`; **C**; **L** |
| F-1-64 | The unprovable “anonymous” statement remains removed. | Privacy page; `.factory/copy-audit.md` |
| F-1-65 | Free MIT status remains checked in metadata and package contents. | `@claim:license-version-package` |
| F-1-66 | No encryption/upload wording is supported by exact fields and no network path. | `@claim:history-fields-only`; `@claim:cli-privacy` |
| F-1-67 | Verification remains narrowly described and tamper behavior tested. | `@claim:verification`; live `/terms/` |
| F-1-68 | Current changelog remains inside the package. | `@claim:license-version-package` |
| F-1-69 | CLI README uses plain local export wording with end-to-end proof. | `@claim:end-to-end-export` |
| F-1-70 | Empty/format/permission/mismatch codes remain distinct and exercised. | `@claim:actionable-errors`; `@claim:verification` |
| F-1-71 | CLI privacy and copy safety remain independently tested. | `@claim:cli-privacy`; `@claim:copy-safety` |
| F-1-72 | CLI README still links exact commands and error codes. | `internal links, legal routes, robots, and sitemap resolve`; README link |
| F-1-73 | Service worker excludes deployment config, bypasses 304 bodies during precache, and reloads query demo offline. | `@claim:offline-reload`; live offline check in **L** |
| F-1-74 | Unknown URLs retain the product shell, HTTP 404, same-origin assets, and security headers. | `static configuration…`; 404 Shot; live unknown-route check in **L** |
| F-1-75 | Auto-discovery permission denial remains code 11 with advice. | `@claim:actionable-errors` as UID 65534 |
| F-1-76 | Every visible 390 px link/button remains at least 44×44 with no overflow. | `mobile layout has no overflow…`; **S**; demo Shot |
| F-1-77 | The used evidence image remains content-fingerprinted and immutable. | `@claim:site-build`; live request log in **L** |
| F-1-78 | Packaged crate still includes LICENSE and CHANGELOG. | `@claim:license-version-package`; clean-clone `cargo package` |
| F-1-79 | Home title stays specific and under 60 characters. | `every route has its own metadata…`; **V** |
| F-1-80 | Canonical, descriptions, OG/Twitter art, favicon, and touch icon remain on every route. | `every route has its own metadata…`; **L** |
| F-1-81 | History navigation updates title, focuses h1, and announces the route. | `internal routing updates history…`; **L** |
| F-1-82 | All routes retain the same header/footer, legal links, factory credit, and build ID. | `every route has its own metadata…`; live route matrix in **L** |
| F-1-83 | Sitemap retains home, demo, privacy, terms, and 404. | `internal links, legal routes, robots, and sitemap resolve` |
| F-1-84 | The required 13-word audience sentence remains in the first screen. | `first screen states…`; `.factory/copy-audit.md` |
| F-1-85 | README sentences remain at or below 22 words. | `.factory/copy-audit.md` |
| F-1-86 | Section heading remains “How Sentinel verifies an export.” | live `/`; home Shot |
| F-1-87 | Section heading remains “See successful and failed exports.” | live `/#sample`; home Shot |
| F-1-88 | Section heading remains “What the verification report records.” | live `/`; home Shot |
| F-1-89 | Install heading remains “Install Sentinel from source.” | live `/#install` |
| F-1-90 | Footer one-liner remains literal JSON/CSV output copy. | live route matrix in **L** |
| F-1-91 | Windows, macOS, and Linux remain named instead of “cross-platform.” | `@claim:profile-discovery`; `.factory/copy-audit.md` |
| F-1-92 | Copy-before-read wording remains in place instead of “copy-first.” | `@claim:copy-safety`; `.factory/copy-audit.md` |
| F-1-93 | Label remains “Copy, export, and verify.” | live `/`; home Shot |
| F-1-94 | Public try-out wording consistently uses sample data/sample-data demo. | `.factory/copy-audit.md`; demo Shot |
| F-1-95 | Public copy continues to use “exported file.” | `.factory/copy-audit.md` |
| F-1-96 | Error copy names the condition, code behavior, and next step in plain words. | `@claim:actionable-errors`; `.factory/copy-audit.md` |
| F-1-97 | Terminology table keeps one term for each product concept. | `.factory/copy-audit.md` |
| F-1-98 | Removed the time-sensitive binary-availability replacement; install now leads directly with source build instructions. | README and landing diffs; live `/#install` |

## Review 2

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-2-1 | Reset clears all `demo:` keys, restores verified state only in memory, and preserves a seeded real key. | `@claim:demo-sandbox`; **L** |
| F-2-2 | Removed “never prompts” from README. | README diff; `.factory/copy-audit.md` |
| F-2-3 | Removed the unregistered deployed-URL sentence from README. | README diff; live root 200 evidence remains in **V** |
| F-2-4 | Removed time-sensitive signed-binary availability copy from landing and README. | landing/README diff; `.factory/copy-audit.md` |
| F-2-5 | Replaced `report.json / format 1` with `Verification report`. | home Shot; live `/` |

## Review 3

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-3-1 | Reduced desktop hero size/spacing and widened its copy column; action and three facts fit at 1440×900. | `first screen states…above the desktop fold`; home Shot; **L** |
| F-3-2 | Playwright now builds before preview, so each browser claim runs from a clone without `dist/site`. | clean-clone `@claim:demo-sandbox` ran second and passed; **C** |
| F-3-3 | Reset now clears storage without immediately rewriting `verified`; test asserts zero demo keys and preserved real key. | `@claim:demo-sandbox`; **L** |
| F-3-4 | Removed the unregistered “never prompts” claim. | README diff; `.factory/copy-audit.md` |
| F-3-5 | Removed the unregistered prose claim describing `npm test`. | README diff; clean-clone **S** verifies the command itself |
| F-3-6 | Removed the unregistered deployment statement. | README diff; **V** separately records the live URL |
| F-3-7 | Removed signed-binary availability statements from both public surfaces. | landing/README diff; live `/#install` |
| F-3-8 | Rewrote the README URL as a repository-relative `/?demo=1` preview and listed README in `demo-sandbox.where`. | `@claim:demo-sandbox`; `.factory/claims.json`; **C** |
| F-3-9 | Replaced the unexplained format label with the standard term `Verification report`. | home Shot; live `/` |

## Result

Every finding from reviews 1–3 is closed. The post-deployment audit initially
found and then fixed query-demo offline caching; the final deployed audit and
claim test both pass the exact `/?demo=1` offline path. No severity is deferred.
