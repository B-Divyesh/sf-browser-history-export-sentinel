# History Export Sentinel

History Export Sentinel is a private, local CLI for people moving or archiving
Firefox and Chromium history. It finds profiles, copies each SQLite database to
a temporary workspace, exports normalized JSON and/or CSV, and writes a report
with record counts, date bounds, and SHA-256 hashes that can be checked later.

It never uploads history, never modifies a live profile, and never treats an
empty export as success.

## Install

Download the single binary for your platform from Releases, or build it with a
current stable Rust toolchain:

```sh
cargo install --path cli
sentinel --help
```

The project starts at version `0.1.0`. Registry publishing is handled by the
Param Factory; maintainers can validate the package with `cargo package
--manifest-path cli/Cargo.toml`.

## Usage

List detected Firefox, Chrome, Chromium, Edge, Brave, and Vivaldi profiles:

```sh
sentinel scan
sentinel --json scan
```

Export every detected profile, or select a profile directory/database. The
destination must not already exist, which prevents accidental overwrites.

```sh
sentinel export --output ./history-archive --format both
sentinel export --profile "$HOME/.mozilla/firefox/abc.default-release" \
  --output ./firefox-archive --format json
```

Each profile gets a normalized export and `report.json`. Verify the report and
file hashes before changing machines or accounts:

```sh
sentinel verify ./history-archive
sentinel --json verify ./history-archive
```

The JSON records use this stable v1 shape:

```json
{
  "url": "https://example.test/",
  "title": "Example",
  "visited_at": "2026-08-28T10:30:00Z",
  "visit_count": 2,
  "browser": "firefox",
  "profile": "default-release"
}
```

`--json` makes command results machine-readable. It does not change the export
format. There are no prompts, network calls, telemetry, or hidden retries.

### Exit codes

| Code | Meaning |
| ---: | --- |
| 0 | Scan/export/verification succeeded |
| 2 | Invalid command-line usage (from `clap`) |
| 10 | No usable profile or no history records |
| 11 | Profile/database could not be read or copied; close the browser or fix permissions |
| 12 | Database schema or export data is unsupported/corrupt |
| 13 | Verification failed: missing, changed, or inconsistent artifacts |
| 1 | Other I/O failure |

Browser passwords and encrypted data are deliberately out of scope. Sentinel
only reads visit URLs, page titles, timestamps, and aggregate visit counts.

## Develop and test

Prerequisites: stable Rust, Node.js 20+, and npm.

```sh
npm ci
npm test
npm run build
```

`npm test` runs Rust unit/integration tests and site tests. `npm run build`
produces the static landing/docs site at `dist/site/`, exactly matching the
factory work order. Run `npm run dev` for the site and `cargo test
--manifest-path cli/Cargo.toml` for the CLI alone.

## Deployment and privacy

The static site is deployed at
https://browser-history-export-sentinel.sociobot.in. It has no analytics,
cookies, third-party scripts, or remote fonts. The CLI is offline-only; exports
remain wherever you place them. See the site's `/privacy/` and `/terms/` pages.

## License

MIT. See [LICENSE](LICENSE).
