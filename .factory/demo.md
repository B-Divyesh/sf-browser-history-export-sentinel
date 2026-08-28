# Sample-data demo

## Browser

Open <https://browser-history-export-sentinel.sociobot.in/demo/> or
`https://browser-history-export-sentinel.sociobot.in/?demo=1`. The landing page
links to `/demo/` with one click.

The demo shows three states from the CLI contract: verified, empty history, and
blocked profile. Its six visits use reserved `.example` domains. Browser state
uses only `localStorage["demo:sentinel:state"]`; no non-demo key is read or
written. **Reset demo** clears every `demo:` key and restores the verified
state. **Start for real** clears demo state and opens the source install step.

## CLI

Run:

```sh
sentinel demo
sentinel --json demo --output ./sample-export
```

The command reads bundled `examples/sample-history.json` data. It creates a
private temporary Firefox and Chromium workspace, exports JSON and CSV, writes
two verification reports, verifies them, and deletes the input workspace. The
output is kept at the printed path for inspection. It never scans the user's
home or reads an installed browser profile.

## Verification

`npm run test:claims -- --grep @claim:demo-isolation` runs the CLI with fresh
`HOME` and `TMPDIR` directories and checks isolation and cleanup.
`npm run test:site -- --project=chromium --grep @claim:demo-sandbox` checks the
browser namespace, reset action, and exit action in a fresh browser context.
