# History Export Sentinel — independent verification

**Verdict: FAIL**

- Work order: `browser-history-export-sentinel-verify-2`
- Candidate: `8236468450bd938d6627a52892a1fb9ecb9ded58`
- Candidate branch: `main`; `origin/main` resolved to the same commit before testing
- Live URL: <https://browser-history-export-sentinel.sociobot.in>
- Verified: 2026-08-28 UTC
- Environment: Debian/Linux container, Node 22.23.2, npm 10.9.8,
  rustc/cargo 1.98.0, Playwright 1.58.2 Chromium 145

The candidate's core CLI works end to end and the live site's principal files
exactly match the candidate build. Release acceptance nevertheless fails because
the advertised service worker cannot install in production, so offline reload
does not work. The deployed error route also violates the site's no-third-party-
scripts claim. Additional permission-discovery and mobile target-size defects are
listed below.

## Defects

### High — V1: production service worker never installs; offline reload fails

The generated `sw.js` precaches `/staticwebapp.config.json`. Azure consumes that
deployment file instead of serving it, so the request returns 404. Because the
install handler uses one `cache.addAll(PRECACHE)`, that rejection aborts service
worker installation.

Fresh-browser evidence on both desktop and 390×844 contexts:

- `GET /sw.js` returned 200 and contains `/staticwebapp.config.json` in
  `PRECACHE`.
- `GET /staticwebapp.config.json` returned 404.
- After registration and an explicit update check,
  `navigator.serviceWorker.getRegistrations()` was empty,
  `navigator.serviceWorker.controller` was null, and only an orphaned
  `sentinel-70d4be64e0dc` cache remained.
- Reloading with the browser offline failed with
  `net::ERR_INTERNET_DISCONNECTED` on desktop and mobile.

Expected: the documented service worker installs, updates, controls the page,
and reloads the precached site offline. Actual: it never reaches an active state.
This contradicts the privacy page's statement that a service worker caches the
public site for offline reading and the builder handoff's offline claim.

### High — V2: the production 404 sends visitors to third-party scripts

`GET /definitely-not-a-real-route-qa` returned the Azure Static Web Apps default
404 rather than a product error page. It omitted the project's CSP, HSTS,
referrer policy, permissions policy, and `nosniff` headers. Browser observation
recorded requests to:

- `ajax.aspnetcdn.com` for Bootstrap CSS, jQuery, and Bootstrap JavaScript;
- `appservice.azureedge.net` for CSS, JavaScript, and two images.

The page also logged a console resource error. This violates the explicit
privacy statement that the static documentation site uses no third-party
scripts, as well as the contract's error-state and response-policy requirements.
Normal product routes made no external requests.

### Medium — V3: auto-discovery suppresses OS permission failures

With a real Firefox fixture below an unreadable `.mozilla/firefox` directory,
running as UID 65534 produced:

```text
sentinel --json scan --home <restricted-home>
exit 0, {"status":"empty","profile_count":0,"profiles":[]}

sentinel export --home <restricted-home> --output <new-output>
exit 10, sentinel: no supported browser profiles found
```

The discovery walker discards `read_dir` errors, so a permission problem is
misreported as an empty machine rather than the actionable exit 11 promised by
the CLI contract. Supplying that profile explicitly correctly returned 11,
mentioned permission repair, and left no partial archive. The explicit path is
a workaround, but users relying on the core detection flow can be misled.

### Medium — V4: several mobile navigation targets are below 44×44 CSS px

At the required 390px viewport, measured visible controls included:

- header “Demo”: 29×19px;
- privacy rail link: 222×25px;
- footer “Privacy”, “Terms”, and “Source”: 59×22, 42×22, and 51×22px.

Primary buttons and demo tabs are large enough, and there is no horizontal
overflow. These navigation links still miss the attached accessibility/design
contract's 44×44 minimum touch target.

### Low — V5: an unfingerprinted image is cached as immutable for one year

`/evidence-desk.webp` has `cache-control: public, max-age=31536000, immutable`
but its URL has no content hash. A future image replacement at the same path can
remain stale for returning users. Hashed JS/CSS correctly use immutable caching;
HTML uses 30-second revalidation and `sw.js` uses `no-cache`.

### Low — V6: the publishable crate omits repository legal/change files

`cargo package --list` contains seven files but not the repository `LICENSE` or
`CHANGELOG.md`. The crate metadata does declare `license = "MIT"`, and both files
exist in the repository, so this does not affect runtime behavior; it is a
packaging-readiness gap against the attached library/CLI contract.

## Clean checkout and repository gates

The starting worktree was clean at the candidate commit. No product source was
changed during verification.

| Check | Result |
| --- | --- |
| `npm ci` | PASS; 24 packages installed, 0 vulnerabilities |
| `npm test` | PASS; 3 Rust unit + 4 Rust integration + 8 Playwright tests |
| `npm run check` | PASS; TypeScript no-emit check |
| `npm run build` | PASS; exact production alias generated `dist/site/` |
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS (inside `npm test`) |
| `cargo build --release --locked` | PASS; binary 2.8 MB |
| `cargo package --manifest-path cli/Cargo.toml --locked --allow-dirty` | PASS; 17.0 KiB crate, package verification compiled |
| `npm audit --audit-level=high` | PASS; 0 vulnerabilities |

Production output was 3,919 bytes JS, 13,015 bytes CSS, 113,772 bytes hero
WebP, and no font payload. All static budgets pass.

## Clean-consumer CLI exercise

The `.crate` was installed with `cargo install --path <unpacked-package>
--locked --root <clean-root>`. The installed `sentinel 0.1.0`, not the workspace
binary, was used for the main end-to-end matrix.

| Case | Result |
| --- | --- |
| `--help`, `--version`, no command, invalid `--format xml` | Helpful output; invalid/no command exit 2 |
| Detect Firefox + Chrome in a synthetic Linux home | PASS; deterministic JSON result, two profiles |
| Export both profiles to JSON + CSV | PASS; 5 visits total, correct browser/profile reports |
| Unicode URL, comma/quote/newline title, null title | PASS in JSON and RFC 4180 CSV round-trip |
| Firefox/Chromium date bounds including Chromium epoch | PASS; exact report bounds |
| Verify root and one profile report | PASS; hashes, counts, and bounds checked |
| Source databases before/after export | PASS; SHA-256 unchanged |
| Active Chromium WAL/SHM with committed row only in WAL | PASS; copied snapshot exported and verified 1 record |
| Two simultaneous exports to distinct outputs | PASS; both verified |
| Empty history | Exit 10, actionable message, no output directory |
| Malformed SQLite schema | Exit 12, no output directory |
| Profile directory without database | Exit 10, no output directory |
| Missing explicit profile | Exit 11, no output directory |
| Unreadable explicit database as unprivileged UID | Exit 11, actionable message, no partial output |
| 100 MB source with continuously changing metadata | Three retries, exit 11, close-browser guidance, no partial output |
| Existing destination | Exit 1; existing archive retained and remained verifiable |
| Changed artifact, missing artifact, altered report count | Each exit 13 with the affected condition/path |
| `../history.json` injected into report | Exit 13 as unsafe artifact path |
| Verify directory without report | Exit 13 |

Runtime source/dependency inspection found no HTTP client or socket use in the
CLI. Its non-development dependency tree is limited to CLI parsing, SQLite,
serialization/CSV, time, and SHA-256. No telemetry or network path was found.

The container executed the binary on Linux. The repository test suite passed
its synthetic discovery-layout checks for Linux, macOS, and Windows, but this
verification did not have native Windows/macOS hosts.

## Live identity, browser, accessibility, privacy, and policy evidence

Candidate/live SHA-256 values matched byte-for-byte:

| Resource | SHA-256 |
| --- | --- |
| `/` | `03acc6c45c1a5a1c4d517f09a5199deec1f070da1c3ec2a7e0a946119cb17879` |
| `/privacy/` | `873d4b4f18d17d0e78a8e81b6f4a58fb9dd621c943c9c1f51fceab05ddf1527f` |
| `/terms/` | `6a3eefb567f02be5782495aaf24f46b6e96464e0e9784e0d28d6eadae30872dc` |
| JS | `80d469df5f29862e567c96379b37783707bf50362332ce211599a5d1032895c0` |
| CSS | `4b28595d88c5901068824ded0f9d75065b11a5a47f73d4e96dab438d7a288f4b` |
| hero WebP | `e6b1edca9be02d3c521aa2caf0a84ff0887be2a9a590152e9470767614bc1ae5` |
| `sw.js` | `840dd13203735c16da83d758b33c2500cafe2c370a14f4fa594b383605fb0a5a` |

Desktop 1280×900 and mobile 390×844 visual inspection found a coherent,
product-specific layout with no clipping or horizontal overflow. At a 640px
layout width (the reflow equivalent of 200% desktop zoom), overflow remained 0
and all main text remained present.

- Factory `verify-url.sh`: PASS; HTTP 200, load 659 ms, title/lang/main/one h1,
  no missing image alt, no unlabeled buttons, no console errors.
- Axe 4.11.4 on `/`, `/privacy/`, and `/terms/`, desktop and mobile: 0
  violations, therefore 0 serious/critical findings.
- Keyboard sequence reached every normally tabbable control without a trap.
  The skip link became visible with a 3px acid focus ring and targeted `#main`;
  tab-list Arrow/Home/End operation worked; focused tabs used the same visible
  ring. Copy worked via Enter on desktop and announced success; denied clipboard
  access on mobile announced the manual-copy recovery.
- Reduced-motion emulation matched, set smooth scrolling to `auto`, and reduced
  transition duration to 0.01 ms.
- Main route requests were same-origin only. There were no cookies and both
  local/session storage were empty; the expected Cache Storage entry is the
  only local persistence.
- Normal 200 responses had CSP, HSTS, `no-referrer`, `nosniff`, and restrictive
  camera/microphone/geolocation policy. Conditional ETag requests returned 304;
  Brotli was used for compressible HTML/JS/CSS/SW responses.

Lighthouse 13.0.1 mobile against the live URL:

| Category/metric | Result |
| --- | ---: |
| Performance | 94 |
| Accessibility | 100 |
| Best Practices | 100 |
| SEO | 100 |
| FCP | 1.28 s |
| LCP | 1.83 s |
| CLS | 0 |
| TBT | 265 ms |
| Total transfer | 123,819 bytes (121 KiB) |

INP is not produced by a no-interaction Lighthouse lab navigation. Lighthouse
also emitted an unscored/experimental label-content-name diagnostic for the
wordmark's decorative `H//S`; default Axe 4.11.4 reported no violation and the
final Lighthouse accessibility score remained 100.

## Required disposition

Do not promote this candidate as PASS. Exclude deployment-only configuration
files from service-worker precaching (and verify a clean offline reload), add a
first-party 404 with the normal policies, propagate discovery permission errors,
and enlarge mobile navigation targets. Rebuild, redeploy, then rerun live PWA,
privacy/outbound-request, and accessibility verification.
