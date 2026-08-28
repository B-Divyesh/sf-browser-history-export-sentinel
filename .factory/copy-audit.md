# Copy audit — polish 1

Audited 2026-08-28. Counts treat hyphenated terms and commands as one word.
No sentence exceeds 22 words. No banned marketing word appears.

## Landing page sentences

| Words | Sentence |
| ---: | --- |
| 9 | Offline mode. The guide and sample-data demo remain available. |
| 11 | Demo — sample data, nothing is saved to your browser history. |
| 12 | For people archiving Firefox or Chromium history before changing a computer or account. |
| 9 | See a finished export and its verification report. |
| 3 | Copy the database. |
| 3 | Export its records. |
| 4 | Verify the exported files. |
| 6 | Your history stays on your computer. |
| 7 | The CLI has no account or telemetry. |
| 6 | Keep the verification report with your export. |
| 8 | Run the check again after moving the folder. |
| 11 | Find Firefox and Chromium-family profiles in standard Windows, macOS, and Linux locations. |
| 13 | Copy the history database and its SQLite side files to a private temporary folder. |
| 12 | Check each file hash, the record count, and the date range against the report. |
| 7 | The demo uses reserved sample domains. |
| 8 | It never reads an installed browser profile. |
| 10 | The report names the profile and database file. |
| 11 | It does not store the source profile’s full path. |
| 6 | Signed binaries are not available yet. |
| 10 | Build the single CLI with a current Rust toolchain. |
| 6 | It does not archive web pages. |
| 5 | It exports history records only. |
| 7 | It does not read passwords or cookies. |
| 6 | Encrypted browser data remains out of scope. |
| 6 | It does not hide errors. |
| 15 | Permission problems, unsupported database formats, and changing files stop with a specific code and next step. |
| 8 | Exports Firefox and Chromium history to JSON or CSV. |
| 5 | Command copied to the clipboard. |
| 4 | Clipboard access was blocked. |
| 7 | Select and copy the command manually. |

Headings and controls are sentence fragments. They were also checked for plain
terms: `Export and verify browser history`, `Try it with sample data`, `How
Sentinel verifies an export`, `See successful and failed exports`, `What the
verification report records`, `Install Sentinel from source`, `What Sentinel
does not do`, `Reset demo`, and `Start for real`.

Dynamic sample output uses short status lines. User-facing terms are `sample-data
demo`, `date range`, `verification report`, `exported file`, and `output folder`.
The words `fixture`, `artifact`, `bounds`, `receipt`, `schema`, `non-zero`,
`copy-first`, and `cross-platform` do not appear in public site copy.

## Terminology

| Concept | Term used |
| --- | --- |
| Try-out experience | sample-data demo |
| Time span | date range |
| Proof document | verification report |
| Produced JSON or CSV | exported file |
| Destination directory | output folder |
| Browser input | history database |
