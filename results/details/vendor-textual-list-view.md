# vendor/textual/list-view

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **0%** (chrome-excluded; 11 non-blank content cells)

## Sizes

| encoding | bytes | tokens | vs plain (bytes) | vs plain (tokens) |
|---|---:|---:|---:|---:|
| raw | 2882 | 874 | 0.65x | 0.11x |
| plain | 1862 | 99 | 1.00x | 1.00x |
| compact | 105 | 65 | 17.73x | 1.52x |
| nogeo | 62 | 30 | 30.03x | 3.30x |
| toon | 65 | 33 | 28.65x | 3.00x |
| json | 487 | 208 | 3.82x | 0.48x |

<details><summary>Before — raw ANSI (2882 bytes, escapes shown literally)</summary>

```
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                                                                                \x1b[0m
\x1b[0;48;2;18;18;18m                         \x1b[0;48;2;39;39;39m                              \x1b[0;48;2;18;18;18m                         \x1b[0m
\x1b[0;48;2;18;18;18m                         \x1b[0;48;2;39;39;39m  \x1b[0;38;2;224;224;224;48;2;39;39;39mOne\x1b[0;48;2;39;39;39m                         \x1b[0;48;2;18;18;18m                         \x1b[0m
\x1b[0;48;2;18;18;18m                         \x1b[0;48;2;39;39;39m                              \x1b[0;48;2;18;18;18m                         \x1b[0m
\x1b[0;48;2;18;18;18m                         \x1b[0;48;2;1;120;212m                              \x1b[0;48;2;18;18;18m                         \x1b[0m
\x1b[0;48;2;18;18;18m                         \x1b[0;48;2;1;120;212m  \x1b[0;1;38;2;221;237;249;48;2;1;120;212mTwo\x1b[0;48;2;1;120;212m                         \x1b[0;48;2;18;18;18m                         \x1b[0m
\x1b[0;48;2;18;18;18m                         \x1b[0;48;2;1;120;212m                              \x1b[0;48;2;18;18;18m                         \x1b[0m
\x1b[0;48;2;18;18;18m                         \x1b[0;48;2;39;39;39m                              \x1b[0;48;2;18;18;18m                         \x1b[0m
\x1b[0;48;2;18;18;18m                         \x1b[0;48;2;39;39;39m  \x1b[0;38;2;224;224;224;48;2;39;39;39mThree\x1b[0;48;2;39;39;39m                       \x1b[0;48;2;18;18;18m                         \x1b[0m
\x1b[0;48;2;18;18;18m                         \x1b[0;48;2;39;39;39m                              \x1b[0;48;2;18;18;18m                         \x1b[0m
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
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                           One                                                  
                                                                                
                                                                                
                           Two                                                  
                                                                                
                                                                                
                           Three
```

## After — compact

```
application @0,0,80,23
  statusbar @0,8,80,1 One
  statusbar @0,11,80,1 Two
  statusbar @0,14,80,1 Three
```

## After — toon

```
application
  statusbar: One
  statusbar: Two
  statusbar: Three
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
      "value": "One",
      "rect": [
        0,
        8,
        80,
        1
      ]
    },
    {
      "role": "statusbar",
      "value": "Two",
      "rect": [
        0,
        11,
        80,
        1
      ]
    },
    {
      "role": "statusbar",
      "value": "Three",
      "rect": [
        0,
        14,
        80,
        1
      ]
    }
  ]
}
```

</details>

