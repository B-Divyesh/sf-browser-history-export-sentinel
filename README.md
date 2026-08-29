# History Export Sentinel

History Export Sentinel is a local CLI for people moving or archiving Firefox
and Chromium history. Sentinel copies each history database before reading it.
It exports JSON or CSV. It records counts, the date range, and file hashes for
later checks.

The CLI does not upload history or modify an installed browser profile. An
empty history database stops with an error instead of creating an empty export.

## Try the sample

The sample command needs no browser profile or account. It creates two sample
profiles in a private temporary folder. It exports and verifies six visits,
then prints the output folder.

```sh
cargo run --manifest-path cli/Cargo.toml -- demo
```

After building and previewing the site, open `/?demo=1` to enter the sample-data
demo directly. See [`.factory/demo.md`](.factory/demo.md) for its isolation rules.

## Install from source

Install with a current stable Rust toolchain:

```sh
cargo install --path cli
sentinel --help
```

The project version is `0.1.0` and the license is MIT.

## Use the CLI

Find supported profiles:

```sh
sentinel scan
sentinel --json scan
```

Export all detected profiles or choose one profile. The output folder must be
new, so existing files cannot be overwritten.

```sh
sentinel export --output ./history-archive --format both
sentinel export --profile /path/to/profile --output ./firefox-archive
```

Check the exported files against each verification report:

```sh
sentinel verify ./history-archive
sentinel --json verify ./history-archive
```

JSON records use this format:

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

`--json` makes command results readable by scripts. It does not change the
exported files. The CLI has no network or telemetry code.

### Error codes

| Code | Meaning |
| ---: | --- |
| 0 | The command succeeded |
| 2 | The command or option is invalid |
| 10 | No usable profile or history record was found |
| 11 | A profile or database could not be read or copied |
| 12 | The database or exported data has an unsupported format |
| 13 | A verification report or exported file does not match |
| 1 | Another input or output operation failed |

Sentinel reads history URLs, titles, visit times, and visit counts. It does not
read passwords, cookies, page bodies, or encrypted browser data.

## Develop and verify

Install stable Rust, Node.js 20 or newer, and npm. Then run:

```sh
npm ci
npm test
npm run check
npm run build
cargo package --manifest-path cli/Cargo.toml --locked
```

`npm run build` writes the static site to `dist/site/`.

## Privacy

The site has no analytics, cookies, third-party scripts, or remote fonts. The
service worker caches its public pages for offline reading.

Read the [privacy policy](https://browser-history-export-sentinel.sociobot.in/privacy/)
and [terms](https://browser-history-export-sentinel.sociobot.in/terms/).

## License

MIT. See [LICENSE](LICENSE).
