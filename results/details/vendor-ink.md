# vendor/ink

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **80%** (chrome-excluded; 205 non-blank content cells)

## Sizes

| encoding | bytes | tokens | vs plain (bytes) | vs plain (tokens) |
|---|---:|---:|---:|---:|
| raw | 2053 | 563 | 0.77x | 0.47x |
| plain | 1572 | 266 | 1.00x | 1.00x |
| compact | 1029 | 529 | 1.53x | 0.50x |
| toon | 387 | 159 | 4.06x | 1.67x |
| json | 7624 | 1914 | 0.21x | 0.14x |

<details><summary>Before — raw ANSI (2053 bytes, escapes shown literally)</summary>

```
\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m╭────────────────────────────────────────────────────────────────────────────╮
\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m Deploy Console — cluster prod-eu-1 \x1b[m                                        \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m│
\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m╰────────────────────────────────────────────────────────────────────────────╯
\x1b[m╭────────────────────────────────────────────╮╭──────────────────────────────╮
\x1b[m│\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mSERVICE        STATE      P99     \x1b[m          ││events                        │
\x1b[m│\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[7m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mapi-gateway    running    12ms    \x1b[m          ││• deploy started              │
\x1b[m│auth-service   running    31ms              ││• image pulled                │
\x1b[m│billing        degraded   412ms             ││• health check ok             │
\x1b[m│search-index   running    88ms              ││                              │
\x1b[m│mailer         stopped    -                 ││                              │
\x1b[m╰────────────────────────────────────────────╯╰──────────────────────────────╯
\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[7m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m j/k move   q quit
\x1b[m
```

</details>

## Before (color-stripped)

```
╭────────────────────────────────────────────────────────────────────────────╮
│ Deploy Console — cluster prod-eu-1                                         │
╰────────────────────────────────────────────────────────────────────────────╯
╭────────────────────────────────────────────╮╭──────────────────────────────╮
│SERVICE        STATE      P99               ││events                        │
│api-gateway    running    12ms              ││• deploy started              │
│auth-service   running    31ms              ││• image pulled                │
│billing        degraded   412ms             ││• health check ok             │
│search-index   running    88ms              ││                              │
│mailer         stopped    -                 ││                              │
╰────────────────────────────────────────────╯╰──────────────────────────────╯
 j/k move   q quit
```

## After — compact

```
application @0,0,78,13
  text @0,0,78,3 Deploy Console — cluster prod-eu-1
  panel @0,3,78,8
    table @0,3,46,8
      rowheader @1,4,44,1
        cell @1,4,12,1 SERVICE
        cell @16,4,8,1 STATE
        cell @27,4,5,1 P99
      row [selected] @1,5,44,1
        cell @1,5,12,1 api-gateway
        cell @16,5,8,1 running
        cell @27,5,5,1 12ms
      row @1,6,44,1
        cell @1,6,12,1 auth-service
        cell @16,6,8,1 running
        cell @27,6,5,1 31ms
      row @1,7,44,1
        cell @1,7,12,1 billing
        cell @16,7,8,1 degraded
        cell @27,7,5,1 412ms
      row @1,8,44,1
        cell @1,8,12,1 search-index
        cell @16,8,8,1 running
        cell @27,8,5,1 88ms
      row @1,9,44,1
        cell @1,9,12,1 mailer
        cell @16,9,8,1 stopped
        cell @27,9,5,1 -
    list @46,3,32,8
      listitem @47,4,30,1 events
      listitem @47,5,30,1 • deploy started
      listitem @47,6,30,1 • image pulled
      listitem @47,7,30,1 • health check ok
  statusbar @0,11,78,1 j/k move   q quit
```

## After — toon

```
application
  text: Deploy Console — cluster prod-eu-1
  panel
    table [5]{SERVICE,STATE,P99}:
      *api-gateway,running,12ms
       auth-service,running,31ms
       billing,degraded,412ms
       search-index,running,88ms
       mailer,stopped,-
    list [4]:
       events
       • deploy started
       • image pulled
       • health check ok
  statusbar: j/k move   q quit
```

<details><summary>After — json</summary>

```json
{
  "role": "application",
  "rect": [
    0,
    0,
    78,
    13
  ],
  "children": [
    {
      "role": "text",
      "value": "Deploy Console — cluster prod-eu-1",
      "rect": [
        0,
        0,
        78,
        3
      ]
    },
    {
      "role": "panel",
      "rect": [
        0,
        3,
        78,
        8
      ],
      "children": [
        {
          "role": "table",
          "rect": [
            0,
            3,
            46,
            8
          ],
          "children": [
            {
              "role": "rowheader",
              "rect": [
                1,
                4,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "SERVICE",
                  "rect": [
                    1,
                    4,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "STATE",
                  "rect": [
                    16,
                    4,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "P99",
                  "rect": [
                    27,
                    4,
                    5,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                1,
                5,
                44,
                1
              ],
              "states": [
                "selected"
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "api-gateway",
                  "rect": [
                    1,
                    5,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "running",
                  "rect": [
                    16,
                    5,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "12ms",
                  "rect": [
                    27,
                    5,
                    5,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                1,
                6,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "auth-service",
                  "rect": [
                    1,
                    6,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "running",
                  "rect": [
                    16,
                    6,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "31ms",
                  "rect": [
                    27,
                    6,
                    5,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                1,
                7,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "billing",
                  "rect": [
                    1,
                    7,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "degraded",
                  "rect": [
                    16,
                    7,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "412ms",
                  "rect": [
                    27,
                    7,
                    5,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                1,
                8,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "search-index",
                  "rect": [
                    1,
                    8,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "running",
                  "rect": [
                    16,
                    8,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "88ms",
                  "rect": [
                    27,
                    8,
                    5,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                1,
                9,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "mailer",
                  "rect": [
                    1,
                    9,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "stopped",
                  "rect": [
                    16,
                    9,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "-",
                  "rect": [
                    27,
                    9,
                    5,
                    1
                  ]
                }
              ]
            }
          ]
        },
        {
          "role": "list",
          "rect": [
            46,
            3,
            32,
            8
          ],
          "children": [
            {
              "role": "listitem",
              "value": "events",
              "rect": [
                47,
                4,
                30,
                1
              ]
            },
            {
              "role": "listitem",
              "value": "• deploy started",
              "rect": [
                47,
                5,
                30,
                1
              ]
            },
            {
              "role": "listitem",
              "value": "• image pulled",
              "rect": [
                47,
                6,
                30,
                1
              ]
            },
            {
              "role": "listitem",
              "value": "• health check ok",
              "rect": [
                47,
                7,
                30,
                1
              ]
            }
          ]
        }
      ]
    },
    {
      "role": "statusbar",
      "value": "j/k move   q quit",
      "rect": [
        0,
        11,
        78,
        1
      ]
    }
  ]
}
```

</details>

