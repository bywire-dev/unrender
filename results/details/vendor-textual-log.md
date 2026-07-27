# vendor/textual/log

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **100%** (chrome-excluded; 24 non-blank content cells)

## Sizes

| encoding | bytes | tokens | vs plain (bytes) | vs plain (tokens) |
|---|---:|---:|---:|---:|
| raw | 1924 | 844 | 0.49x | 0.03x |
| plain | 944 | 25 | 1.00x | 1.00x |
| compact | 244 | 149 | 3.87x | 0.17x |
| toon | 77 | 43 | 12.26x | 0.58x |
| json | 1950 | 547 | 0.48x | 0.05x |

<details><summary>Before — raw ANSI (1924 bytes, escapes shown literally)</summary>

```
\x1b[0;38;2;226;226;226;48;2;39;39;39mHello, World!\x1b[0;48;2;39;39;39m                          \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;38;2;226;226;226;48;2;39;39;39mWhat's up?\x1b[0;48;2;39;39;39m                             \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;38;2;226;226;226;48;2;39;39;39mFOO\x1b[0;48;2;39;39;39m                                    \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
\x1b[0;48;2;39;39;39m                                       \x1b[0;48;2;0;0;0m \x1b[0m
```

</details>

## Before (color-stripped)

```
Hello, World!                           
What's up?                              
FOO
```

## After — compact

```
application @0,0,40,23
  table @0,0,40,3
    row @0,0,40,1
      cell @0,0,6,1 Hello,
      cell @7,0,6,1 World!
    row @0,1,40,1
      cell @0,1,6,1 What's
      cell @7,1,6,1 up?
    row @0,2,40,1
      cell @0,2,6,1 FOO
      cell @7,2,6,1
```

## After — toon

```
application
  table [3]{c0,c1}:
     Hello,,World!
     What's,up?
     FOO,
```

<details><summary>After — json</summary>

```json
{
  "role": "application",
  "rect": [
    0,
    0,
    40,
    23
  ],
  "children": [
    {
      "role": "table",
      "rect": [
        0,
        0,
        40,
        3
      ],
      "children": [
        {
          "role": "row",
          "rect": [
            0,
            0,
            40,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "Hello,",
              "rect": [
                0,
                0,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "World!",
              "rect": [
                7,
                0,
                6,
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
            40,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "What's",
              "rect": [
                0,
                1,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "value": "up?",
              "rect": [
                7,
                1,
                6,
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
            40,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "FOO",
              "rect": [
                0,
                2,
                6,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                7,
                2,
                6,
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

