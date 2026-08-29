# Copy audit — polish 3

Audited 2026-08-29. Counts treat URLs, commands, hyphenated terms, and version
strings as one word. No sentence exceeds 22 words. No banned marketing word
appears.

## Landing page sentences

| Words | Sentence |
| ---: | --- |
| 2 | Offline mode. |
| 7 | The guide and sample-data demo remain available. |
| 7 | Demo — sample data, nothing is saved. |
| 13 | For people archiving Firefox or Chromium history before changing a computer or account. |
| 8 | See a finished export and its verification report. |
| 5 | History stays on your computer. |
| 3 | Copy the database. |
| 3 | Export its records. |
| 4 | Verify the exported files. |
| 6 | Your history stays on your computer. |
| 7 | The CLI has no account or telemetry. |
| 7 | Keep the verification report with your export. |
| 8 | Run the check again after moving the folder. |
| 12 | Find Firefox and Chromium-family profiles in standard Windows, macOS, and Linux locations. |
| 14 | Copy the history database and its SQLite side files to a private temporary folder. |
| 14 | Check each file hash, the record count, and the date range against the report. |
| 6 | The demo uses reserved sample domains. |
| 7 | It never reads an installed browser profile. |
| 8 | The report names the profile and database file. |
| 9 | It does not store the source profile’s full path. |
| 9 | Build the single CLI with a current Rust toolchain. |
| 6 | It does not archive web pages. |
| 5 | It exports history records only. |
| 7 | It does not read passwords or cookies. |
| 6 | Encrypted browser data remains out of scope. |
| 5 | It does not hide errors. |
| 16 | Permission problems, unsupported database formats, and changing files stop with a specific code and next step. |
| 9 | Exports Firefox and Chromium history to JSON or CSV. |
| 7 | Built by Param Factory · build 0.1.0-polish.3. |
| 6 | No installed browser profile was read. |
| 5 | The export matches both reports. |
| 7 | No exported file or report was kept. |
| 14 | sentinel: places.sqlite contains no history visits; confirm the selected profile, then run sentinel scan. |
| 15 | sentinel: cannot read/copy History: permission denied; close the browser or fix OS permissions, then retry. |
| 6 | No partial export folder was kept. |
| 5 | Command copied to the clipboard. |
| 4 | Clipboard access was blocked. |
| 6 | Select and copy the command manually. |

Headings and controls were checked separately. The h1 and h2 headings name
their sections. `Try it with sample data`, Install, Copy, Reset demo, Start for
real, and the three demo state controls name their result or state.
`Verification report` replaces the unexplained `report.json / format 1` label.

## Demo route sentences

| Words | Sentence |
| ---: | --- |
| 7 | Demo — sample data, nothing is saved. |
| 8 | Switch between a verified export and two failures. |
| 8 | Reset returns the sample to its original state. |
| 11 | Firefox and Chromium each contribute three visits from reserved `.example` domains. |

## README sentences

| Words | Sentence |
| ---: | --- |
| 16 | History Export Sentinel is a local CLI for people moving or archiving Firefox and Chromium history. |
| 8 | Sentinel copies each history database before reading it. |
| 5 | It exports JSON or CSV. |
| 12 | It records counts, the date range, and file hashes for later checks. |
| 12 | The CLI does not upload history or modify an installed browser profile. |
| 14 | An empty history database stops with an error instead of creating an empty export. |
| 9 | The sample command needs no browser profile or account. |
| 10 | It creates two sample profiles in a private temporary folder. |
| 11 | It exports and verifies six visits, then prints the output folder. |
| 14 | After building and previewing the site, open `/?demo=1` to enter the sample-data demo directly. |
| 6 | See `.factory/demo.md` for its isolation rules. |
| 7 | Install with a current stable Rust toolchain. |
| 10 | The project version is 0.1.0 and the license is MIT. |
| 3 | Find supported profiles. |
| 8 | Export all detected profiles or choose one profile. |
| 12 | The output folder must be new, so existing files cannot be overwritten. |
| 8 | Check the exported files against each verification report. |
| 5 | JSON records use this format. |
| 7 | `--json` makes command results readable by scripts. |
| 7 | It does not change the exported files. |
| 8 | The CLI has no network or telemetry code. |
| 10 | Sentinel reads history URLs, titles, visit times, and visit counts. |
| 12 | It does not read passwords, cookies, page bodies, or encrypted browser data. |
| 9 | Install stable Rust, Node.js 20 or newer, and npm. |
| 2 | Then run. |
| 9 | `npm run build` writes the static site to `dist/site/`. |
| 11 | The site has no analytics, cookies, third-party scripts, or remote fonts. |
| 10 | The service worker caches its public pages for offline reading. |
| 6 | Read the privacy policy and terms. |
| 1 | MIT. |
| 2 | See LICENSE. |

README headings are literal section names. `CLI`, SQLite, JSON, CSV, SHA-256,
and Rust are necessary terms for this command-line product.

## Terminology

| Concept | Term used |
| --- | --- |
| Try-out experience | sample-data demo |
| Time span | date range |
| Proof document | verification report |
| Produced JSON or CSV | exported file |
| Destination directory | output folder |
| Browser input | history database |

Removed statements do not leave implied claims: release availability, the
deployed URL, the composition of `npm test`, and interactive-prompt behavior
are no longer asserted in public copy.
