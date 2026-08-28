# History Export Sentinel — visual thesis

## Direction

**Neo-brutalist utility: the evidence desk.** This is not a soft cloud app. It is
a local inspection tool used just before a consequential machine or account
change, so the page behaves like a forensic workbench: heavy rules, stamped
states, square corners, visible identifiers, and paper-like fields. Decoration
only explains the pipeline from browser database to independently checkable
artifacts.

The treatment is intentionally single-mode. A warm, explicitly painted paper
background keeps the visual metaphor consistent with a printed audit sheet and
avoids an automatic dark treatment changing the meaning of ink, warning stamps,
or status colors. It remains comfortable at night through low-luminance ink and
cream rather than pure black and white.

## Tokens

| Role | Token | Value | Reason |
| --- | --- | --- | --- |
| Background | `--paper` | `#F3ECDD` | aged export ledger, never clinical white |
| Surface | `--sheet` | `#FFFDF5` | readable report paper |
| Ink | `--ink` | `#171713` | near-black terminal ink |
| Muted ink | `--muted` | `#5B594F` | secondary copy, 6.1:1 on paper |
| Action | `--cobalt` | `#2447D8` | file-system blue, 6.3:1 with white |
| Signal | `--acid` | `#D9FF43` | a literal highlighter; always paired with ink |
| Success | `--ok` | `#16623A` | verification stamp, 6.3:1 on sheet |
| Warning | `--warn` | `#8A4B00` | amber ink, never color alone |
| Danger | `--danger` | `#B3261E` | failed check stamp, 6.4:1 on sheet |

Borders are 2px ink; emphasized panels use a hard 6px `--ink` offset shadow.
Corners stay at 0–2px. Status always includes a word or icon, never color alone.

## Type and spacing

- Display: `Arial Black`, `Arial Narrow Bold`, system sans-serif. Condensed,
  poster-like headings make the small utility feel decisive without a webfont.
- Working text: `ui-monospace`, `SFMono-Regular`, `Cascadia Code`, `Roboto Mono`,
  monospace. It makes hashes, paths, counts, and commands visually trustworthy.
- Scale: 14 / 16 / 20 / 28 / clamp(44–80) px, with body fixed at 16px minimum.
- Rhythm: 4px base; primary gaps 8, 16, 24, 32, 48, 64, 96px. Reading measure
  caps at 68 characters. Numeric data uses tabular figures.

## Interaction grammar

Primary controls look like labeled machine switches: cobalt fill, 2px ink
border, hard shadow, and a 2px press translation. Links use persistent underline
offsets. Focus is a 3px acid ring plus a 2px ink outer ring. The recorded demo
has three explicit state buttons—verified, empty, and locked—so error and empty
behavior are inspectable without pretending the browser can read local history.

On a phone, the nav collapses to only install and demo jumps, the evidence
rail stacks after the copy, report rows become label/value blocks, and purely
decorative annotation marks disappear. No critical content is removed.

## Motion policy

Only state changes move: panels enter 8px from their source over 180ms, buttons
press over 90ms, and report bars expand once when a demo state changes. There is
no looping motion. Under `prefers-reduced-motion: reduce`, transforms and smooth
scrolling are disabled and states switch with an opacity cross-fade under 80ms.

## Original asset plan and provenance

- `site/public/evidence-desk-e6b1edca.webp`: a content-fingerprinted 1200×630
  crop of the generated editorial still-life showing a
  local browser database becoming JSON/CSV evidence sheets through a physical
  verifier gate. No browser logos, UI text, people, clouds, or realistic history
  content. It supports the product's data-flow explanation instead of acting as
  generic atmosphere.
- Generation prompt: “Neo-brutalist editorial still life for a privacy utility:
  top-down evidence desk, one dark database cartridge on the left, two crisp
  punched paper exports on the right, a square verifier gate between them,
  visible abstract rows and checkmarks but no legible words, warm archival paper,
  cobalt blue, acid-lime highlighter, red registration marks, thick black ink,
  screen-printed halftone texture, hard offset shadows, wide 3:2 composition,
  no logos, no people, no browser branding, no gradients, no watermark.”
- Generator: Factory `factory-image` deployment via
  `/opt/fleet/lib/gen-image.sh`, 1536×1024 PNG source. Converted locally to WebP.
  The sidecar records the exact prompt and deployment. Product artwork is
  project-original and distributed under the repository MIT license.
- `site/public/og-history-sentinel.webp` and `apple-touch-icon.png` are local
  crops of that same original asset. ImageMagick 6 produced both derivatives;
  they add no third-party source material.
