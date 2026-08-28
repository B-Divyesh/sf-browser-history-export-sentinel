# History Export Sentinel — review 1 handoff

## Outcome

Adversarial first-read review 1 completed against candidate
`1ea21df25b5d189d69de43b0d63c44bd59671f7c` and the live deployment on
2026-08-28 UTC. Verdict: **FAIL**. Full evidence, copy/claim inventories,
carried-forward checks, and concrete fixes are in `.factory/review-1.md`.
No product code was modified.

## Verification performed

- Fresh Chromium at 390×844 and 1440×900: cold first screen, metadata, links,
  requests/console, Axe, focus/back, offline reload, and touch targets.
- `/demo`, `/?demo=1`, legal pages, and unknown routes.
- Both possible CLI demo commands in a new temporary directory.
- Unreadable Firefox discovery as UID 65534.
- `npm ci`, `npm test`, and `cargo package --locked --allow-dirty --list`.
- Full review of the earlier handoff/verification plus brief, design, source,
  tests, site, legal pages, and READMEs.

`npm test` passed: 3 Rust unit tests, 4 integration tests, clippy, site build,
and 8 Playwright tests. This does not satisfy claims verification because
`.factory/claims.json`, a real demo entry, and `@claim:*` tests are absent.

## Blocking state

The site lacks a one-click isolated demo; public claims are unregistered;
offline reload fails; unknown routes load a third-party Azure 404; discovery
hides permission failures; mobile links miss 44px; hero caching is unsafe; and
the crate omits LICENSE/CHANGELOG. README also advertises release binaries when
GitHub reports no releases.

Repair every finding in `.factory/review-1.md`, deploy, and rerun the whole
checklist from clean contexts. There are no reviewer product-code changes to
preserve or revert.
