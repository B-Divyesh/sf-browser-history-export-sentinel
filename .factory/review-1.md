# Adversarial first-read review 1 — History Export Sentinel

**Verdict: FAIL**

Candidate `1ea21df25b5d189d69de43b0d63c44bd59671f7c`; live URL
<https://browser-history-export-sentinel.sociobot.in>; reviewed 2026-08-28 UTC
in fresh Chromium contexts at 390×844 and 1440×900.

## Cold first screen

Before scrolling, both contexts showed “Local CLI Firefox + Chromium,” “Export
history. Prove it.,” a 26-word description, “Install Sentinel,” and “Inspect the
recorded run.” My answers were:

- **What:** export Firefox/Chromium history, then record counts, dates, and
  hashes for checking it.
- **For whom:** cannot answer. The exact description is “A cross-platform,
  copy-first history exporter that fails loudly when the result is empty—and
  gives you the hashes, counts, and date range to check it later.” It names no
  moving, archiving, machine-change, or account-change situation.
- **First click:** “Install Sentinel” is primary. The secondary action is a page
  anchor, not a usable demo.

## Blocking findings

### F-1-1 — first screen does not identify the user

The quoted description is also 26 words, over the 22-word cap. A cold visitor
cannot identify the intended situation. **Fix:** h1 `Export and verify browser
history`; sentence `For people archiving Firefox or Chromium history before
changing a computer or account.`

### F-1-2 — no one-click isolated demo

“Inspect the recorded run” jumps to `/#demo`. `/demo` is Azure's 404 and
`/?demo=1` is the ordinary landing page. In a new temp directory, `sentinel
demo` and `sentinel --demo` both exit 2. There is no `examples/`, banner, Reset,
Start for real, storage namespace, or `.factory/demo.md`. **Fix:** bundle
realistic Firefox and Chromium samples; implement `sentinel demo` in a new temp
directory; make `Try it with sample data` the hero action; add `/demo` with the
required banner/actions; test that no real profile/storage is touched.

### F-1-3 — claims registry is absent

`.factory/claims.json` does not exist. Thus zero listed claim commands could be
run. `npm test` passes, but it contains no `@claim:<id>` tests and does not enter
a real demo. Every claim below is untested. **Fix:** create the registry and one
tagged, observable clean-demo test for every retained claim; otherwise delete
the claim.

### F-1-4 through F-1-72 — unlisted claims

Each table row is a separate blocking finding. The exact sentence/fragment is
quoted. The proposed test name is the concrete fix; it must exercise bundled
sample data from a clean temp directory/context and assert the outcome, not the
presence of UI. Repeated claims may point to the same test, but each public
location must be listed in `where`.

| ID | Location and exact claim | Add/test, or remove |
|---|---|---|
| F-1-4 | Landing: “A cross-platform, copy-first history exporter that fails loudly when the result is empty—and gives you the hashes, counts, and date range to check it later.” | `@claim:export-report`: empty failure, hashes, count, range |
| F-1-5 | “0 network calls” | `@claim:cli-no-network`: process socket trace |
| F-1-6 | “2 browser families” | `@claim:browser-families`: Firefox + Chromium samples |
| F-1-7 | “SHA-256 on every artifact” | `@claim:artifact-hashes`: every output file has valid digest |
| F-1-8 | “Database copied first.” | `@claim:copy-before-read`: trace open order/paths |
| F-1-9 | “Export read second.” | same ordering test |
| F-1-10 | “Evidence checked last.” | `@claim:verify-after-export` |
| F-1-11 | “History stays on this machine.” | `@claim:history-local`: browser/process traffic trace |
| F-1-12 | “No account.” | `@claim:no-account`: clean demo without credentials |
| F-1-13 | “No cloud.” | `@claim:no-cloud`: no remote traffic |
| F-1-14 | “No telemetry.” | `@claim:no-telemetry`: whole-flow request/socket log |
| F-1-15 | “Sentinel treats verification as part of the export—not a chore you remember after wiping the old machine.” | Assert every successful export has a verifiable report, or rewrite as instruction |
| F-1-16 | “Detect Firefox, Chrome, Chromium, Edge, Brave, and Vivaldi profiles in their native Windows, macOS, and Linux locations.” | `@claim:profile-discovery`: all browser/OS fixture layouts and errors |
| F-1-17 | “Copy the history database and SQLite sidecars into a private temporary workspace.” | `@claim:copy-sidecars`: DB/WAL/SHM and private temp permissions |
| F-1-18 | “The live profile is never opened by SQLite.” | `@claim:live-db-not-opened`: trace SQLite paths |
| F-1-19 | “Recompute artifact hashes, record counts, and visit bounds.” | `@claim:verify-report`: mutate every dimension |
| F-1-20 | “A mismatch exits non-zero with the exact file to inspect.” | `@claim:mismatch-exit`: assert code and exact path |
| F-1-21 | “This browser demo uses synthetic URLs.” | Replace recording with demo; assert reserved sample domains |
| F-1-22 | “The real CLI works offline against your copied database and never sends its contents here.” | `@claim:cli-offline`: network disabled + request trace |
| F-1-23 | “The report stores no absolute profile path.” | `@claim:no-absolute-path`: scan all outputs |
| F-1-24 | “Free / MIT / v0.1.0” | `@claim:license-version-price` |
| F-1-25 | “Build from source today.” | `@claim:source-install`: clean Cargo root |
| F-1-26 | “Signed release binaries can be added by the factory without changing the CLI or privacy model.” | Remove untestable internal/future copy |
| F-1-27 | “No page archiving.” | `@claim:no-page-archive`: no bodies/assets |
| F-1-28 | “It exports history records, not copies of visited websites.” | same output-content test |
| F-1-29 | “No encryption bypass.” | `@claim:no-encryption-bypass` |
| F-1-30 | “Passwords, cookies, and encrypted browser data remain out of scope.” | `@claim:history-fields-only` |
| F-1-31 | “Permission, schema, and changing-file errors stop the export with an actionable exit code.” | `@claim:actionable-errors`: cases, codes, advice, no partial output |
| F-1-32 | README: “History Export Sentinel is a private, local CLI for people moving or archiving Firefox and Chromium history.” | `@claim:local-cli` |
| F-1-33 | “It finds profiles, copies each SQLite database to a temporary workspace, exports normalized JSON and/or CSV, and writes a report with record counts, date bounds, and SHA-256 hashes that can be checked later.” | `@claim:end-to-end-export`: every stated stage/output |
| F-1-34 | “It never uploads history, never modifies a live profile, and never treats an empty export as success.” | `@claim:source-safety`: traffic, source hashes, empty exit |
| F-1-35 | “Download the single binary for your platform from Releases, or build it with a current stable Rust toolchain:” | **False:** GitHub says “There aren’t any releases here.” Publish tested signed binaries or state source-only |
| F-1-36 | “The project starts at version 0.1.0.” | `@claim:version` |
| F-1-37 | “Registry publishing is handled by the Param Factory; maintainers can validate the package with cargo package --manifest-path cli/Cargo.toml.” | Remove factory-process copy; test packaging in contributor docs |
| F-1-38 | “List detected Firefox, Chrome, Chromium, Edge, Brave, and Vivaldi profiles:” | `@claim:profile-discovery` |
| F-1-39 | “Export every detected profile, or select a profile directory/database.” | `@claim:profile-selection`: auto, directory, database |
| F-1-40 | “The destination must not already exist, which prevents accidental overwrites.” | `@claim:no-overwrite`: existing contents unchanged |
| F-1-41 | “Each profile gets a normalized export and report.json.” | `@claim:per-profile-output` |
| F-1-42 | “Verify the report and file hashes before changing machines or accounts:” | `@claim:verify-command`: intact pass, changed fail |
| F-1-43 | “The JSON records use this stable v1 shape:” | `@claim:json-v1`: validate against checked-in schema |
| F-1-44 | “--json makes command results machine-readable.” | `@claim:json-mode`: parse stdout for every command |
| F-1-45 | “It does not change the export format.” | `@claim:json-mode-format`: compare artifacts |
| F-1-46 | “There are no prompts, network calls, telemetry, or hidden retries.” | `@claim:noninteractive-offline`: closed stdin, traffic, documented retries |
| F-1-47 | “Browser passwords and encrypted data are deliberately out of scope.” | `@claim:history-fields-only` |
| F-1-48 | “Sentinel only reads visit URLs, page titles, timestamps, and aggregate visit counts.” | `@claim:history-fields-only`: source reads + output schema |
| F-1-49 | “npm test runs Rust unit/integration tests and site tests.” | CI smoke check observes all stages |
| F-1-50 | “npm run build produces the static landing/docs site at dist/site/, exactly matching the factory work order.” | `@claim:site-build-output` |
| F-1-51 | “The static site is deployed at https://browser-history-export-sentinel.sociobot.in.” | `@claim:deployment`: live bytes/build ID |
| F-1-52 | “It has no analytics, cookies, third-party scripts, or remote fonts.” | `@claim:site-privacy`: storage/requests on every route; currently fails on 404 |
| F-1-53 | “The CLI is offline-only; exports remain wherever you place them.” | `@claim:cli-offline-output`: network off + write trace |
| F-1-54 | “MIT.” | `@claim:license`: repo and packaged crate license text |
| F-1-55 | Privacy: “History Export Sentinel is designed so your browser history does not need to leave your computer.” | `@claim:history-local` |
| F-1-56 | “The command-line tool has no network client, account system, telemetry, advertising, or analytics.” | `@claim:cli-privacy`: dependency/source + traffic trace |
| F-1-57 | “When you run an export, Sentinel copies the selected browser history SQLite database and its present sidecars to a temporary local directory.” | `@claim:copy-sidecars` |
| F-1-58 | “It reads the copy to export URLs, page titles, visit timestamps, and visit counts.” | `@claim:history-fields-only` |
| F-1-59 | “The temporary copy is deleted when the command finishes.” | `@claim:temp-cleanup`: success and every failure |
| F-1-60 | “Sentinel writes JSON and/or CSV history records and a verification report only to the output directory you choose.” | `@claim:write-scope`: trace persistent writes |
| F-1-61 | “The report uses a profile label and database filename, not the absolute source path.” | `@claim:no-absolute-path` |
| F-1-62 | “This static documentation site sets no cookies and uses no analytics, tracking pixels, third-party fonts, or third-party scripts.” | `@claim:site-privacy`, including demo/404 |
| F-1-63 | “A service worker caches the public site files on your device for offline reading.” | `@claim:offline-reload`; currently fails |
| F-1-64 | “Because the software is local and anonymous, we cannot identify or retrieve your exports.” | Prove no identifiers/account/upload, or remove “anonymous” |
| F-1-65 | Terms: “History Export Sentinel is free, open-source software provided under the MIT License.” | `@claim:license-version-price` |
| F-1-66 | “Sentinel does not encrypt exports or upload them for you.” | `@claim:no-encryption-or-upload` |
| F-1-67 | “Verification confirms that the exported artifacts still match the report Sentinel created; it does not prove that a browser itself retained every historical visit or that another program did not alter the source before export.” | `@claim:verification-scope`: intact/tampered behavior |
| F-1-68 | “Versioned behavior and notable changes are recorded in the project changelog.” | `@claim:changelog`: current version entry |
| F-1-69 | CLI README: “An offline, copy-first Firefox and Chromium history exporter with normalized JSON/CSV output and SHA-256 verification reports.” | `@claim:end-to-end-export` |
| F-1-70 | “Empty history, permission failures, unsupported schemas, and verification mismatches return distinct non-zero exit codes.” | `@claim:actionable-errors` |
| F-1-71 | “The CLI has no telemetry or network client and never opens a live profile database through SQLite.” | `@claim:cli-privacy` + `@claim:live-db-not-opened` |
| F-1-72 | “Full documentation and the exit-code contract are available in the project repository.” | Link exact document; test every documented code |

### Carried-forward defects

The earlier `.factory/handoff.md` and `.factory/verification.md` were read in
full. No earlier `review-*.md` or `polish-*.md` existed. Every earlier defect
below remains and is therefore blocking again under its original ID.

- **F-1-73 / V1 — service worker still cannot install.** Live `sw.js` still
  precaches the 404ing `/staticwebapp.config.json`. Fresh mobile and desktop
  contexts had zero registrations, no controller, and an orphaned cache;
  offline reload failed with `ERR_INTERNET_DISCONNECTED`. Exclude deployment
  files and add `@claim:offline-reload` on production-equivalent hosting.
- **F-1-74 / V2 — third-party Azure 404 remains.** `/demo` and an unknown route
  have no h1/product shell, omit site security headers, request
  `ajax.aspnetcdn.com` and `appservice.azureedge.net`, and log an error. Add a
  designed first-party 404 and Azure `responseOverrides`; test same-origin only.
- **F-1-75 / V3 — discovery still hides permissions.** As UID 65534, an
  unreadable Firefox root made scan return exit 0/empty and export exit 10.
  `read_dir` errors are discarded and entries flattened. Return exit 11 with
  path/advice and test it.
- **F-1-76 / V4 — mobile targets remain small.** At 390px: Demo 29×19,
  privacy-rail link 221×25, footer Privacy/Terms/Source 59×22, 42×22, 50×22.
  Give every control a tested 44px minimum hit area.
- **F-1-77 / V5 — unfingerprinted image remains immutable for a year.** Live
  `/evidence-desk.webp` and repo config use `max-age=31536000, immutable`.
  Fingerprint it or require revalidation.
- **F-1-78 / V6 — crate still omits LICENSE/CHANGELOG.** `cargo package --list`
  confirms both absent. Include and test them.

## Minor findings

- **F-1-79 — vague home title.** `History Export Sentinel — export it, then
  prove it` does not name “it.” Use `History Export Sentinel — verify browser
  history exports`. Privacy/Terms titles pass.
- **F-1-80 — metadata incomplete.** All product pages lack canonical, OG,
  Twitter card, and apple-touch metadata; no 1200×630 product OG image exists.
  Add route-specific metadata and original product art.
- **F-1-81 — route focus is absent.** After Home → Privacy and Back, active
  element was BODY; no announcement exists. Focus the destination h1 and
  announce it. Back did restore scroll and all existing deep anchors resolved.
- **F-1-82 — shell is inconsistent.** Home and legal headers have different
  nav; legal footers omit “Built by Param Factory”; all footers omit build ID.
  Use one shared Home/Demo/Privacy shell with Terms, one-liner, provenance, ID.
- **F-1-83 — sitemap lacks required routes.** It lists only home/privacy/terms
  because demo and designed 404 do not exist. Implement and list them.
- **F-1-84 — overlong/jargon hero sentence.** The 26-word F-1-1 quote uses
  “cross-platform” and “copy-first.” Use F-1-1's 13-word audience sentence.
- **F-1-85 — overlong README sentence.** The 33-word F-1-33 quote exceeds 22.
  Use: `Sentinel finds Firefox and Chromium profiles and copies each history
  database before reading it. It exports JSON or CSV. It records counts, the
  date range, and file hashes for later checks.`
- **F-1-86 — non-section heading.** “A file existing is not proof.” → `How
  Sentinel verifies an export`.
- **F-1-87 — mood heading.** “See every ending.” → `See successful and failed
  exports`.
- **F-1-88 — metaphor heading.** “Keep the receipt with the archive.” → `What
  the verification report records`.
- **F-1-89 — non-section install heading.** “One binary. No account.” →
  `Install Sentinel from source` until binaries exist.
- **F-1-90 — slogan.** “Portability without guesswork.” → `Exports Firefox and
  Chromium history to JSON or CSV.`
- **F-1-91 — vague term.** “cross-platform” → tested `Windows, macOS, and Linux`.
- **F-1-92 — jargon.** “copy-first” → `copies the database before reading it`.
- **F-1-93 — jargon.** “Copy / normalize / attest” → `Copy, export, and verify`.
- **F-1-94 — test jargon.** “recorded fixture”/“fixture” → `sample data`.
- **F-1-95 — jargon.** “artifact” → `exported file` in user copy.
- **F-1-96 — jargon.** “non-zero”/“schema” sentence → `Permission errors,
  unsupported database formats, and files that change during export stop with
  a specific error and next step.`
- **F-1-97 — inconsistent terms.** Use `sample-data demo` (not recorded
  run/browser demo/fixture), `date range` (not bounds), `verification report`
  (not receipt/evidence), and `exported file` (reserve archive for the folder).
- **F-1-98 — internal future copy.** Replace “Signed release binaries can be
  added by the factory…” with `Signed binaries are not available yet. Install
  from source with Cargo.`

## Copy audit

Counts treat hyphenated/slash terms, versions, and commands as one word. † marks
a copy finding above. Dynamic demo/error sentences are included.

### Landing page — every sentence

|#|Words|Sentence|
|---:|---:|---|
|1|9|Offline mode — the guide and recorded demo still work.|
|2|6|Sentinel itself never needs a network.|
|3|2|Export history.|
|4|2|Prove it.|
|5|26|A cross-platform, copy-first history exporter that fails loudly when the result is empty—and gives you the hashes, counts, and date range to check it later. †|
|6–8|3 each|Database copied first. / Export read second. / Evidence checked last.|
|9|5|History stays on this machine.|
|10–12|2 each|No account. / No cloud. / No telemetry.|
|13|6|A file existing is not proof. †|
|14|18|Sentinel treats verification as part of the export—not a chore you remember after wiping the old machine.|
|15|17|Detect Firefox, Chrome, Chromium, Edge, Brave, and Vivaldi profiles in their native Windows, macOS, and Linux locations.|
|16|12|Copy the history database and SQLite sidecars into a private temporary workspace.|
|17|8|The live profile is never opened by SQLite.|
|18|8|Recompute artifact hashes, record counts, and visit bounds. †|
|19|10|A mismatch exits non-zero with the exact file to inspect. †|
|20|3|See every ending. †|
|21|6|This browser demo uses synthetic URLs.|
|22|15|The real CLI works offline against your copied database and never sends its contents here.|
|23|3|JavaScript is off.|
|24|14|The verified fixture is shown; run the CLI to inspect empty and locked-profile errors. †|
|25|7|Keep the receipt with the archive. †|
|26|7|The report stores no absolute profile path.|
|27|13|Share the archive only if you mean to share the history inside it.|
|28–29|2 each|One binary. † / No account.|
|30|4|Build from source today.|
|31|16|Signed release binaries can be added by the factory without changing the CLI or privacy model. †|
|32|5|What Sentinel does not do.|
|33|3|No page archiving.|
|34|9|It exports history records, not copies of visited websites.|
|35|3|No encryption bypass.|
|36|10|Passwords, cookies, and encrypted browser data remain out of scope.|
|37|3|No silent recovery.|
|38|13|Permission, schema, and changing-file errors stop the export with an actionable exit code. †|
|39|3|Portability without guesswork. †|
|40|5|Built by the Param Factory.|
|41|7|No artifact or report was kept.|
|42–43|5 each|No live database was opened. / No partial archive was kept.|
|44|5|Command copied to the clipboard.|
|45|4|Clipboard access was blocked.|
|46|7|Select and copy the command manually.|

Fragments/controls checked: “Local CLI Firefox + Chromium,” “Copy / normalize /
attest,” “Recorded fixture / no local access,” “report.json / schema v1,” “Free
/ MIT / v0.1.0,” “Honest boundaries,” Install, Inspect, state tabs, and Copy
actions. Install and the accessible Copy names are result-naming verbs. The
required sample action is absent. No banned marketing word appears.

### Root README — every sentence

|#|Words|Sentence|
|---:|---:|---|
|1|17|History Export Sentinel is a private, local CLI for people moving or archiving Firefox and Chromium history.|
|2|33|It finds profiles, copies each SQLite database to a temporary workspace, exports normalized JSON and/or CSV, and writes a report with record counts, date bounds, and SHA-256 hashes that can be checked later. †|
|3|17|It never uploads history, never modifies a live profile, and never treats an empty export as success.|
|4|18|Download the single binary for your platform from Releases, or build it with a current stable Rust toolchain: †|
|5|6|The project starts at version 0.1.0.|
|6|18|Registry publishing is handled by the Param Factory; maintainers can validate the package with cargo package --manifest-path cli/Cargo.toml. †|
|7|10|List detected Firefox, Chrome, Chromium, Edge, Brave, and Vivaldi profiles:|
|8|9|Export every detected profile, or select a profile directory/database.|
|9|10|The destination must not already exist, which prevents accidental overwrites.|
|10|8|Each profile gets a normalized export and report.json.|
|11|11|Verify the report and file hashes before changing machines or accounts:|
|12|8|The JSON records use this stable v1 shape:|
|13|5|--json makes command results machine-readable.|
|14|7|It does not change the export format.|
|15|10|There are no prompts, network calls, telemetry, or hidden retries.|
|16|10|Browser passwords and encrypted data are deliberately out of scope.|
|17|12|Sentinel only reads visit URLs, page titles, timestamps, and aggregate visit counts.|
|18|7|Prerequisites: stable Rust, Node.js 20+, and npm.|
|19|9|npm test runs Rust unit/integration tests and site tests.|
|20|16|npm run build produces the static landing/docs site at dist/site/, exactly matching the factory work order.|
|21|16|Run npm run dev for the site and cargo test --manifest-path cli/Cargo.toml for the CLI alone.|
|22|8|The static site is deployed at https://browser-history-export-sentinel.sociobot.in.|
|23|10|It has no analytics, cookies, third-party scripts, or remote fonts.|
|24|10|The CLI is offline-only; exports remain wherever you place them.|
|25|7|See the site's /privacy/ and /terms/ pages.|
|26|1|MIT.|
|27|2|See LICENSE.|

README headings name their sections. `cli/README.md` has four short sentences
(12, 13, 15, 10 words); none exceeds 22, but F-1-69–72 cover its claims.

## Remaining checks and evidence

- Normal home/privacy/terms requests were same-origin; `/demo` and 404 requested
  two Microsoft origins. With no demo, Reset, storage isolation, and preservation
  of real data cannot be verified.
- No AI feature, provider key, Azure model endpoint, or Sociobot request exists.
  That is appropriate for deterministic sensitive-data export. The obvious
  missed leverage is tested signed binaries: README advertises them but none
  exist (F-1-35). Import is not required because the brief forbids touching live
  databases; copy must not imply restore support.
- Existing links and hash targets returned 200/resolved. `/demo` is absent.
- Fresh live Axe 4.10.2 scans found zero violations at desktop and 390px. Normal
  routes logged no console errors; 404 did. No horizontal overflow was found.
- The neo-brutalist evidence-desk visual system is product-specific and matches
  `.factory/design.md`; it is not a generic SaaS template.
- Root/privacy/terms have `lang=en`, one h1, and main. First-load JS is 3.92 kB
  uncompressed (1.79 kB gzip). Existing favicon and legal pages work.

## Commands/results

|Check|Result|
|---|---|
|Claims file / listed tests|**FAIL:** missing / zero commands, all claims untested|
|`npm test`|PASS: 3 Rust unit, 4 integration, clippy, build, 8 Playwright|
|Live offline reload desktop + 390px|**FAIL:** no registration/controller; disconnected|
|`sentinel demo`; `sentinel --demo` in temp dir|**FAIL:** both exit 2|
|Unreadable discovery root as UID 65534|**FAIL:** scan 0/empty; export 10|
|`cargo package --locked --allow-dirty --list`|Command PASS; contents FAIL LICENSE/CHANGELOG|
|Live request log|Normal routes PASS; demo/404 FAIL third-party requests|
|Existing-link crawl; live Axe|PASS|

## What would make this perfect

Resolve every finding, deploy, then repeat this full review. Acceptance requires
a real temp-directory sample demo, a complete claims registry with passing
tagged tests, honest install availability, fixed offline/404/discovery/package
behavior, complete route metadata/focus/shell/touch targets, and one plain
vocabulary. The target is zero findings and zero untested claims.
