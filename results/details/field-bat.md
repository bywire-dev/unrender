# field/bat

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **94%** (chrome-excluded; 799 non-blank content cells)

## Sizes

| encoding | bytes | tokens | vs plain (bytes) | vs plain (tokens) |
|---|---:|---:|---:|---:|
| raw | 9413 | 5286 | 0.15x | 0.10x |
| plain | 1411 | 548 | 1.00x | 1.00x |
| compact | 3006 | 1500 | 0.47x | 0.37x |
| toon | 1382 | 520 | 1.02x | 1.05x |
| json | 17946 | 4688 | 0.08x | 0.12x |

<details><summary>Before — raw ANSI (9413 bytes, escapes shown literally)</summary>

```
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m   2\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;117;113;94m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m//! tests use for golden files) back into a character grid, then re-emits
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m    \x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;117;113;94m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m it
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m   3\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;117;113;94m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m//! as ANSI — the SVG equivalent of `ansi.rs`'s `Buffer -> ANSI` writer.
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m   4\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;117;113;94m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m//!
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m   5\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;117;113;94m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m//! Confirmed by hand against five varied real snapshots (table, tree,
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m   6\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;117;113;94m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m//! progress bar, list, log) before trusting this broadly, per the plan:
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m    \x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;117;113;94m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mthe
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m   7\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;117;113;94m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m//! format looks fixed at a glance but Rich mangles the `terminal-*` id
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m   8\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;117;113;94m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m//! prefix into `terminal-<random-u32>-*` on some renders (apparently to
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m   9\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;117;113;94m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m//! avoid id collisions when multiple SVGs land in one HTML page), which
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m    \x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;117;113;94m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23ma
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  10\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;117;113;94m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m//! parser hardcoded to the literal `terminal-` prefix silently reads as
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  11\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;117;113;94m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m//! empty. Every id is matched relative to a prefix captured from the fil
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m    \x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;117;113;94m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23me
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  12\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;117;113;94m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m//! itself, never assumed.
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  13\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  14\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;249;38;114m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23muse\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m std::collections::HashMap;
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  15\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  16\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;249;38;114m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23muse\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m anyhow::{Context, \x1b[38;2;166;226;46m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mResult\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m};
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  17\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;249;38;114m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23muse\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m regex::Regex;
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  18\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  19\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;249;38;114m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mpub\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m \x1b[38;2;102;217;239m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mtype\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m \x1b[38;2;166;226;46m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mRgb\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m \x1b[38;2;249;38;114m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m=\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m (\x1b[38;2;102;217;239m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mu8\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m, \x1b[38;2;102;217;239m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mu8\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m, \x1b[38;2;102;217;239m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mu8\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m);
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  20\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  21\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m#[derive(Clone, Debug, Default)]
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  22\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;102;217;239m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mstruct\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m \x1b[38;2;166;226;46m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mStyleClass\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m {
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  23\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m    \x1b[38;2;255;255;255m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mfg\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m: \x1b[38;2;166;226;46m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mOption\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m<Rgb>,
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  24\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m    \x1b[38;2;255;255;255m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mbold\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m: \x1b[38;2;102;217;239m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mbool\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m,
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  25\x1b[m \x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m \x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m    \x1b[38;2;255;255;255m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mitalic\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m: \x1b[38;2;102;217;239m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mbool\x1b[38;2;248;248;242m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m,
\x1b[38;2;131;148;150m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m─────┴──────────────────────────────────────────────────────────────────────────
\x1b[m
```

</details>

## Before (color-stripped)

```
   2 │ //! tests use for golden files) back into a character grid, then re-emits
     │  it
   3 │ //! as ANSI — the SVG equivalent of `ansi.rs`'s `Buffer -> ANSI` writer.
   4 │ //!
   5 │ //! Confirmed by hand against five varied real snapshots (table, tree,
   6 │ //! progress bar, list, log) before trusting this broadly, per the plan:
     │ the
   7 │ //! format looks fixed at a glance but Rich mangles the `terminal-*` id
   8 │ //! prefix into `terminal-<random-u32>-*` on some renders (apparently to
   9 │ //! avoid id collisions when multiple SVGs land in one HTML page), which
     │ a
  10 │ //! parser hardcoded to the literal `terminal-` prefix silently reads as
  11 │ //! empty. Every id is matched relative to a prefix captured from the fil
     │ e
  12 │ //! itself, never assumed.
  13 │
  14 │ use std::collections::HashMap;
  15 │
  16 │ use anyhow::{Context, Result};
  17 │ use regex::Regex;
  18 │
  19 │ pub type Rgb = (u8, u8, u8);
  20 │
  21 │ #[derive(Clone, Debug, Default)]
  22 │ struct StyleClass {
  23 │     fg: Option<Rgb>,
  24 │     bold: bool,
  25 │     italic: bool,
─────┴──────────────────────────────────────────────────────────────────────────
```

## After — compact

```
application @0,0,80,30
  table @0,0,80,29
    row @0,0,80,1
      cell @2,0,2,1 2
      cell @7,0,73,1 //! tests use for golden files) back into a character grid, then re-emits
    row @0,1,80,1
      cell @2,1,2,1
      cell @7,1,73,1 it
    row @0,2,80,1
      cell @2,2,2,1 3
      cell @7,2,73,1 //! as ANSI — the SVG equivalent of `ansi.rs`'s `Buffer -> ANSI` writer.
    row @0,3,80,1
      cell @2,3,2,1 4
      cell @7,3,73,1 //!
    row @0,4,80,1
      cell @2,4,2,1 5
      cell @7,4,73,1 //! Confirmed by hand against five varied real snapshots (table, tree,
    row @0,5,80,1
      cell @2,5,2,1 6
      cell @7,5,73,1 //! progress bar, list, log) before trusting this broadly, per the plan:
    row @0,6,80,1
      cell @2,6,2,1
      cell @7,6,73,1 the
    row @0,7,80,1
      cell @2,7,2,1 7
      cell @7,7,73,1 //! format looks fixed at a glance but Rich mangles the `terminal-*` id
    row @0,8,80,1
      cell @2,8,2,1 8
      cell @7,8,73,1 //! prefix into `terminal-<random-u32>-*` on some renders (apparently to
    row @0,9,80,1
      cell @2,9,2,1 9
      cell @7,9,73,1 //! avoid id collisions when multiple SVGs land in one HTML page), which
    row @0,10,80,1
      cell @2,10,2,1
      cell @7,10,73,1 a
    row @0,11,80,1
      cell @2,11,2,1 10
      cell @7,11,73,1 //! parser hardcoded to the literal `terminal-` prefix silently reads as
    row @0,12,80,1
      cell @2,12,2,1 11
      cell @7,12,73,1 //! empty. Every id is matched relative to a prefix captured from the fil
    row @0,13,80,1
      cell @2,13,2,1
      cell @7,13,73,1 e
    row @0,14,80,1
      cell @2,14,2,1 12
      cell @7,14,73,1 //! itself, never assumed.
    row @0,15,80,1
      cell @2,15,2,1 13
      cell @7,15,73,1
    row @0,16,80,1
      cell @2,16,2,1 14
      cell @7,16,73,1 use std::collections::HashMap;
    row @0,17,80,1
      cell @2,17,2,1 15
      cell @7,17,73,1
    row @0,18,80,1
      cell @2,18,2,1 16
      cell @7,18,73,1 use anyhow::{Context, Result};
    row @0,19,80,1
      cell @2,19,2,1 17
      cell @7,19,73,1 use regex::Regex;
    row @0,20,80,1
      cell @2,20,2,1 18
      cell @7,20,73,1
    row @0,21,80,1
      cell @2,21,2,1 19
      cell @7,21,73,1 pub type Rgb = (u8, u8, u8);
    row @0,22,80,1
      cell @2,22,2,1 20
      cell @7,22,73,1
    row @0,23,80,1
      cell @2,23,2,1 21
      cell @7,23,73,1 #[derive(Clone, Debug, Default)]
    row @0,24,80,1
      cell @2,24,2,1 22
      cell @7,24,73,1 struct StyleClass {
    row @0,25,80,1
      cell @2,25,2,1 23
      cell @7,25,73,1 fg: Option<Rgb>,
    row @0,26,80,1
      cell @2,26,2,1 24
      cell @7,26,73,1 bold: bool,
    row @0,27,80,1
      cell @2,27,2,1 25
      cell @7,27,73,1 italic: bool,
    row @0,28,80,1
      cell @2,28,2,1 ──
      cell @7,28,73,1 ─────────────────────────────────────────────────────────────────────────
```

## After — toon

```
application
  table [29]{c0,c1}:
     2,//! tests use for golden files) back into a character grid, then re-emits
     ,it
     3,//! as ANSI — the SVG equivalent of `ansi.rs`'s `Buffer -> ANSI` writer.
     4,//!
     5,//! Confirmed by hand against five varied real snapshots (table, tree,
     6,//! progress bar, list, log) before trusting this broadly, per the plan:
     ,the
     7,//! format looks fixed at a glance but Rich mangles the `terminal-*` id
     8,//! prefix into `terminal-<random-u32>-*` on some renders (apparently to
     9,//! avoid id collisions when multiple SVGs land in one HTML page), which
     ,a
     10,//! parser hardcoded to the literal `terminal-` prefix silently reads as
     11,//! empty. Every id is matched relative to a prefix captured from the fil
     ,e
     12,//! itself, never assumed.
     13,
     14,use std::collections::HashMap;
     15,
     16,use anyhow::{Context, Result};
     17,use regex::Regex;
     18,
     19,pub type Rgb = (u8, u8, u8);
     20,
     21,#[derive(Clone, Debug, Default)]
     22,struct StyleClass {
     23,fg: Option<Rgb>,
     24,bold: bool,
     25,italic: bool,
     ──,─────────────────────────────────────────────────────────────────────────
```

<details><summary>After — json</summary>

```json
{
  "role": "application",
  "rect": [
    0,
    0,
    80,
    30
  ],
  "children": [
    {
      "role": "table",
      "rect": [
        0,
        0,
        80,
        29
      ],
      "children": [
        {
          "role": "row",
          "rect": [
            0,
            0,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "2",
              "rect": [
                2,
                0,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "//! tests use for golden files) back into a character grid, then re-emits",
              "rect": [
                7,
                0,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            1,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "rect": [
                2,
                1,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "it",
              "rect": [
                7,
                1,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            2,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "3",
              "rect": [
                2,
                2,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "//! as ANSI — the SVG equivalent of `ansi.rs`'s `Buffer -> ANSI` writer.",
              "rect": [
                7,
                2,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            3,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "4",
              "rect": [
                2,
                3,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "//!",
              "rect": [
                7,
                3,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            4,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "5",
              "rect": [
                2,
                4,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "//! Confirmed by hand against five varied real snapshots (table, tree,",
              "rect": [
                7,
                4,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            5,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "6",
              "rect": [
                2,
                5,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "//! progress bar, list, log) before trusting this broadly, per the plan:",
              "rect": [
                7,
                5,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            6,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "rect": [
                2,
                6,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "the",
              "rect": [
                7,
                6,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            7,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "7",
              "rect": [
                2,
                7,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "//! format looks fixed at a glance but Rich mangles the `terminal-*` id",
              "rect": [
                7,
                7,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            8,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "8",
              "rect": [
                2,
                8,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "//! prefix into `terminal-<random-u32>-*` on some renders (apparently to",
              "rect": [
                7,
                8,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            9,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "9",
              "rect": [
                2,
                9,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "//! avoid id collisions when multiple SVGs land in one HTML page), which",
              "rect": [
                7,
                9,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            10,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "rect": [
                2,
                10,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "a",
              "rect": [
                7,
                10,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            11,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "10",
              "rect": [
                2,
                11,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "//! parser hardcoded to the literal `terminal-` prefix silently reads as",
              "rect": [
                7,
                11,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            12,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "11",
              "rect": [
                2,
                12,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "//! empty. Every id is matched relative to a prefix captured from the fil",
              "rect": [
                7,
                12,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            13,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "rect": [
                2,
                13,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "e",
              "rect": [
                7,
                13,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            14,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "12",
              "rect": [
                2,
                14,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "//! itself, never assumed.",
              "rect": [
                7,
                14,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            15,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "13",
              "rect": [
                2,
                15,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                7,
                15,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            16,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "14",
              "rect": [
                2,
                16,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "use std::collections::HashMap;",
              "rect": [
                7,
                16,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            17,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "15",
              "rect": [
                2,
                17,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                7,
                17,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            18,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "16",
              "rect": [
                2,
                18,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "use anyhow::{Context, Result};",
              "rect": [
                7,
                18,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            19,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "17",
              "rect": [
                2,
                19,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "use regex::Regex;",
              "rect": [
                7,
                19,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            20,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "18",
              "rect": [
                2,
                20,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                7,
                20,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            21,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "19",
              "rect": [
                2,
                21,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "pub type Rgb = (u8, u8, u8);",
              "rect": [
                7,
                21,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            22,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "20",
              "rect": [
                2,
                22,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                7,
                22,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            23,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "21",
              "rect": [
                2,
                23,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "#[derive(Clone, Debug, Default)]",
              "rect": [
                7,
                23,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            24,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "22",
              "rect": [
                2,
                24,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "struct StyleClass {",
              "rect": [
                7,
                24,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            25,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "23",
              "rect": [
                2,
                25,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "fg: Option<Rgb>,",
              "rect": [
                7,
                25,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            26,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "24",
              "rect": [
                2,
                26,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "bold: bool,",
              "rect": [
                7,
                26,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            27,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "25",
              "rect": [
                2,
                27,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "italic: bool,",
              "rect": [
                7,
                27,
                73,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            28,
            80,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "──",
              "rect": [
                2,
                28,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "─────────────────────────────────────────────────────────────────────────",
              "rect": [
                7,
                28,
                73,
                1
              ]
            }
          ]
        }
      ]
    }
  ]
}
```

</details>

