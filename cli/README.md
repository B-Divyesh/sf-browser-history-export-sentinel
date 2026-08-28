# History Export Sentinel CLI

A local Firefox and Chromium history exporter. It copies each database before
reading it and writes JSON, CSV, and a verification report.

```sh
cargo install --git https://github.com/B-Divyesh/sf-browser-history-export-sentinel history-export-sentinel
sentinel demo
sentinel scan
sentinel export --output ./history-archive --format both
sentinel verify ./history-archive
```

Empty history, permission errors, unsupported database formats, and changed
exports return specific error codes. The CLI has no telemetry or network client.

See the [full commands and error codes](https://github.com/B-Divyesh/sf-browser-history-export-sentinel#use-the-cli).
