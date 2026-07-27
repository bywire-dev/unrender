# field/lazygit

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **86%** (chrome-excluded; 582 non-blank content cells)

## Sizes

| encoding | bytes | tokens | vs plain (bytes) | vs plain (tokens) |
|---|---:|---:|---:|---:|
| raw | 7311 | 2933 | 0.56x | 0.32x |
| plain | 4073 | 939 | 1.00x | 1.00x |
| compact | 5917 | 3025 | 0.69x | 0.31x |
| toon | 2106 | 864 | 1.93x | 1.09x |
| json | 37413 | 9363 | 0.11x | 0.10x |

<details><summary>Before — raw ANSI (7311 bytes, escapes shown literally)</summary>

```
\x1b[m╭─[1]─Status────╮╭─[0]─Unstaged changes───────────────────╮╭─Staged changes────────────────────────╮
\x1b[m│unrender-field-││\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mdiff --git a/src/main.rs b/src/main.rs\x1b[m  ▐│\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mdiff --git a/src/lib.rs b/src/lib.rs\x1b[m   ▐
\x1b[m╰───────────────╯│\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mindex a9c3211..64218dc 100644\x1b[m           ▐│\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mnew file mode 100644\x1b[m                   ▐
\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m╭─[2]─Files \x1b[m- W\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m─╮\x1b[m│\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m--- a/src/main.rs\x1b[m                       ▐│\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mindex 0000000..a051497\x1b[m                 ▐
\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[38;5;11m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m▼\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m \x1b[38;5;11m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23msrc\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m          \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[m│\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m+++ b/src/main.rs\x1b[m                       ▐│\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m--- /dev/null\x1b[m                          ▐
\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[m  \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mA  lib.rs\x1b[m    \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[m│\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m@@ -3,3 +3,5 @@\x1b[m fn main() {             ▐│\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m+++ b/src/lib.rs\x1b[m                       ▐
\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[m   \x1b[31m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mM\x1b[m main.rs   \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[m│ }                                      ▐│\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m@@ -0,0 +1 @@\x1b[m                          ▐
\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[m               \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[m│                                        ▐│\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m+pub fn add(a: i32, b: i32) -> i32 { a\x1b[m ▐
\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[m               \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[m│ fn helper() {}                         ▐│\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m+ b }\x1b[m                                  ▐
\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[m               \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[m│\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m+\x1b[m                                       ▐│                                       ▐
\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m╰────────1 of 3─╯\x1b[m│\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m+// TODO: fix this\x1b[m                      ▐│                                       ▐
\x1b[m╭─[3]─\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mLocal bra\x1b[m─╮│                                        ▐│                                       ▐
\x1b[m│\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  *\x1b[m main       ││                                        ▐│                                       ▐
\x1b[m│               ││                                        ▐│                                       ▐
\x1b[m│               ││                                        ▐│                                       ▐
\x1b[m│               ││                                        ▐│                                       ▐
\x1b[m│               ││                                        ▐│                                       ▐
\x1b[m│               ││                                        ▐│                                       ▐
\x1b[m╰────────1 of 1─╯│                                        ▐│                                       ▐
\x1b[m╭─[4]─\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mCommits \x1b[m-─╮│                                        ││                                       ▐
\x1b[m│\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m88649d95\x1b[m \x1b[38;2;214;226;76m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mDe\x1b[m \x1b[38;2;214;226;76m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m○\x1b[m a││                                        ││                                       │
\x1b[m│\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m96658a62\x1b[m \x1b[38;2;214;226;76m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mDe\x1b[m \x1b[38;2;214;226;76m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m○\x1b[m a││                                        ││                                       │
\x1b[m│\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m8eb6eeea\x1b[m \x1b[38;2;214;226;76m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mDe\x1b[m \x1b[38;2;214;226;76m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m○\x1b[m i││                                        ││                                       │
\x1b[m│               ││                                        ││                                       │
\x1b[m│               ││                                        ││                                       │
\x1b[m╰────────1 of 3─╯╰────────────────────────────────────────╯╰───────────────────────────────────────╯
\x1b[m╭─[5]─Stash─────╮╭─Command log─────────────────────────────────────────────────────────────────────╮
\x1b[m│               ││\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mcan press 'M' in the files panel to open merge options\x1b[m                           │
\x1b[m╰────────0 of 0─╯╰─────────────────────────────────────────────────────────────────────────────────╯
\x1b[34m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mStage: <space> | Commit: c | Stash: s | Discard: d | Reset: D | …\x1b[m         \x1b[35m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[4m\x1b[22m\x1b[23mDonate\x1b[m \x1b[33m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[4m\x1b[22m\x1b[23mAsk Question\x1b[m 0.63.1\x1b[m
```

</details>

## Before (color-stripped)

```
╭─[1]─Status────╮╭─[0]─Unstaged changes───────────────────╮╭─Staged changes────────────────────────╮
│unrender-field-││diff --git a/src/main.rs b/src/main.rs  ▐│diff --git a/src/lib.rs b/src/lib.rs   ▐
╰───────────────╯│index a9c3211..64218dc 100644           ▐│new file mode 100644                   ▐
╭─[2]─Files - W─╮│--- a/src/main.rs                       ▐│index 0000000..a051497                 ▐
│▼ src          ││+++ b/src/main.rs                       ▐│--- /dev/null                          ▐
│  A  lib.rs    ││@@ -3,3 +3,5 @@ fn main() {             ▐│+++ b/src/lib.rs                       ▐
│   M main.rs   ││ }                                      ▐│@@ -0,0 +1 @@                          ▐
│               ││                                        ▐│+pub fn add(a: i32, b: i32) -> i32 { a ▐
│               ││ fn helper() {}                         ▐│+ b }                                  ▐
│               ││+                                       ▐│                                       ▐
╰────────1 of 3─╯│+// TODO: fix this                      ▐│                                       ▐
╭─[3]─Local bra─╮│                                        ▐│                                       ▐
│  * main       ││                                        ▐│                                       ▐
│               ││                                        ▐│                                       ▐
│               ││                                        ▐│                                       ▐
│               ││                                        ▐│                                       ▐
│               ││                                        ▐│                                       ▐
│               ││                                        ▐│                                       ▐
╰────────1 of 1─╯│                                        ▐│                                       ▐
╭─[4]─Commits -─╮│                                        ││                                       ▐
│88649d95 De ○ a││                                        ││                                       │
│96658a62 De ○ a││                                        ││                                       │
│8eb6eeea De ○ i││                                        ││                                       │
│               ││                                        ││                                       │
│               ││                                        ││                                       │
╰────────1 of 3─╯╰────────────────────────────────────────╯╰───────────────────────────────────────╯
╭─[5]─Stash─────╮╭─Command log─────────────────────────────────────────────────────────────────────╮
│               ││can press 'M' in the files panel to open merge options                           │
╰────────0 of 0─╯╰─────────────────────────────────────────────────────────────────────────────────╯
Stage: <space> | Commit: c | Stash: s | Discard: d | Reset: D | …         Donate Ask Question 0.63.1
```

## After — compact

```
application @0,0,100,30
  table @0,0,100,26
    row @0,0,100,1
      cell @0,0,17,1 ╭─[1]─Status────╮
      cell @18,0,38,1 ─[0]─Unstaged changes─────────────────
      cell @58,0,1,1 ╮
      cell @60,0,36,1 ─Staged changes─────────────────────
      cell @97,0,1,1 ─
      cell @99,0,1,1 ╮
    row @0,1,100,1
      cell @0,1,17,1 │unrender-field-│
      cell @18,1,38,1 diff --git a/src/main.rs b/src/main.rs
      cell @58,1,1,1 ▐
      cell @60,1,36,1 diff --git a/src/lib.rs b/src/lib.rs
      cell @97,1,1,1
      cell @99,1,1,1 ▐
    row @0,2,100,1
      cell @0,2,17,1 ╰───────────────╯
      cell @18,2,38,1 index a9c3211..64218dc 100644
      cell @58,2,1,1 ▐
      cell @60,2,36,1 new file mode 100644
      cell @97,2,1,1
      cell @99,2,1,1 ▐
    row @0,3,100,1
      cell @0,3,17,1 ╭─[2]─Files - W─╮
      cell @18,3,38,1 --- a/src/main.rs
      cell @58,3,1,1 ▐
      cell @60,3,36,1 index 0000000..a051497
      cell @97,3,1,1
      cell @99,3,1,1 ▐
    row [selected] @0,4,100,1
      cell @0,4,17,1 │▼ src          │
      cell @18,4,38,1 +++ b/src/main.rs
      cell @58,4,1,1 ▐
      cell @60,4,36,1 --- /dev/null
      cell @97,4,1,1
      cell @99,4,1,1 ▐
    row @0,5,100,1
      cell @0,5,17,1 │  A  lib.rs    │
      cell @18,5,38,1 @@ -3,3 +3,5 @@ fn main() {
      cell @58,5,1,1 ▐
      cell @60,5,36,1 +++ b/src/lib.rs
      cell @97,5,1,1
      cell @99,5,1,1 ▐
    row @0,6,100,1
      cell @0,6,17,1 │   M main.rs   │
      cell @18,6,38,1 }
      cell @58,6,1,1 ▐
      cell @60,6,36,1 @@ -0,0 +1 @@
      cell @97,6,1,1
      cell @99,6,1,1 ▐
    row @0,7,100,1
      cell @0,7,17,1 │               │
      cell @18,7,38,1
      cell @58,7,1,1 ▐
      cell @60,7,36,1 +pub fn add(a: i32, b: i32) -> i32 {
      cell @97,7,1,1 a
      cell @99,7,1,1 ▐
    row @0,8,100,1
      cell @0,8,17,1 │               │
      cell @18,8,38,1 fn helper() {}
      cell @58,8,1,1 ▐
      cell @60,8,36,1 + b }
      cell @97,8,1,1
      cell @99,8,1,1 ▐
    row @0,9,100,1
      cell @0,9,17,1 │               │
      cell @18,9,38,1 +
      cell @58,9,1,1 ▐
      cell @60,9,36,1
      cell @97,9,1,1
      cell @99,9,1,1 ▐
    row @0,10,100,1
      cell @0,10,17,1 ╰────────1 of 3─╯
      cell @18,10,38,1 +// TODO: fix this
      cell @58,10,1,1 ▐
      cell @60,10,36,1
      cell @97,10,1,1
      cell @99,10,1,1 ▐
    row @0,11,100,1
      cell @0,11,17,1 ╭─[3]─Local bra─╮
      cell @18,11,38,1
      cell @58,11,1,1 ▐
      cell @60,11,36,1
      cell @97,11,1,1
      cell @99,11,1,1 ▐
    row @0,12,100,1
      cell @0,12,17,1 │  * main       │
      cell @18,12,38,1
      cell @58,12,1,1 ▐
      cell @60,12,36,1
      cell @97,12,1,1
      cell @99,12,1,1 ▐
    row @0,13,100,1
      cell @0,13,17,1 │               │
      cell @18,13,38,1
      cell @58,13,1,1 ▐
      cell @60,13,36,1
      cell @97,13,1,1
      cell @99,13,1,1 ▐
    row @0,14,100,1
      cell @0,14,17,1 │               │
      cell @18,14,38,1
      cell @58,14,1,1 ▐
      cell @60,14,36,1
      cell @97,14,1,1
      cell @99,14,1,1 ▐
    row @0,15,100,1
      cell @0,15,17,1 │               │
      cell @18,15,38,1
      cell @58,15,1,1 ▐
      cell @60,15,36,1
      cell @97,15,1,1
      cell @99,15,1,1 ▐
    row @0,16,100,1
      cell @0,16,17,1 │               │
      cell @18,16,38,1
      cell @58,16,1,1 ▐
      cell @60,16,36,1
      cell @97,16,1,1
      cell @99,16,1,1 ▐
    row @0,17,100,1
      cell @0,17,17,1 │               │
      cell @18,17,38,1
      cell @58,17,1,1 ▐
      cell @60,17,36,1
      cell @97,17,1,1
      cell @99,17,1,1 ▐
    row @0,18,100,1
      cell @0,18,17,1 ╰────────1 of 1─╯
      cell @18,18,38,1
      cell @58,18,1,1 ▐
      cell @60,18,36,1
      cell @97,18,1,1
      cell @99,18,1,1 ▐
    row @0,19,100,1
      cell @0,19,17,1 ╭─[4]─Commits -─╮
      cell @18,19,38,1
      cell @58,19,1,1 │
      cell @60,19,36,1
      cell @97,19,1,1
      cell @99,19,1,1 ▐
    row @0,20,100,1
      cell @0,20,17,1 │88649d95 De ○ a│
      cell @18,20,38,1
      cell @58,20,1,1 │
      cell @60,20,36,1
      cell @97,20,1,1
      cell @99,20,1,1 │
    row @0,21,100,1
      cell @0,21,17,1 │96658a62 De ○ a│
      cell @18,21,38,1
      cell @58,21,1,1 │
      cell @60,21,36,1
      cell @97,21,1,1
      cell @99,21,1,1 │
    row @0,22,100,1
      cell @0,22,17,1 │8eb6eeea De ○ i│
      cell @18,22,38,1
      cell @58,22,1,1 │
      cell @60,22,36,1
      cell @97,22,1,1
      cell @99,22,1,1 │
    row @0,23,100,1
      cell @0,23,17,1 │               │
      cell @18,23,38,1
      cell @58,23,1,1 │
      cell @60,23,36,1
      cell @97,23,1,1
      cell @99,23,1,1 │
    row @0,24,100,1
      cell @0,24,17,1 │               │
      cell @18,24,38,1
      cell @58,24,1,1 │
      cell @60,24,36,1
      cell @97,24,1,1
      cell @99,24,1,1 │
    row @0,25,100,1
      cell @0,25,17,1 ╰────────1 of 3─╯
      cell @18,25,38,1 ──────────────────────────────────────
      cell @58,25,1,1 ╯
      cell @60,25,36,1 ────────────────────────────────────
      cell @97,25,1,1 ─
      cell @99,25,1,1 ╯
  panel "[5] Stash Command log" @0,26,100,3
    panel "[5] Stash" @0,26,17,3
    text "Command log" @17,26,83,3 can press 'M' in the files panel to open merge options
  statusbar @0,29,100,1 Stage: <space> | Commit: c | Stash: s | Discard: d | Reset: D | …         Donate Ask Question 0.63.1
```

## After — toon

```
application
  table [26]{c0,c1,c2,c3,c4,c5}:
     ╭─[1]─Status────╮,─[0]─Unstaged changes─────────────────,╮,─Staged changes─────────────────────,─,╮
     │unrender-field-│,diff --git a/src/main.rs b/src/main.rs,▐,diff --git a/src/lib.rs b/src/lib.rs,,▐
     ╰───────────────╯,index a9c3211..64218dc 100644,▐,new file mode 100644,,▐
     ╭─[2]─Files - W─╮,--- a/src/main.rs,▐,index 0000000..a051497,,▐
    *│▼ src          │,+++ b/src/main.rs,▐,--- /dev/null,,▐
     │  A  lib.rs    │,@@ -3,3 +3,5 @@ fn main() {,▐,+++ b/src/lib.rs,,▐
     │   M main.rs   │,},▐,@@ -0,0 +1 @@,,▐
     │               │,,▐,+pub fn add(a: i32, b: i32) -> i32 {,a,▐
     │               │,fn helper() {},▐,+ b },,▐
     │               │,+,▐,,,▐
     ╰────────1 of 3─╯,+// TODO: fix this,▐,,,▐
     ╭─[3]─Local bra─╮,,▐,,,▐
     │  * main       │,,▐,,,▐
     │               │,,▐,,,▐
     │               │,,▐,,,▐
     │               │,,▐,,,▐
     │               │,,▐,,,▐
     │               │,,▐,,,▐
     ╰────────1 of 1─╯,,▐,,,▐
     ╭─[4]─Commits -─╮,,│,,,▐
     │88649d95 De ○ a│,,│,,,│
     │96658a62 De ○ a│,,│,,,│
     │8eb6eeea De ○ i│,,│,,,│
     │               │,,│,,,│
     │               │,,│,,,│
     ╰────────1 of 3─╯,──────────────────────────────────────,╯,────────────────────────────────────,─,╯
  panel [5] Stash Command log
    panel [5] Stash
    text Command log: can press 'M' in the files panel to open merge options
  statusbar: Stage: <space> | Commit: c | Stash: s | Discard: d | Reset: D | …         Donate Ask Question 0.63.1
```

<details><summary>After — json</summary>

```json
{
  "role": "application",
  "rect": [
    0,
    0,
    100,
    30
  ],
  "children": [
    {
      "role": "table",
      "rect": [
        0,
        0,
        100,
        26
      ],
      "children": [
        {
          "role": "row",
          "rect": [
            0,
            0,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "╭─[1]─Status────╮",
              "rect": [
                0,
                0,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "value": "─[0]─Unstaged changes─────────────────",
              "rect": [
                18,
                0,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "╮",
              "rect": [
                58,
                0,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "─Staged changes─────────────────────",
              "rect": [
                60,
                0,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "value": "─",
              "rect": [
                97,
                0,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "╮",
              "rect": [
                99,
                0,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│unrender-field-│",
              "rect": [
                0,
                1,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "value": "diff --git a/src/main.rs b/src/main.rs",
              "rect": [
                18,
                1,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                1,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "diff --git a/src/lib.rs b/src/lib.rs",
              "rect": [
                60,
                1,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                1,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                1,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "╰───────────────╯",
              "rect": [
                0,
                2,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "value": "index a9c3211..64218dc 100644",
              "rect": [
                18,
                2,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                2,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "new file mode 100644",
              "rect": [
                60,
                2,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                2,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                2,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "╭─[2]─Files - W─╮",
              "rect": [
                0,
                3,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "value": "--- a/src/main.rs",
              "rect": [
                18,
                3,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                3,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "index 0000000..a051497",
              "rect": [
                60,
                3,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                3,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                3,
                1,
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
            100,
            1
          ],
          "states": [
            "selected"
          ],
          "children": [
            {
              "role": "cell",
              "value": "│▼ src          │",
              "rect": [
                0,
                4,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "value": "+++ b/src/main.rs",
              "rect": [
                18,
                4,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                4,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "--- /dev/null",
              "rect": [
                60,
                4,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                4,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                4,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│  A  lib.rs    │",
              "rect": [
                0,
                5,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "value": "@@ -3,3 +3,5 @@ fn main() {",
              "rect": [
                18,
                5,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                5,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "+++ b/src/lib.rs",
              "rect": [
                60,
                5,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                5,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                5,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│   M main.rs   │",
              "rect": [
                0,
                6,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "value": "}",
              "rect": [
                18,
                6,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                6,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "@@ -0,0 +1 @@",
              "rect": [
                60,
                6,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                6,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                6,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│               │",
              "rect": [
                0,
                7,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                18,
                7,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                7,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "+pub fn add(a: i32, b: i32) -> i32 {",
              "rect": [
                60,
                7,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "value": "a",
              "rect": [
                97,
                7,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                7,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│               │",
              "rect": [
                0,
                8,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "value": "fn helper() {}",
              "rect": [
                18,
                8,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                8,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "+ b }",
              "rect": [
                60,
                8,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                8,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                8,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│               │",
              "rect": [
                0,
                9,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "value": "+",
              "rect": [
                18,
                9,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                9,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                9,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                9,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                9,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "╰────────1 of 3─╯",
              "rect": [
                0,
                10,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "value": "+// TODO: fix this",
              "rect": [
                18,
                10,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                10,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                10,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                10,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                10,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "╭─[3]─Local bra─╮",
              "rect": [
                0,
                11,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                18,
                11,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                11,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                11,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                11,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                11,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│  * main       │",
              "rect": [
                0,
                12,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                18,
                12,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                12,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                12,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                12,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                12,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│               │",
              "rect": [
                0,
                13,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                18,
                13,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                13,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                13,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                13,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                13,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│               │",
              "rect": [
                0,
                14,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                18,
                14,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                14,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                14,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                14,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                14,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│               │",
              "rect": [
                0,
                15,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                18,
                15,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                15,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                15,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                15,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                15,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│               │",
              "rect": [
                0,
                16,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                18,
                16,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                16,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                16,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                16,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                16,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│               │",
              "rect": [
                0,
                17,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                18,
                17,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                17,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                17,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                17,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                17,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "╰────────1 of 1─╯",
              "rect": [
                0,
                18,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                18,
                18,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                58,
                18,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                18,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                18,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                18,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "╭─[4]─Commits -─╮",
              "rect": [
                0,
                19,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                18,
                19,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "│",
              "rect": [
                58,
                19,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                19,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                19,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "▐",
              "rect": [
                99,
                19,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│88649d95 De ○ a│",
              "rect": [
                0,
                20,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                18,
                20,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "│",
              "rect": [
                58,
                20,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                20,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                20,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "│",
              "rect": [
                99,
                20,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│96658a62 De ○ a│",
              "rect": [
                0,
                21,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                18,
                21,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "│",
              "rect": [
                58,
                21,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                21,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                21,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "│",
              "rect": [
                99,
                21,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│8eb6eeea De ○ i│",
              "rect": [
                0,
                22,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                18,
                22,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "│",
              "rect": [
                58,
                22,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                22,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                22,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "│",
              "rect": [
                99,
                22,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│               │",
              "rect": [
                0,
                23,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                18,
                23,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "│",
              "rect": [
                58,
                23,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                23,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                23,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "│",
              "rect": [
                99,
                23,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "│               │",
              "rect": [
                0,
                24,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                18,
                24,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "│",
              "rect": [
                58,
                24,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                60,
                24,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                97,
                24,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "│",
              "rect": [
                99,
                24,
                1,
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
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "╰────────1 of 3─╯",
              "rect": [
                0,
                25,
                17,
                1
              ]
            },
            {
              "role": "cell",
              "value": "──────────────────────────────────────",
              "rect": [
                18,
                25,
                38,
                1
              ]
            },
            {
              "role": "cell",
              "value": "╯",
              "rect": [
                58,
                25,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "────────────────────────────────────",
              "rect": [
                60,
                25,
                36,
                1
              ]
            },
            {
              "role": "cell",
              "value": "─",
              "rect": [
                97,
                25,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "╯",
              "rect": [
                99,
                25,
                1,
                1
              ]
            }
          ]
        }
      ]
    },
    {
      "role": "panel",
      "name": "[5] Stash Command log",
      "rect": [
        0,
        26,
        100,
        3
      ],
      "children": [
        {
          "role": "panel",
          "name": "[5] Stash",
          "rect": [
            0,
            26,
            17,
            3
          ]
        },
        {
          "role": "text",
          "name": "Command log",
          "value": "can press 'M' in the files panel to open merge options",
          "rect": [
            17,
            26,
            83,
            3
          ]
        }
      ]
    },
    {
      "role": "statusbar",
      "value": "Stage: <space> | Commit: c | Stash: s | Discard: d | Reset: D | …         Donate Ask Question 0.63.1",
      "rect": [
        0,
        29,
        100,
        1
      ]
    }
  ]
}
```

</details>

