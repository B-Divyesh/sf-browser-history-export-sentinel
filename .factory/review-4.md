# Adversarial first-read review 4 — History Export Sentinel

**Verdict: PASS**

Reviewed 2026-08-29 UTC at `98dc5417adf5c395189f6a9ac4c1229eeb6760e3`
and against <https://browser-history-export-sentinel.sociobot.in>. This review
used fresh Chromium contexts at 390×844 and 1440×900, plus the clean clone at
`/tmp/history-sentinel-review4-9fcEx0/repo`.

No findings remain. There are no untested registry claims.

## Cold first screen

Before scrolling, the phone and desktop pages both answered the three required
questions:

| Question | Answer from the first screen |
| --- | --- |
| What does this do? | It exports and verifies Firefox or Chromium browser history. |
| Who is it for? | “For people archiving Firefox or Chromium history before changing a computer or account.” |
| What should I click first? | **Try it with sample data**, which says it will show a finished export and verification report. |

The sample action and all three facts were visible above the fold. Measured
action/facts bottoms were 559.6/809.0 px at 390×844 and 702.6/825.6 px at
1440×900. Neither viewport had horizontal overflow or console errors.

## Copy audit

Counts treat URLs, commands, hyphenated terms, and versions as one word.
There are no sentences over 22 words, banned marketing words, unexplained mood
headings, inconsistent product terms, or non-result-naming buttons. Headings
name their sections; navigation links are not action buttons. `CLI`, `SQLite`,
JSON, CSV, SHA-256, and Rust are necessary terms for this command-line product.

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
| 21 | 9 | Build the single CLI with a current Rust toolchain. |
| 22 | 6 | It does not archive web pages. |
| 23 | 5 | It exports history records only. |
| 24 | 7 | It does not read passwords or cookies. |
| 25 | 6 | Encrypted browser data remains out of scope. |
| 26 | 5 | It does not hide errors. |
| 27 | 16 | Permission problems, unsupported database formats, and changing files stop with a specific code and next step. |
| 28 | 9 | Exports Firefox and Chromium history to JSON or CSV. |
| 29 | 7 | Built by Param Factory · build 0.1.0-polish.3. |
| 30 | 6 | No installed browser profile was read. |
| 31 | 5 | The export matches both reports. |
| 32 | 7 | No exported file or report was kept. |
| 33 | 14 | sentinel: places.sqlite contains no history visits; confirm the selected profile, then run sentinel scan. |
| 34 | 15 | sentinel: cannot read/copy History: permission denied; close the browser or fix OS permissions, then retry. |
| 35 | 6 | No partial export folder was kept. |
| 36 | 5 | Command copied to the clipboard. |
| 37 | 4 | Clipboard access was blocked. |
| 38 | 6 | Select and copy the command manually. |

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
| 10 | 14 | After building and previewing the site, open `/?demo=1` to enter the sample-data demo directly. |
| 11 | 6 | See `.factory/demo.md` for its isolation rules. |
| 12 | 7 | Install with a current stable Rust toolchain. |
| 13 | 10 | The project version is 0.1.0 and the license is MIT. |
| 14 | 3 | Find supported profiles. |
| 15 | 8 | Export all detected profiles or choose one profile. |
| 16 | 12 | The output folder must be new, so existing files cannot be overwritten. |
| 17 | 8 | Check the exported files against each verification report. |
| 18 | 5 | JSON records use this format. |
| 19 | 7 | `--json` makes command results readable by scripts. |
| 20 | 7 | It does not change the exported files. |
| 21 | 8 | The CLI has no network or telemetry code. |
| 22 | 10 | Sentinel reads history URLs, titles, visit times, and visit counts. |
| 23 | 12 | It does not read passwords, cookies, page bodies, or encrypted browser data. |
| 24 | 9 | Install stable Rust, Node.js 20 or newer, and npm. |
| 25 | 2 | Then run. |
| 26 | 9 | `npm run build` writes the static site to `dist/site/`. |
| 27 | 10 | Deploy the contents of `dist/site/` as the static site root. |
| 28 | 11 | The site has no analytics, cookies, third-party scripts, or remote fonts. |
| 29 | 10 | The service worker caches its public pages for offline reading. |
| 30 | 6 | Read the privacy policy and terms. |
| 31 | 1 | MIT. |
| 32 | 2 | See LICENSE. |

Each claim-like sentence maps to the registry: export/report behavior to
`end-to-end-export`, copying to `copy-safety`, errors to `actionable-errors`,
data limits to `history-fields-only`, privacy to `cli-privacy` or
`site-privacy`, demo behavior to `demo-isolation`/`demo-sandbox`/
`sample-domains`, platform discovery to `profile-discovery`, package facts to
`license-version-package`, output refusal and JSON mode to their named claims,
and offline/build statements to `offline-reload` and `site-build`.

## Demo and sandbox

The hero action opened `/?demo=1` in one click. The first displayed product
screen already showed a verified Firefox/Chromium sample with two profiles,
six visits, JSON/CSV output, hashes, and a verification result. The persistent
banner read “Demo — sample data, nothing is saved.” It included working **Reset
demo** and **Start for real** controls.

In a fresh live browser context, a seeded `real:sentinel:marker=keep` survived
entry and selecting Empty history. The only demo write was
`demo:sentinel:state`. Reset restored the verified display, removed every
`demo:` key, and retained the seeded real key. Start for real removed demo
state and reached `/#install`.

The browser request log across home, demo, legal routes, the first-party 404,
and the demo interaction contained only the product origin. Cookies were
empty. After service-worker control, `/?demo=1` reloaded offline and showed
“Try a verified history export” and “Six sample visits.” The intentional 404
request emitted the expected failed-resource console entry; normal routes had
no console errors.

The installed CLI was also exercised from a temporary location. `sentinel demo
--output /tmp/history-sentinel-review4-installed-output` produced two profile
folders containing JSON, CSV, and `report.json`; it reported two profiles and
six records. A marker in a fresh `HOME` remained unchanged. The landing page’s
`cargo install --git … history-export-sentinel` command installed the same
`sentinel 0.1.0` binary successfully.

## Claims and repository gates

All registry commands were run separately, in registry order, after `npm ci`
in the clean clone. The browser commands ran before any prior tracked or
generated `dist/site` and passed.

| Claim | Result |
| --- | --- |
| `demo-isolation` | PASS |
| `demo-sandbox` | PASS |
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
| `site-privacy` | PASS |
| `offline-reload` | PASS |

`npm test` passed: 4 Rust unit tests, 4 Rust integration tests, clippy, 13 CLI
claim checks, and 26 Playwright tests. `npm run check` also passed.

## Earlier-finding recheck

Every earlier review, polish report, verification report, and handoff was read.
The live site and current source confirm every earlier ID is fixed, rather than
only marked fixed:

| Earlier finding IDs | Confirmation in this round |
| --- | --- |
| F-1-1 | The job, audience, action, and facts fit above both tested folds. |
| F-1-2 | The real CLI and browser sample demos work and use isolated state. |
| F-1-3 | The registry exists and all 16 listed commands pass independently. |
| F-1-4–F-1-10, F-1-15–F-1-23 | Plain export, copy, sample, report, and verification claims are registered and observed. |
| F-1-5, F-1-11–F-1-14 | CLI privacy and live same-origin/no-cookie behavior are confirmed. |
| F-1-24–F-1-31 | Package, source installation, scope boundaries, and actionable errors are confirmed. |
| F-1-32–F-1-45 | README instructions, no-overwrite behavior, reports, record schema, and JSON mode are confirmed. |
| F-1-46, F-1-49, F-1-51 | The unlisted prompt, aggregate-test, and deployment assertions remain absent. |
| F-1-47–F-1-48, F-1-52–F-1-72 | Field exclusions, storage isolation, output containment, package materials, privacy, and documentation remain confirmed. |
| F-1-73–F-1-83 | Offline reload, first-party 404, permission errors, touch targets, immutable assets, metadata, history/focus, shell, and sitemap remain confirmed. |
| F-1-84–F-1-98 | Current landing/README copy is short, literal, consistently named, and free of the prior binary-availability wording. |
| F-2-1–F-2-5 | Reset now removes demo keys; the unlisted copy is absent; `Verification report` is the present label. |
| F-3-1 | The desktop sample action and facts fit above the 900px fold. |
| F-3-2 | Browser claim commands build before preview and passed from a clean clone. |
| F-3-3 | Reset’s observed storage result is zero `demo:` keys. |
| F-3-4–F-3-9 | The unlisted phrases and unexplained label remain removed or corrected. |

## Structure, accessibility, and visual identity

`/`, `/demo/`, `/privacy/`, `/terms/`, and `/404.html` returned 200 and had the
expected route-specific titles, descriptions, canonicals, OG/Twitter art,
favicon/touch icon, `lang=en`, one h1, and one main. An unknown route returned
the designed first-party 404 with HTTP 404 and the configured security headers.
The robots file, sitemap, every internal target, and the GitHub source link
resolved. The SPA test confirms forward/back title updates, h1 focus, polite
route announcements, and restored navigation state.

The local accessibility suite found no serious or critical Axe issues. Keyboard
tabs, skip link, visible focus, arrow-key tabs, reduced motion, 44px controls,
and 390px overflow checks passed. The evidence-desk design is visibly distinct:
warm paper, hard ink rules and shadows, cobalt controls, acid focus markers,
monospace evidence fields, and original desk art align with `.factory/design.md`
without resembling a generic SaaS template.

## Missed leverage

No additional AI, sync, or import feature is expected. The brief calls for a
deterministic local export of sensitive history. JSON/CSV output, profile
discovery, verification, and a portable isolated sample already cover the
valuable adjacent work; remote inference would add risk without improving it.

## What would make this perfect

No outstanding change was identified. Preserve this complete cold-browser,
clean-clone claim, CLI-demo, and live offline/privacy check on future changes.
