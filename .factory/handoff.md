# Review 3 handoff — 2026-08-29 UTC

This review made no product-code changes. The complete report is
`.factory/review-3.md`; the verdict is **FAIL**.

Verified from fresh live Chromium contexts at 390×844 and 1440×900: cold first
screen, demo entry and Reset/exit storage behavior, real-storage isolation,
same-origin requests, cookies, offline reload, all routes and links, designed
404, metadata, History API focus/Back behavior, Axe, overflow, and visual
identity. The CLI demo also ran under fresh HOME/TMPDIR directories.

Clean clone `/tmp/history-sentinel-review3-clone-K6fa18` was at
`2368d614c5ba678111473a01fa39390c50f93174`. All 16 claim commands were run in
registry order. Fifteen passed; `demo-sandbox` failed because its standalone
Playwright command previews an absent `dist/site`. After a build, `npm test`
passed with 21 Playwright tests, 8 Rust tests, 13 CLI claim checks, and one
intentional skip; `npm run check` passed.

Outstanding blockers are the desktop sample action below the first fold, the
non-standalone demo claim test, the false Reset-clears-storage wording, and five
unlisted public claims. The unexplained `report.json / format 1` label is a
minor finding. No infrastructure, deployment, or product source was changed.
