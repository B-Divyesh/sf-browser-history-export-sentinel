# History Export Sentinel CLI

An offline, copy-first Firefox and Chromium history exporter with normalized
JSON/CSV output and SHA-256 verification reports.

```sh
cargo install history-export-sentinel
sentinel scan
sentinel export --output ./history-archive --format both
sentinel verify ./history-archive
```

Empty history, permission failures, unsupported schemas, and verification
mismatches return distinct non-zero exit codes. The CLI has no telemetry or
network client and never opens a live profile database through SQLite.

Full documentation and the exit-code contract are available in the project
repository.
