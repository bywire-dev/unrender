# field/gitui

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **37%** (chrome-excluded; 213 non-blank content cells)

## Sizes

| encoding | bytes | tokens | vs plain (bytes) | vs plain (tokens) |
|---|---:|---:|---:|---:|
| raw | 8716 | 3532 | 0.45x | 0.16x |
| plain | 3943 | 550 | 1.00x | 1.00x |
| compact | 793 | 311 | 4.97x | 1.77x |
| toon | 672 | 228 | 5.87x | 2.41x |
| json | 2035 | 651 | 1.94x | 0.84x |

<details><summary>Before — raw ANSI (8716 bytes, escapes shown literally)</summary>

```
\x1b[m \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m \x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[4m\x1b[22m\x1b[23mStatus [1]\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  |  Log [2]  |  Files [3]  |  Stashing [4]  |  Stashes [5]   […]-field-synthetic-repo/
\x1b[m \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m──────────────────────────────────────────────────────────────────────────────────────────────────
\x1b[m┌\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mUnstaged Changes\x1b[m────────────────────────────────┐\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m┌Diff: ──────────────────────────────────────────┐
\x1b[m│\x1b[38;5;15m\x1b[48;5;4m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  ▾src                                          \x1b[m│\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[m│\x1b[38;5;3m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mM   main.rs\x1b[m                                     │\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[m│                                                │\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[m│                                                │\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[m│                                                │\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[m│                                                │\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[m│                                                │\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[m│                                                │\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[m│                                                │\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[m│                                                │\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[m│                                                │\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[m│                                                │\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[m│                                                │\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[m│                                                │\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[m└──────────────────────────────────────────{main}┘\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m┌Staged Changes──────────────────────────────────┐│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m  ▾src                                          \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m││\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[38;5;10m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m+   lib.rs\x1b[m                                      \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m││\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m││\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m││\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m││\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m││\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m││\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m││\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m││\x1b[m                                                \x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[38;5;8m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m└────────────────────────────────────────────────┘└────────────────────────────────────────────────┘
\x1b[38;5;15m\x1b[48;5;4m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mStage All [a]\x1b[m \x1b[38;5;15m\x1b[48;5;4m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mStage [⏎]\x1b[m \x1b[38;5;15m\x1b[48;5;4m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mReset [⇧D]\x1b[m \x1b[38;5;15m\x1b[48;5;4m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mIgnore [i]\x1b[m \x1b[38;5;15m\x1b[48;5;4m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mBranches [b]\x1b[m \x1b[38;5;8m\x1b[48;5;4m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mPush [p]\x1b[m \x1b[38;5;8m\x1b[48;5;4m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mFetch [⇧F]\x1b[m \x1b[38;5;8m\x1b[48;5;4m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mPull [f]\x1b[m     more [.]\x1b[m
```

</details>

## Before (color-stripped)

```
  Status [1]  |  Log [2]  |  Files [3]  |  Stashing [4]  |  Stashes [5]   […]-field-synthetic-repo/
 ──────────────────────────────────────────────────────────────────────────────────────────────────
┌Unstaged Changes────────────────────────────────┐┌Diff: ──────────────────────────────────────────┐
│  ▾src                                          ││                                                │
│M   main.rs                                     ││                                                │
│                                                ││                                                │
│                                                ││                                                │
│                                                ││                                                │
│                                                ││                                                │
│                                                ││                                                │
│                                                ││                                                │
│                                                ││                                                │
│                                                ││                                                │
│                                                ││                                                │
│                                                ││                                                │
│                                                ││                                                │
│                                                ││                                                │
└──────────────────────────────────────────{main}┘│                                                │
┌Staged Changes──────────────────────────────────┐│                                                │
│  ▾src                                          ││                                                │
│+   lib.rs                                      ││                                                │
│                                                ││                                                │
│                                                ││                                                │
│                                                ││                                                │
│                                                ││                                                │
│                                                ││                                                │
│                                                ││                                                │
│                                                ││                                                │
└────────────────────────────────────────────────┘└────────────────────────────────────────────────┘
Stage All [a] Stage [⏎] Reset [⇧D] Ignore [i] Branches [b] Push [p] Fetch [⇧F] Pull [f]     more [.]
```

## After — compact

```
application @0,0,100,30
  log @0,0,100,2 Status [1]  |  Log [2]  |  Files [3]  |  Stashing [4]  |  Stashes [5]   […]-field-synthetic-repo/\n──────────────────────────────────────────────────────────────────────────────────────────────────
  panel "Unstaged Changes Diff:" @0,2,100,27
    progressbar "Unstaged Changes" @0,2,50,16 0%
    panel "Diff:" @50,2,50,27
    list "Staged Changes" @0,18,50,11
      listitem @1,19,48,1 src
      listitem @1,20,48,1 lib.rs
  statusbar @0,29,100,1 Stage All [a] Stage [⏎] Reset [⇧D] Ignore [i] Branches [b] Push [p] Fetch [⇧F] Pull [f]     more [.]
```

## After — toon

```
application
  log: Status [1]  |  Log [2]  |  Files [3]  |  Stashing [4]  |  Stashes [5]   […]-field-synthetic-repo/\n──────────────────────────────────────────────────────────────────────────────────────────────────
  panel Unstaged Changes Diff:
    progressbar Unstaged Changes: 0%
    panel Diff:
    list Staged Changes[2]:
       src
       lib.rs
  statusbar: Stage All [a] Stage [⏎] Reset [⇧D] Ignore [i] Branches [b] Push [p] Fetch [⇧F] Pull [f]     more [.]
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
      "role": "log",
      "value": "Status [1]  |  Log [2]  |  Files [3]  |  Stashing [4]  |  Stashes [5]   […]-field-synthetic-repo/\n──────────────────────────────────────────────────────────────────────────────────────────────────",
      "rect": [
        0,
        0,
        100,
        2
      ]
    },
    {
      "role": "panel",
      "name": "Unstaged Changes Diff:",
      "rect": [
        0,
        2,
        100,
        27
      ],
      "children": [
        {
          "role": "progressbar",
          "name": "Unstaged Changes",
          "value": "0%",
          "rect": [
            0,
            2,
            50,
            16
          ]
        },
        {
          "role": "panel",
          "name": "Diff:",
          "rect": [
            50,
            2,
            50,
            27
          ]
        },
        {
          "role": "list",
          "name": "Staged Changes",
          "rect": [
            0,
            18,
            50,
            11
          ],
          "children": [
            {
              "role": "listitem",
              "value": "src",
              "rect": [
                1,
                19,
                48,
                1
              ]
            },
            {
              "role": "listitem",
              "value": "lib.rs",
              "rect": [
                1,
                20,
                48,
                1
              ]
            }
          ]
        }
      ]
    },
    {
      "role": "statusbar",
      "value": "Stage All [a] Stage [⏎] Reset [⇧D] Ignore [i] Branches [b] Push [p] Fetch [⇧F] Pull [f]     more [.]",
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

