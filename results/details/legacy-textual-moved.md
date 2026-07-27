# legacy/textual-moved

[← back to REPORT.md](../REPORT.md)

truth source: `framework`

content preservation: **67%** (chrome-excluded; 233 non-blank content cells)

structural: recall 100%, role agreement 100%, mean IoU 1.00

## Before (color-stripped)

```
 ⭘                                        Deploy Console                                            
╭──────────────────────────────────────────────────────────╮╭──────────────────────────────────────╮
│ SERVICE       STATE     P99                              ││▼ cluster                             │
│ api-gateway   running   12ms                             ││├── ▼ prod-eu-1                       │
│ auth-service  running   31ms                             │││   ├── node-a                        │
│ billing       degraded  412ms                            │││   └── node-b                        │
│ search-index  running   88ms                             ││└── ▼ prod-us-1                       │
│ mailer        stopped   -                                ││    └── node-c                        │
│ scheduler     running   5ms                              ││                                      │
╰──────────────────────────────────────────────────────────╯│                                      │
                                                            │                                      │
                                                            │                                      │
                                                            │                                      │
                                                            │                                      │
                                                            │                                      │
                                                            │                                      │
                                                            │                                      │
                                                            │                                      │
                                                            │                                      │
                                                            │                                      │
                                                            │                                      │
                                                            │                                      │
                                                            │                                      │
                                                            │                                      │
                                                            │                                      │
                                                            ╰──────────────────────────────────────╯
╭──────────────────────────────────────────────────────────────────────────────────────────────────╮
│rollout 50% :: 3 of 6 healthy                                                                     │
╰──────────────────────────────────────────────────────────────────────────────────────────────────╯
 q Quit                                                                                 ▏^p palette
```

## After — compact

```
application @0,0,100,30
  heading @0,0,100,1 ⭘                                        Deploy Console
  table @0,1,60,9
    row [selected] @1,2,58,1
      cell @2,2,12,1 SERVICE
      cell @16,2,8,1 STATE
      cell @26,2,5,1 P99
    row @1,3,58,1
      cell @2,3,12,1 api-gateway
      cell @16,3,8,1 running
      cell @26,3,5,1 12ms
    row @1,4,58,1
      cell @2,4,12,1 auth-service
      cell @16,4,8,1 running
      cell @26,4,5,1 31ms
    row @1,5,58,1
      cell @2,5,12,1 billing
      cell @16,5,8,1 degraded
      cell @26,5,5,1 412ms
    row @1,6,58,1
      cell @2,6,12,1 search-index
      cell @16,6,8,1 running
      cell @26,6,5,1 88ms
    row [selected] @1,7,58,1
      cell @2,7,12,1 mailer
      cell @16,7,8,1 stopped
      cell @26,7,5,1 -
    row @1,8,58,1
      cell @2,8,12,1 scheduler
      cell @16,8,8,1 running
      cell @26,8,5,1 5ms
  tree @60,1,40,25
    treeitem [depth=0,expanded,selected] @61,2,38,1 cluster
    treeitem [depth=1,expanded] @61,3,38,1 prod-eu-1
    treeitem [depth=2] @61,4,38,1 node-a
    treeitem [depth=2] @61,5,38,1 node-b
    treeitem [depth=1,expanded] @61,6,38,1 prod-us-1
    treeitem [depth=2] @61,7,38,1 node-c
  text @0,26,100,3 rollout 50% :: 3 of 6 healthy
  statusbar @0,29,100,1 q Quit                                                                                 ▏^p palette
```

## After — toon

```
application
  heading: ⭘                                        Deploy Console
  table [7]{c0,c1,c2}:
    *SERVICE,STATE,P99
     api-gateway,running,12ms
     auth-service,running,31ms
     billing,degraded,412ms
     search-index,running,88ms
    *mailer,stopped,-
     scheduler,running,5ms
  tree [6]:
    *0:cluster
     1:prod-eu-1
     2:node-a
     2:node-b
     1:prod-us-1
     2:node-c
  text: rollout 50% :: 3 of 6 healthy
  statusbar: q Quit                                                                                 ▏^p palette
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
      "role": "heading",
      "value": "⭘                                        Deploy Console",
      "rect": [
        0,
        0,
        100,
        1
      ]
    },
    {
      "role": "table",
      "rect": [
        0,
        1,
        60,
        9
      ],
      "children": [
        {
          "role": "row",
          "rect": [
            1,
            2,
            58,
            1
          ],
          "states": [
            "selected"
          ],
          "children": [
            {
              "role": "cell",
              "value": "SERVICE",
              "rect": [
                2,
                2,
                12,
                1
              ]
            },
            {
              "role": "cell",
              "value": "STATE",
              "rect": [
                16,
                2,
                8,
                1
              ]
            },
            {
              "role": "cell",
              "value": "P99",
              "rect": [
                26,
                2,
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
            3,
            58,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "api-gateway",
              "rect": [
                2,
                3,
                12,
                1
              ]
            },
            {
              "role": "cell",
              "value": "running",
              "rect": [
                16,
                3,
                8,
                1
              ]
            },
            {
              "role": "cell",
              "value": "12ms",
              "rect": [
                26,
                3,
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
            4,
            58,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "auth-service",
              "rect": [
                2,
                4,
                12,
                1
              ]
            },
            {
              "role": "cell",
              "value": "running",
              "rect": [
                16,
                4,
                8,
                1
              ]
            },
            {
              "role": "cell",
              "value": "31ms",
              "rect": [
                26,
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
            58,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "billing",
              "rect": [
                2,
                5,
                12,
                1
              ]
            },
            {
              "role": "cell",
              "value": "degraded",
              "rect": [
                16,
                5,
                8,
                1
              ]
            },
            {
              "role": "cell",
              "value": "412ms",
              "rect": [
                26,
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
            58,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "search-index",
              "rect": [
                2,
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
              "value": "88ms",
              "rect": [
                26,
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
            58,
            1
          ],
          "states": [
            "selected"
          ],
          "children": [
            {
              "role": "cell",
              "value": "mailer",
              "rect": [
                2,
                7,
                12,
                1
              ]
            },
            {
              "role": "cell",
              "value": "stopped",
              "rect": [
                16,
                7,
                8,
                1
              ]
            },
            {
              "role": "cell",
              "value": "-",
              "rect": [
                26,
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
            58,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "scheduler",
              "rect": [
                2,
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
              "value": "5ms",
              "rect": [
                26,
                8,
                5,
                1
              ]
            }
          ]
        }
      ]
    },
    {
      "role": "tree",
      "rect": [
        60,
        1,
        40,
        25
      ],
      "children": [
        {
          "role": "treeitem",
          "value": "cluster",
          "rect": [
            61,
            2,
            38,
            1
          ],
          "states": [
            "depth=0",
            "expanded",
            "selected"
          ]
        },
        {
          "role": "treeitem",
          "value": "prod-eu-1",
          "rect": [
            61,
            3,
            38,
            1
          ],
          "states": [
            "depth=1",
            "expanded"
          ]
        },
        {
          "role": "treeitem",
          "value": "node-a",
          "rect": [
            61,
            4,
            38,
            1
          ],
          "states": [
            "depth=2"
          ]
        },
        {
          "role": "treeitem",
          "value": "node-b",
          "rect": [
            61,
            5,
            38,
            1
          ],
          "states": [
            "depth=2"
          ]
        },
        {
          "role": "treeitem",
          "value": "prod-us-1",
          "rect": [
            61,
            6,
            38,
            1
          ],
          "states": [
            "depth=1",
            "expanded"
          ]
        },
        {
          "role": "treeitem",
          "value": "node-c",
          "rect": [
            61,
            7,
            38,
            1
          ],
          "states": [
            "depth=2"
          ]
        }
      ]
    },
    {
      "role": "text",
      "value": "rollout 50% :: 3 of 6 healthy",
      "rect": [
        0,
        26,
        100,
        3
      ]
    },
    {
      "role": "statusbar",
      "value": "q Quit                                                                                 ▏^p palette",
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

