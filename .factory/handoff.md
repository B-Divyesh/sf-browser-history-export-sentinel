# Polish round 3 handoff — 2026-08-29 UTC

## Outcome

All findings from `.factory/review-1.md`, `review-2.md`, and `review-3.md` are
closed. The repaired static site and Rust CLI are deployed at
<https://browser-history-export-sentinel.sociobot.in>. Product commits are
`98efe56f995da51acd0e117105d90e7669fe2f73` and
`a6cbb2c6dbddee242d96604751494aef5e2b7279`.

The first screen now keeps the job, audience, sample action, and three facts
inside 1440×900. The action opens the isolated demo at `/?demo=1`. That URL
renders the dedicated sample document immediately, uses only `demo:` storage,
preserves real keys, and provides Reset demo and Start for real. Reset removes
the demo key and shows the verified state in memory.

Unregistered public claims were removed or mapped. Each browser claim builds
its own preview, so it no longer depends on an earlier build. Route titles,
metadata, canonical links, History API focus/announcements, legal links,
first-party 404 behavior, mobile targets, and product wording are covered by
the browser suite. The `Verification report` label replaces the unexplained
format label. The evidence-desk visual identity is unchanged.

During the cold deployed audit, the exact query demo exposed an offline cache
key edge case. The service worker now maps `/?demo=1` to the precached demo
document and fetches precache bodies with `cache: reload`; the local claim and
final live offline run both pass.

## Clean-clone evidence

Final clone: `/tmp/history-sentinel-polish3-final-TV1XdT/repo` at
`a6cbb2c6dbddee242d96604751494aef5e2b7279`, created with `git clone --no-local`
and prepared with `npm ci` (0 vulnerabilities).

All 16 commands in `.factory/claims.json` were executed separately in registry
order. All passed:

`demo-isolation`, `demo-sandbox`, `end-to-end-export`, `sample-domains`,
`history-fields-only`, `copy-safety`, `verification`, `profile-discovery`,
`actionable-errors`, `cli-privacy`, `no-overwrite`, `json-mode`,
`license-version-package`, `site-build`, `site-privacy`, and `offline-reload`.

The `demo-sandbox` command ran before any tracked or generated `dist/site` and
passed because Playwright built the site before previewing it. The final
`offline-reload` command used `/?demo=1`, asserted non-empty cache bodies, set
the browser offline, reloaded, and found the dedicated demo h1 and sample.

Full clean-clone gates:

- `npm test` — PASS: 4 Rust unit tests, 4 Rust integration tests, clippy, 13
  CLI claim checks, and 25 Playwright checks passed; one desktop invocation of
  the mobile-only size check was skipped.
- `npm run check` — PASS.
- `npm run build` — PASS; wrote `dist/site/`.
- `cargo fmt --all -- --check` — PASS.
- `cargo build --release --locked` — PASS.
- `cargo package --manifest-path cli/Cargo.toml --locked` — PASS; 10 files,
  77.2 KiB unpacked and 19.5 KiB compressed, including LICENSE and CHANGELOG.
- `npm audit --audit-level=high` — PASS; 0 vulnerabilities.

Production assets are below budget: JavaScript 7.18 kB raw / 2.80 kB gzip,
CSS 15.28 kB raw / 4.21 kB gzip, hero WebP 83,456 bytes, and no font payload.

## Deployment and cold-live evidence

The work-order command `npm ci && npm run build:site` completed, then
`/opt/fleet/lib/deploy-static.sh browser-history-export-sentinel dist/site`
reported deployment `ab76cc50-e6c9-42c5-911d-54b16b5a67d6` succeeded. The
custom domain returned 200 over managed TLS.

`npm run audit:live -- https://browser-history-export-sentinel.sociobot.in
.factory/evidence/live-polish-3` passed from fresh Chromium contexts:

- `/`, `/demo/`, `/privacy/`, and `/terms/` returned 200 with exact titles,
  h1s, metadata, legal links, no overflow, and 0 serious/critical Axe findings
  at 1440×900 and 390×844.
- The first-screen action and all three facts fit at 1440×900.
- Clicking the action reached `/?demo=1`; Reset left zero `demo:` keys and kept
  `real:sentinel:marker=keep`; Start for real discarded demo state.
- History navigation changed titles, focused/announced h1s, and restored focus
  on Back.
- The exact query demo reloaded offline with its six-visit sample.
- An unknown URL returned the designed product page with HTTP 404, CSP,
  `no-referrer`, and `nosniff`.
- All 26 observed requests were same-origin; both route contexts had zero
  cookies.

Factory `verify-url.sh` passed with a 618 ms load, no console errors, one h1,
one main, `lang=en`, no missing image alt, and no unlabeled button. Live/local
SHA-256 values match:

- `index.html`: `948ab08301c8bb9c4cb595afb2888d3b22f91fe5b46cd733fb278a7c38023bb5`
- `sw.js`: `67720b05602075e3e6be2acb88a9003331ecad1da5d93e39a66bc572590dc708`

Lighthouse 13.0.1 mobile on the live root scored 100 Performance, 100
Accessibility, 100 Best Practices, and 100 SEO. FCP was 0.9 s, LCP 1.3 s, CLS
0, TBT 0 ms, and total transfer 92 KiB.

Evidence files are in `.factory/evidence/live-polish-3/`: `live-audit.json`,
`verify.json`, `lighthouse.json`, `home-desktop.png`, `demo-query-mobile.png`,
`404-desktop.png`, and the factory verifier screenshots.

## Run and verify

```sh
npm ci
npm test
npm run check
npm run build
npm run audit:live -- https://browser-history-export-sentinel.sociobot.in .factory/evidence/live-polish-3
cargo run --manifest-path cli/Cargo.toml -- demo
```

## Known gaps and next steps

No acceptance gaps remain. The CLI intentionally remains a source-installed,
local, deterministic tool; publishing registry or signed release artifacts is
a factory release task, not part of this repository repair. No AI feature was
added because remote inference would not improve this sensitive local export
and verification job.
