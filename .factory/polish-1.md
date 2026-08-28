# Polish round 1 — finding closure map

Candidate repaired: `94d45272cfd68041eadff7acf452dcc80f4c3332`.
Evidence aliases: **CLI claims** = `scripts/claim-tests.mjs`; **site suite** =
`site/tests/site.spec.ts`; **live** =
<https://browser-history-export-sentinel.sociobot.in>. Screenshots are under
`.factory/evidence/`.

| Finding | Change made | Evidence |
| --- | --- | --- |
| F-1-1 | Replaced the headline and audience sentence with the review’s exact plain wording. | `first screen states…`; `home-desktop.png`; live `/` |
| F-1-2 | Added bundled samples, `sentinel demo`, `/demo/`, `?demo=1`, banner, reset, exit, and namespaced state. | `@claim:demo-isolation`; `@claim:demo-sandbox`; `demo-mobile.png`; live `/demo/` |
| F-1-3 | Added `.factory/claims.json` with one tagged observable test per retained claim. | Clean-clone claim loop in handoff |
| F-1-4 | Rewrote the long claim and proved export, empty failure, counts, ranges, and hashes. | `@claim:end-to-end-export`; `@claim:actionable-errors` |
| F-1-5 | Removed the numeric network slogan; retained privacy wording is tested. | `@claim:cli-privacy` |
| F-1-6 | Replaced the numeric slogan with named browser families. | `@claim:end-to-end-export` |
| F-1-7 | Replaced “artifact” wording and recomputes every exported-file digest. | `@claim:end-to-end-export` |
| F-1-8 | Rewrote as “Copy the database” and tests copied source stability. | `@claim:copy-safety` |
| F-1-9 | Rewrote as “Export its records” and tests the produced records. | `@claim:end-to-end-export` |
| F-1-10 | Rewrote as “Verify the exported files” and tests verification after export. | `@claim:verification` |
| F-1-11 | Retained the local-history fact with source/dependency and isolated execution proof. | `@claim:cli-privacy` |
| F-1-12 | Retained “no account” only in tested privacy copy. | `@claim:cli-privacy` |
| F-1-13 | Removed the vague “No cloud” fragment. | Copy audit; `@claim:cli-privacy` |
| F-1-14 | Replaced slogan with exact no-telemetry wording. | `@claim:cli-privacy` |
| F-1-15 | Rewrote as an instruction to keep and rerun the report. | Copy audit; `@claim:verification` |
| F-1-16 | Named supported families and systems in short copy. | `@claim:profile-discovery` |
| F-1-17 | Retained copy and side-file behavior with private-workspace proof. | `@claim:copy-safety` |
| F-1-18 | Removed SQLite implementation jargon from marketing; source safety remains enforced. | `@claim:copy-safety` |
| F-1-19 | Uses “file hash, record count, date range” and verifies all three. | `@claim:end-to-end-export`; `@claim:verification` |
| F-1-20 | Error test asserts code 13 and the exact changed file path. | `@claim:verification` |
| F-1-21 | Replaced the recording with a real sample and reserved domains. | `@claim:sample-domains` |
| F-1-22 | Replaced the browser/CLI comparison with tested local behavior. | `@claim:cli-privacy`; `@claim:demo-isolation` |
| F-1-23 | Retained path-omission copy and scans generated reports. | `@claim:end-to-end-export` |
| F-1-24 | Retained price/license/version facts under one package test. | `@claim:license-version-package` |
| F-1-25 | Changed install copy to honest source-only instructions. | `cargo package`; clean-clone install evidence |
| F-1-26 | Removed future factory/release wording. | Copy audit |
| F-1-27 | Rewrote as “does not archive web pages.” | `@claim:history-fields-only` |
| F-1-28 | Rewrote as “exports history records only.” | `@claim:history-fields-only` |
| F-1-29 | Replaced encryption-bypass jargon with exact excluded fields. | `@claim:history-fields-only` |
| F-1-30 | Exact record keys are asserted for every sample row. | `@claim:history-fields-only` |
| F-1-31 | Plain error copy now names conditions, code, and next step. | `@claim:actionable-errors` |
| F-1-32 | README keeps the local audience statement and isolated execution proof. | `@claim:cli-privacy` |
| F-1-33 | Split the 33-word README claim into four short sentences. | Copy audit; `@claim:end-to-end-export` |
| F-1-34 | Source safety and empty-export failure are observable. | `@claim:copy-safety`; `@claim:actionable-errors` |
| F-1-35 | Removed the false Releases instruction and states source-only availability. | README; live `/#install` |
| F-1-36 | Version 0.1.0 is checked from package metadata. | `@claim:license-version-package` |
| F-1-37 | Removed factory registry-process copy. | Copy audit |
| F-1-38 | Discovery list is covered by all browser/OS layout fixtures. | `@claim:profile-discovery` |
| F-1-39 | README documents automatic and selected-profile export. | Rust integration suite; `@claim:end-to-end-export` |
| F-1-40 | Existing output contents are proved unchanged. | `@claim:no-overwrite` |
| F-1-41 | The demo asserts one JSON, CSV, and report per profile. | `@claim:end-to-end-export` |
| F-1-42 | README verify example is run against intact and changed exports. | `@claim:json-mode`; `@claim:verification` |
| F-1-43 | Replaced “stable shape” with an exact record example and field test. | `@claim:history-fields-only` |
| F-1-44 | Parses demo and verify JSON stdout. | `@claim:json-mode` |
| F-1-45 | Runs JSON control output against the same unchanged exported files. | `@claim:json-mode` |
| F-1-46 | Removed “hidden retries”; retained no-prompt/network wording under privacy test. | `@claim:cli-privacy`; CLI help/tests |
| F-1-47 | Password and encrypted-data exclusion uses plain words. | `@claim:history-fields-only` |
| F-1-48 | Exact six-key history record shape is enforced. | `@claim:history-fields-only` |
| F-1-49 | `npm test` now visibly runs Rust, claims, build, and Playwright. | Full-suite transcript in handoff |
| F-1-50 | Removed internal work-order wording; build output is asserted. | `@claim:site-build` |
| F-1-51 | Deployment statement remains only as the documented site URL and is cold-checked. | live `/`; handoff hashes |
| F-1-52 | Every route, including demo and 404, has same-origin request and cookie checks. | `@claim:site-privacy` |
| F-1-53 | CLI local-output behavior is exercised under a fresh HOME/TMPDIR. | `@claim:demo-isolation`; `@claim:cli-privacy` |
| F-1-54 | Repository and crate MIT materials are present. | `@claim:license-version-package` |
| F-1-55 | Privacy copy now says the CLI processes history on the computer. | `@claim:cli-privacy` |
| F-1-56 | Runtime source/dependencies are scanned; isolated demo completes. | `@claim:cli-privacy` |
| F-1-57 | Active WAL/side-file copying is covered. | `@claim:copy-safety` |
| F-1-58 | Output schema allows only URLs, titles, times, counts, browser, and profile. | `@claim:history-fields-only` |
| F-1-59 | Temporary sample and snapshot folders are asserted removed. | `@claim:demo-isolation`; `@claim:copy-safety` |
| F-1-60 | Demo persistent writes are limited to its explicit output. | `@claim:demo-isolation` |
| F-1-61 | Reports are scanned for temporary/home absolute paths. | `@claim:end-to-end-export` |
| F-1-62 | Site privacy covers home, demo, legal pages, and first-party 404. | `@claim:site-privacy` |
| F-1-63 | Deployment-only config is excluded and demo reloads under forced offline mode. | `@claim:offline-reload`; `@claim:site-build` |
| F-1-64 | Removed the unprovable “anonymous” sentence. | Privacy page; copy audit |
| F-1-65 | MIT status is checked in repository and packaged crate metadata. | `@claim:license-version-package` |
| F-1-66 | Excluded fields and absence of network code cover encryption/upload behavior. | `@claim:history-fields-only`; `@claim:cli-privacy` |
| F-1-67 | Terms use narrow verification wording; changed-file behavior is tested. | `@claim:verification` |
| F-1-68 | Changelog ships inside the package with the current version. | `@claim:license-version-package` |
| F-1-69 | CLI README replaces jargon and is covered by end-to-end demo proof. | `@claim:end-to-end-export` |
| F-1-70 | Distinct empty/format/mismatch/permission codes are exercised. | `@claim:actionable-errors`; `@claim:verification` |
| F-1-71 | CLI privacy and source-copy tests replace the compound unproved sentence. | `@claim:cli-privacy`; `@claim:copy-safety` |
| F-1-72 | CLI README links directly to commands and the error-code table. | Link crawl; README link |
| F-1-73 | Service worker excludes deployment config and offline reload passes. | `@claim:offline-reload`; live offline check |
| F-1-74 | Added styled first-party `404.html`, response override, and normal CSP. | `404-desktop.png`; live unknown-route check |
| F-1-75 | Discovery returns exit 11 for unreadable roots instead of flattening errors. | `@claim:actionable-errors` as UID 65534 |
| F-1-76 | All visible mobile links/buttons now measure at least 44×44. | `mobile layout has no overflow…`; `demo-mobile.png` |
| F-1-77 | Hero now uses a content-fingerprinted URL; OG/touch derivatives are documented. | build config; `.factory/design.md` |
| F-1-78 | Crate now contains `LICENSE`, `CHANGELOG.md`, and its bundled sample. | `cargo package --list`; `@claim:license-version-package` |
| F-1-79 | Home title now names browser history exports. | metadata route test; live `/` |
| F-1-80 | Added canonical, OG, Twitter, touch icon, and original 1200×630 art on every route. | metadata route test; `identify` evidence in handoff |
| F-1-81 | History API routing restores routes and focuses/announces each h1. | `internal routing updates history…` |
| F-1-82 | Home, demo, legal, and 404 share Home/Demo/Privacy/Install and full footer/build ID. | shell route test; screenshots |
| F-1-83 | Sitemap lists home, demo, privacy, terms, and 404. | `site/public/sitemap.xml`; live `/sitemap.xml` |
| F-1-84 | Replaced the 26-word jargon sentence with the requested 12-word sentence. | first-screen test; copy audit |
| F-1-85 | README sentence is split into short concrete statements. | `.factory/copy-audit.md` |
| F-1-86 | Heading is now “How Sentinel verifies an export.” | live `/`; `home-desktop.png` |
| F-1-87 | Heading is now “See successful and failed exports.” | live `/#sample` |
| F-1-88 | Heading is now “What the verification report records.” | live `/`; screenshot |
| F-1-89 | Heading is now “Install Sentinel from source.” | live `/#install` |
| F-1-90 | Footer one-liner states JSON/CSV output. | shell route test |
| F-1-91 | Named Windows, macOS, and Linux replaces “cross-platform.” | `@claim:profile-discovery`; copy audit |
| F-1-92 | “copies the database before reading it” replaces “copy-first.” | copy audit; `@claim:copy-safety` |
| F-1-93 | Label is “Copy, export, and verify.” | live `/`; copy audit |
| F-1-94 | “sample data” replaces fixture/recording terms. | copy audit; demo pages |
| F-1-95 | Public copy uses “exported file.” | copy audit |
| F-1-96 | Error sentence uses the review’s plain condition/next-step structure. | copy audit; `@claim:actionable-errors` |
| F-1-97 | Terminology table enforces sample-data demo, date range, verification report, and exported file. | `.factory/copy-audit.md` |
| F-1-98 | Honest source-only sentence replaces internal future release copy. | live `/#install`; README |

## Final result

All 98 findings have an implemented change and repeatable evidence. Final local,
clean-clone, deployment, and cold-live results are recorded in
`.factory/handoff.md`.
