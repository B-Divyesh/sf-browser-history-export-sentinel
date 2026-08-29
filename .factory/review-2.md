# Adversarial first-read review 2 — History Export Sentinel

**Verdict: FAIL**

Reviewed 2026-08-29 UTC against commit `84702d6228c4e7c0cfe86bc5f4f1fe0cc56ffead` and the live site at <https://browser-history-export-sentinel.sociobot.in>. This was a fresh Chromium review at 390×844 and 1440×900, followed by a clean-clone claim run.

## Cold first screen

Before scrolling, both viewports showed the same useful answer:

- **What it does:** exports and verifies Firefox or Chromium browser history.
- **Who it is for:** people archiving that history before changing a computer or account.
- **First action:** click **Try it with sample data** to see a completed export and verification report.

The mobile page had no horizontal overflow or undersized visible control. The desktop page likewise made the job, audience, and first action visible without scrolling. This closes F-1-1.

## Findings

### F-2-1 — BLOCKING: Reset demo does not clear the demo key it claims to clear

**Location and quote:** `.factory/claims.json`, `demo-sandbox`: “Reset demo clears it”; `.factory/demo.md`: “Reset demo clears every `demo:` key and restores the verified state.”

From a fresh live context, selecting **Empty history** produced only `demo:sentinel:state=empty`. Clicking **Reset demo** left `demo:sentinel:state=verified` in localStorage. The implementation calls `clearDemoStorage()` and then calls `activateTab(verified)`, which calls `writeDemoState("verified")`.

This does not touch real storage, but it contradicts the literal sandbox claim and the claim test only checks the visual tab. **Fix:** either make Reset leave no `demo:` key and make the default state in memory, or change the claim/documentation to say “Reset returns the demo to the verified sample state” and test that exact state. The tagged test must assert the documented storage outcome.

### F-1-46 / F-2-2 — BLOCKING: README retains an unlisted “never prompts” claim

**Location and quote:** README, “The CLI never prompts and has no network or telemetry code.”

`cli-privacy` covers the no-account/network/telemetry statement, but no claims entry says that the CLI never prompts and no tagged test runs each supported command with closed stdin to prove it. This is the remaining unproved portion of earlier F-1-46, not merely a wording change.

**Fix:** remove “never prompts,” or extend `cli-privacy` with that exact claim and a clean-temp test that closes stdin and verifies `scan`, `export`, `verify`, and `demo` complete or fail without requesting input.

### F-1-51 / F-2-3 — BLOCKING: README deployment statement has no claims entry

**Location and quote:** README, “The static documentation is deployed at <https://browser-history-export-sentinel.sociobot.in>.”

This is a visitor-reliant deployment claim, but `.factory/claims.json` has no `deployment` entry or tagged test. The prior closure map says it was “cold-checked”; that is not a repeatable claim test in a clean clone.

**Fix:** remove the self-evident deployment sentence, or add a `deployment` claim with a controlled HTTP test that checks the canonical URL serves the expected product title and build identifier.

### F-2-4 — BLOCKING: “Signed binaries are not available” is unlisted on both public surfaces

**Locations and quotes:** landing install section and README: “Signed binaries are not available yet.”

The statement is honest in this review, but it is still a time-sensitive claim with no registry entry or test. It appears in two places and is not covered by `license-version-package`, which only checks license/version/package contents.

**Fix:** remove the sentence and lead directly with source installation, or add a claim that checks the intended release source and states exactly what availability condition is tested.

### F-2-5 — minor: an unexplained technical label breaks the documented vocabulary

**Location and quote:** landing report-section eyebrow, “report.json / format 1”.

The product’s terminology says this object is a “verification report.” “format 1” is unexplained and its purpose is not clear to a first-time visitor; it also reintroduces a different label next to the otherwise clear heading “What the verification report records.”

**Fix:** replace it with `Verification report` (or `Verification report format 1` only if the version has a documented user consequence).

## Copy audit

Counts treat URLs, commands, hyphenated terms, and version strings as one word. There are no sentences over 22 words. The tables include visible text and dynamic demo/error sentences; labels, headings, code commands, and table values are assessed after them.

### Landing page

| # | Words | Sentence |
| ---: | ---: | --- |
| 1 | 2 | Offline mode. |
| 2 | 7 | The guide and sample-data demo remain available. |
| 3 | 7 | Demo — sample data, nothing is saved. |
| 4 | 13 | For people archiving Firefox or Chromium history before changing a computer or account. |
| 5 | 8 | See a finished export and its verification report. |
| 6 | 5 | History stays on your computer. |
| 7 | 3 | Copy the database. |
| 8 | 3 | Export its records. |
| 9 | 4 | Verify the exported files. |
| 10 | 6 | Your history stays on your computer. |
| 11 | 7 | The CLI has no account or telemetry. |
| 12 | 7 | Keep the verification report with your export. |
| 13 | 8 | Run the check again after moving the folder. |
| 14 | 12 | Find Firefox and Chromium-family profiles in standard Windows, macOS, and Linux locations. |
| 15 | 14 | Copy the history database and its SQLite side files to a private temporary folder. |
| 16 | 14 | Check each file hash, the record count, and the date range against the report. |
| 17 | 6 | The demo uses reserved sample domains. |
| 18 | 7 | It never reads an installed browser profile. |
| 19 | 8 | The report names the profile and database file. |
| 20 | 9 | It does not store the source profile’s full path. |
| 21 | 6 | Signed binaries are not available yet. |
| 22 | 9 | Build the single CLI with a current Rust toolchain. |
| 23 | 6 | It does not archive web pages. |
| 24 | 5 | It exports history records only. |
| 25 | 7 | It does not read passwords or cookies. |
| 26 | 7 | Encrypted browser data remains out of scope. |
| 27 | 5 | It does not hide errors. |
| 28 | 16 | Permission problems, unsupported database formats, and changing files stop with a specific code and next step. |
| 29 | 9 | Exports Firefox and Chromium history to JSON or CSV. |
| 30 | 7 | Built by Param Factory · build 0.1.0-polish.1. |
| 31 | 6 | No installed browser profile was read. |
| 32 | 5 | The export matches both reports. |
| 33 | 7 | No exported file or report was kept. |
| 34 | 14 | sentinel: places.sqlite contains no history visits; confirm the selected profile, then run sentinel scan. |
| 35 | 15 | sentinel: cannot read/copy History: permission denied; close the browser or fix OS permissions, then retry. |
| 36 | 6 | No partial export folder was kept. |

Headings name their sections and the main action is result-naming. `report.json / format 1` is the exception in F-2-5. `CLI`, `SQLite`, `SHA-256`, and profile names are appropriate technical terms for this explicitly command-line product; their surrounding sentences say what to do with them.

### README

| # | Words | Sentence |
| ---: | ---: | --- |
| 1 | 16 | History Export Sentinel is a local CLI for people moving or archiving Firefox and Chromium history. |
| 2 | 8 | Sentinel copies each history database before reading it. |
| 3 | 5 | It exports JSON or CSV. |
| 4 | 12 | It records counts, the date range, and file hashes for later checks. |
| 5 | 12 | The CLI does not upload history or modify an installed browser profile. |
| 6 | 14 | An empty history database stops with an error instead of creating an empty export. |
| 7 | 9 | The sample command needs no browser profile or account. |
| 8 | 10 | It creates two sample profiles in a private temporary folder. |
| 9 | 11 | It exports and verifies six visits, then prints the output folder. |
| 10 | 7 | The same sample is available at https://browser-history-export-sentinel.sociobot.in/demo/. |
| 11 | 6 | See `.factory/demo.md` for its isolation rules. |
| 12 | 7 | Signed release binaries are not available yet. |
| 13 | 7 | Install with a current stable Rust toolchain: |
| 14 | 10 | The project version is 0.1.0 and the license is MIT. |
| 15 | 3 | Find supported profiles: |
| 16 | 8 | Export all detected profiles or choose one profile. |
| 17 | 12 | The output folder must be new, so existing files cannot be overwritten. |
| 18 | 8 | Check the exported files against each verification report: |
| 19 | 5 | JSON records use this format: |
| 20 | 7 | `--json` makes command results readable by scripts. |
| 21 | 7 | It does not change the exported files. |
| 22 | 11 | The CLI never prompts and has no network or telemetry code. |
| 23 | 10 | Sentinel reads history URLs, titles, visit times, and visit counts. |
| 24 | 12 | It does not read passwords, cookies, page bodies, or encrypted browser data. |
| 25 | 9 | Install stable Rust, Node.js 20 or newer, and npm. |
| 26 | 2 | Then run: |
| 27 | 13 | `npm test` runs the Rust tests, claim tests, site build, and browser tests. |
| 28 | 9 | `npm run build` writes the static site to `dist/site/`. |
| 29 | 7 | The static documentation is deployed at https://browser-history-export-sentinel.sociobot.in. |
| 30 | 11 | The site has no analytics, cookies, third-party scripts, or remote fonts. |
| 31 | 10 | The service worker caches its public pages for offline reading. |
| 32 | 6 | Read the privacy policy and terms. |

No banned marketing adjective, mood slogan, or overlong sentence was found. F-1-46/F-2-2, F-1-51/F-2-3, and F-2-4 are the claim-registry failures in this table.

## Demo, privacy, claims, and history checks

- **One-click demo:** passed. The hero action opened `/demo/`; its first screen already displayed two named sample profiles, six visits, JSON/CSV, report hashes, and a verified result. The persistent banner, Reset, and Start for real controls were present. `?demo=1` also entered a visible demo immediately.
- **Sandbox behavior:** passed except F-2-1. Demo mode read/wrote only the `demo:sentinel:state` namespace; selecting a sample state never touched a non-demo key. Start for real removed the demo key and returned to `/#install`. The CLI `sentinel demo` claim uses a fresh HOME/TMPDIR and shipped sample data.
- **Requests/privacy/offline:** passed. A fresh mobile context over home, demo, legal pages, and a first-party 404 observed only `https://browser-history-export-sentinel.sociobot.in`, set no cookies, and reloaded `/demo/` offline after service-worker control. The 404 document’s expected HTTP 404 appeared in the browser resource log; no product script error or third-party request occurred.
- **Claims:** all 16 listed commands passed independently in a fresh clone at `/tmp/history-sentinel-review-YOYrMv`. `npm test` and `npm run check` passed locally. Passing commands do not cure F-2-1 because its test omits the promised clear-storage assertion.
- **Earlier review/history:** read `review-1.md`, `polish-1.md`, `verification.md`, prior handoff, and demo/copy evidence. Live/code checks confirm F-1-1–45, F-1-47–50, F-1-52–98 are fixed: real CLI/browser demos, claims registry, source-safe export, profile permission error, service worker, product 404, mobile targets, packaging files, metadata, routing/focus, and terminology all verify. F-1-46 and F-1-51 remain only partially closed as recorded above.
- **Structure/accessibility:** passed apart from F-2-5. Each normal route has a route-specific title, description, canonical, OG/Twitter metadata, favicon/touch icon, one h1, and main. The unknown route has the product 404 shell and HTTP 404. Forward/back focused and announced the destination h1. The shared header/footer, sitemap/robots, CSP, internal/external link crawl, keyboard tabs, reduced motion, and 390px target/overflow checks passed. The visual evidence-desk system is distinct and matches `.factory/design.md`, not a generic SaaS template.
- **Missed leverage:** none. The brief calls for a deterministic, local, sensitive-data CLI; AI, sync, or remote import would add risk without advancing this job. JSON/CSV export and verification are already the valuable additional steps.

## What would make this perfect

Make Reset’s storage behavior truthful and test it. Remove or register and test every remaining public assertion identified above. Rename the unexplained report label. Then rerun this complete cold/live/clean-clone review; PASS requires zero findings and no untested claim.
