# vendor/textual/progress-bar

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **0%** (chrome-excluded; 11 non-blank content cells)

## Sizes

| encoding | bytes | tokens | vs plain (bytes) | vs plain (tokens) |
|---|---:|---:|---:|---:|
| raw | 2645 | 682 | 0.73x | 0.17x |
| plain | 1926 | 117 | 1.00x | 1.00x |
| compact | 157 | 73 | 12.27x | 1.60x |
| nogeo | 135 | 56 | 14.27x | 2.09x |
| toon | 136 | 57 | 14.16x | 2.05x |
| json | 331 | 145 | 5.82x | 0.81x |

<details><summary>Before — raw ANSI (2645 bytes, escapes shown literally)</summary>

```
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                 \x1b[0;38;2;1;120;212;48;2;18;18;18m━━━━━━━━━━━━\x1b[0;38;2;30;30;30;48;2;18;18;18m╺━━━━━━━━━━━━━━━━━━━\x1b[0;48;2;18;18;18m  \x1b[0;38;2;224;224;224;48;2;18;18;18m39%\x1b[0;48;2;18;18;18m \x1b[0;38;2;224;224;224;48;2;18;18;18m00:00:07\x1b[0;48;2;18;18;18m                 \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
```

</details>

## Before (color-stripped)

```
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                 ━━━━━━━━━━━━╺━━━━━━━━━━━━━━━━━━━  39% 00:00:07
```

## After — compact

```
application @0,0,80,23
  statusbar @0,11,80,1 ━━━━━━━━━━━━╺━━━━━━━━━━━━━━━━━━━  39% 00:00:07
```

## After — toon

```
application
  statusbar: ━━━━━━━━━━━━╺━━━━━━━━━━━━━━━━━━━  39% 00:00:07
```

<details><summary>After — json</summary>

```json
{
  "role": "application",
  "rect": [
    0,
    0,
    80,
    23
  ],
  "children": [
    {
      "role": "statusbar",
      "value": "━━━━━━━━━━━━╺━━━━━━━━━━━━━━━━━━━  39% 00:00:07",
      "rect": [
        0,
        11,
        80,
        1
      ]
    }
  ]
}
```

</details>

