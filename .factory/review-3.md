# Adversarial first-read review 3 — History Export Sentinel

**Verdict: FAIL**

Reviewed 2026-08-29 UTC at repository commit
`2368d614c5ba678111473a01fa39390c50f93174` and against the live site at
<https://browser-history-export-sentinel.sociobot.in>. The review used fresh
Chromium contexts at 390×844 and 1440×900 and a clean clone at
`/tmp/history-sentinel-review3-clone-K6fa18`.

## Cold first screen

At 390×844, before scrolling:

- **What it does:** exports and verifies Firefox or Chromium browser history.
- **Who it is for:** people archiving that history before changing a computer
  or account.
- **What to click first:** **Try it with sample data**.

At 1440×900, the job and audience are clear, but the required first action is
not. The sample-data action starts at y=901.05 and is entirely below the
900-pixel viewport. The only visible action is the header's **Install ↓** link.
That is F-3-1 below.

## Findings

### F-3-1 — BLOCKING: the desktop first screen hides the required first action

**Location and quote:** live `/`, 1440×900. The visible text includes “Export
and verify browser history” and “For people archiving Firefox or Chromium
history before changing a computer or account.” The only visible action is
“Install ↓”; “Try it with sample data” begins 1.05 CSS pixels below the fold.

A first-time desktop visitor cannot answer what to click first without
scrolling and may reasonably install before trying the product. **Fix:** reduce
the desktop hero type/vertical spacing or move the sample action directly under
the audience sentence so its complete 52.8-pixel control and explanatory note
fit at 1440×900. Add a viewport test that asserts the action's bottom is no
greater than `window.innerHeight`.

### F-3-2 — BLOCKING: a listed claim test fails from the clean clone

**Location and quote:** `.factory/claims.json`, `demo-sandbox`, command
`npm run test:site -- --project=chromium --grep @claim:demo-sandbox`.

Run as listed after `npm ci` in the clean clone, it timed out after 60 seconds:
`Error: Timed out waiting 60000ms from config.webServer.` The clone has no
tracked `dist/site`, while `playwright.config.ts` starts only
`npm run preview`; it does not build first. The same test passes later only
after the separately listed `site-build` claim creates `dist/site`.

Every claim command must work from a clean state and must not depend on claim
order. **Fix:** make each browser claim command build first, or make the
Playwright web server run the build before preview. Rerun all listed commands
from a new clone in registry order and independently.

### F-2-1 / F-3-3 — BLOCKING: Reset still does not clear the key it claims to clear

**Locations and quotes:** `.factory/claims.json`: “Reset demo clears it”;
`.factory/demo.md`: “Reset demo clears every `demo:` key”; privacy page: “Reset
demo removes it.”

Live reproduction: select **Empty history**, then **Reset demo**. Storage
changes from `demo:sentinel:state=empty` to
`demo:sentinel:state=verified`; the key remains. `site/main.ts` clears storage
and immediately calls `activateTab(verified)`, which writes the key again. The
tagged test checks the selected tab but not the promised storage result.

**Fix:** either keep the default verified state only in memory after Reset, or
rewrite all three claims to “Reset demo returns to the verified sample state”
and assert the exact post-reset storage value.

### F-1-46 / F-2-2 / F-3-4 — BLOCKING: “never prompts” remains unlisted

**Location and quote:** README: “The CLI never prompts and has no network or
telemetry code.”

`cli-privacy` lists and tests network and telemetry behavior, but neither the
claim nor its test covers prompts. **Fix:** remove “never prompts,” or add it to
the registry and run `scan`, `export`, `verify`, and `demo` with closed stdin.

### F-1-49 / F-3-5 — BLOCKING: the README's `npm test` claim remains unlisted

**Location and quote:** README: “`npm test` runs the Rust tests, claim tests,
site build, and browser tests.”

The statement is true in this run, but no `.factory/claims.json` entry lists
it. The `site-build` claim covers only `npm run build`. Earlier F-1-49 was
therefore documented as fixed without satisfying the registry rule. **Fix:**
add a non-circular smoke test that observes each `npm test` stage, or remove the
sentence.

### F-1-51 / F-2-3 / F-3-6 — BLOCKING: the deployment statement remains unlisted

**Location and quote:** README: “The static documentation is deployed at
https://browser-history-export-sentinel.sociobot.in.”

No claim entry or repeatable test covers the deployed URL. A cold check in a
handoff is not a registered test. **Fix:** remove the redundant sentence, or
add a deployment claim that checks the canonical URL, product title, and build
identifier.

### F-2-4 / F-3-7 — BLOCKING: signed-binary availability remains unlisted

**Locations and quotes:** landing: “Signed binaries are not available yet.”
README: “Signed release binaries are not available yet.”

This time-sensitive availability claim has no registry entry. **Fix:** remove
both sentences and lead with source installation, or test the named release
source and list both locations.

### F-3-8 — BLOCKING: the public demo URL is an unlisted availability claim

**Location and quote:** README: “The same sample is available at
https://browser-history-export-sentinel.sociobot.in/demo/.”

The URL worked in this review, but `demo-sandbox.where` does not list the
README, and its local Playwright test does not prove that production URL is
available. **Fix:** include the README location and test the canonical deployed
demo, or rewrite the README as a repository-relative local preview instruction.

### F-2-5 / F-3-9 — minor: the unexplained report label remains

**Location and quote:** landing eyebrow, “report.json / format 1”.

“format 1” has no explained consequence and conflicts with the otherwise
consistent term “verification report.” **Fix:** use `Verification report`, or
document what format 1 means and when a user needs it.

## Copy audit

Counts treat URLs, commands, hyphenated terms, and versions as one word. No
sentence exceeds 22 words and no banned marketing adjective appears. Claim
flags are F-3-4 through F-3-8; the terminology flag is F-3-9.

### Landing page — every sentence

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
| 37 | 5 | Command copied to the clipboard. |
| 38 | 4 | Clipboard access was blocked. |
| 39 | 6 | Select and copy the command manually. |

Headings and controls were checked separately. The h1 and h2 headings name
their sections. `Try it with sample data`, Install, the accessible Copy action,
Reset, and the demo state controls name their results or states. The exception
is the `report.json / format 1` label in F-3-9.

### Root README — every sentence

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
| 33 | 1 | MIT. |
| 34 | 2 | See LICENSE. |

README headings are literal section names. It has no button copy. `CLI`,
`SQLite`, JSON, CSV, and SHA-256 are appropriate terms for this command-line
product; each is used consistently.

## Demo, sandbox, privacy, and CLI behavior

- The mobile hero opens `/demo/` in one click. Its first screen already shows
  two named profiles, six visits, JSON/CSV output, report hashes, and a verified
  result. The persistent banner, Reset, and Start for real are present.
- A seeded `real:sentinel:marker=keep` survived entering the demo, selecting
  every state, Reset, and Start for real. Demo interaction wrote only
  `demo:sentinel:state`; Start for real removed that key.
- Reset's behavior remains truthful only at the UI level, not against the
  literal clear/remove claim (F-2-1/F-3-3).
- The live flow made same-origin requests only and set no cookies. After service
  worker control, `/demo/` reloaded offline with its six-visit sample visible.
- The built CLI ran in fresh HOME/TMPDIR directories, exported two profiles and
  six visits, wrote four exported files plus two reports, exited 0, and left the
  temporary directory empty. The real-home marker used by the registered test
  remained unchanged.

## Claim results

All 16 registered commands were attempted in their file order after `npm ci`
in the clean clone.

| Claim | Result |
| --- | --- |
| `demo-isolation` | PASS |
| `demo-sandbox` | **FAIL** — preview timed out because `dist/site` did not exist |
| `end-to-end-export` | PASS |
| `sample-domains` | PASS |
| `history-fields-only` | PASS |
| `copy-safety` | PASS |
| `verification` | PASS |
| `profile-discovery` | PASS |
| `actionable-errors` | PASS |
| `cli-privacy` | PASS |
| `no-overwrite` | PASS |
| `json-mode` | PASS |
| `license-version-package` | PASS |
| `site-build` | PASS |
| `site-privacy` | PASS after `site-build` created `dist/site` |
| `offline-reload` | PASS after `site-build` created `dist/site` |

The repository-wide `npm test` subsequently passed: 4 Rust unit tests, 4 Rust
integration tests, clippy, 13 CLI claims, the build, and 21 Playwright tests
passed; one desktop run of the mobile-only measurement was skipped. `npm run
check` also passed. Those aggregate results do not cure the exact clean-clone
claim failure in F-3-2.

## Earlier-finding recheck

Every prior review, polish report, verification report, and handoff was read.
The following mapping explicitly accounts for every earlier ID:

| Earlier IDs | Live/code confirmation in this round |
| --- | --- |
| F-1-1 | Audience wording is fixed; F-3-1 is a new desktop action-visibility failure. |
| F-1-2 | Fixed: one-click web demo and isolated CLI demo both work. |
| F-1-3 | Fixed as an existence issue: 16 claims are registered; F-3-2 is a new execution defect. |
| F-1-4, F-1-6–F-1-10, F-1-15–F-1-23 | Fixed by plain landing copy and passing export/copy/verification/demo tests. |
| F-1-5, F-1-11–F-1-14 | Fixed by `cli-privacy` and live same-origin/no-cookie evidence. |
| F-1-24–F-1-25 | Fixed by package/version/license checks and successful source build. |
| F-1-26–F-1-31 | Fixed by removed future copy, exact six-field output, and error tests. |
| F-1-32–F-1-45 | Fixed by current README and passing export, no-overwrite, verify, and JSON tests. |
| F-1-46 | **Not fixed**; repeated as F-2-2/F-3-4. |
| F-1-47–F-1-48 | Fixed by exact output-field checks. |
| F-1-49 | **Not fully fixed**; repeated as F-3-5. |
| F-1-50 | Fixed: clean build writes all required `dist/site` routes. |
| F-1-51 | **Not fixed**; repeated as F-2-3/F-3-6. |
| F-1-52–F-1-63 | Fixed by live request/storage/offline checks and CLI write/cleanup tests. |
| F-1-64–F-1-72 | Fixed by narrower copy, package contents, verification tests, and working documentation links. |
| F-1-73 | Fixed: service worker controls and reloads the demo offline. |
| F-1-74 | Fixed: unknown URLs return the designed product 404 with status 404 and same-origin assets. |
| F-1-75 | Fixed: permission discovery is covered by the passing unprivileged test. |
| F-1-76 | Fixed: local suite confirms 44px mobile targets and live pages have no overflow. |
| F-1-77–F-1-78 | Fixed: fingerprinted art caching and packaged LICENSE/CHANGELOG verify. |
| F-1-79–F-1-83 | Fixed: route titles/metadata, focus/history, common shell, and sitemap verify live. |
| F-1-84–F-1-97 | Fixed by the current short, literal vocabulary except the separate F-2-5/F-3-9 label. |
| F-1-98 | The internal future sentence was removed; its replacement creates F-2-4/F-3-7. |
| F-2-1 | **Not fixed**; repeated as F-3-3. |
| F-2-2 | **Not fixed**; repeated as F-3-4. |
| F-2-3 | **Not fixed**; repeated as F-3-6. |
| F-2-4 | **Not fixed**; repeated as F-3-7. |
| F-2-5 | **Not fixed**; repeated as F-3-9. |

## Structure, accessibility, and visual identity

- `/`, `/demo/`, `/privacy/`, `/terms/`, `/404.html`, and an unknown route
  have route-specific titles, descriptions, canonicals, OG/Twitter metadata,
  favicons, `lang=en`, one h1, and one main. The unknown route returns HTTP 404
  with the designed shell.
- Forward navigation and Back update the URL, title, h1, focus, and polite
  announcement after route loading. All discovered internal, hash, and GitHub
  links returned 200; the intentionally unknown route returned 404.
- Fresh live Axe scans found zero violations on every route at both viewports.
  Normal routes logged no console errors. Chromium logs the expected failed
  top-level resource for the intentional HTTP 404.
- The header/footer are consistent and include Privacy and Terms. The first
  load uses 6.33 kB JavaScript (2.63 kB gzip), with no remote font or script.
- The warm paper, cobalt, acid highlighter, hard rules/shadows, mono text, and
  evidence-desk art match `.factory/design.md`. The result is product-specific,
  not a generic SaaS template.

## Missed leverage

No missing AI feature, sync, or import is justified. The brief calls for a
deterministic local export of highly sensitive browser history. Remote AI or
sync would add privacy risk without improving the core job. JSON/CSV export,
verification, and a portable sample are already present; no provider key or AI
endpoint is embedded.

## What would make this perfect

Put the sample-data action fully above the desktop fold. Make every registered
claim command self-contained in a clean clone. Align Reset's words, behavior,
and test. Remove or register the five unlisted public assertions. Replace the
`report.json / format 1` label. Then repeat the complete cold, live, and
clean-clone review; PASS requires zero findings and no untested claim.
