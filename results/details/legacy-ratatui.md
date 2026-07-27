# legacy/ratatui

[← back to REPORT.md](../REPORT.md)

truth source: `framework`

content preservation: **70%** (chrome-excluded; 298 non-blank content cells)

structural: recall 100%, role agreement 100%, mean IoU 1.00

## Before (color-stripped)

```
┌zellij-spike──────────────────────────────────────────────────────────────────────────────────────┐
│Deploy Console  ::  cluster prod-eu-1                                                             │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘
┌services──────────────────────────────────────────────────┐┌events────────────────────────────────┐
│  SERVICE          STATE      P99                         ││deploy started                        │
│> api-gateway      running    12ms                        ││image pulled                          │
│  auth-service     running    31ms                        ││health check ok                       │
│  billing          degraded   412ms                       ││traffic shifted 10%                   │
│  search-index     running    88ms                        ││traffic shifted 50%                   │
│  mailer           stopped    -                           ││                                      │
│  scheduler        running    5ms                         ││                                      │
│                                                          ││                                      │
│                                                          ││                                      │
│                                                          ││                                      │
│                                                          ││                                      │
│                                                          ││                                      │
│                                                          ││                                      │
│                                                          ││                                      │
│                                                          ││                                      │
│                                                          ││                                      │
│                                                          ││                                      │
│                                                          ││                                      │
│                                                          ││                                      │
│                                                          ││                                      │
│                                                          ││                                      │
│                                                          │└──────────────────────────────────────┘
│                                                          │┌rollout───────────────────────────────┐
│                                                          ││█████████████████50%                  │
└──────────────────────────────────────────────────────────┘└──────────────────────────────────────┘
 q quit  j/k move  ENTER select
```

## After — compact

```
application @0,0,100,30
  text "zellij-spike" @0,0,100,3 Deploy Console  ::  cluster prod-eu-1
  panel "services events" @0,3,100,26
    table "services" @0,3,60,26
      rowheader @1,4,58,1
        cell @1,4,1,1
        cell @3,4,12,1 SERVICE
        cell @20,4,8,1 STATE
        cell @31,4,5,1 P99
      row [selected] @1,5,58,1
        cell @1,5,1,1 >
        cell @3,5,12,1 api-gateway
        cell @20,5,8,1 running
        cell @31,5,5,1 12ms
      row @1,6,58,1
        cell @1,6,1,1
        cell @3,6,12,1 auth-service
        cell @20,6,8,1 running
        cell @31,6,5,1 31ms
      row @1,7,58,1
        cell @1,7,1,1
        cell @3,7,12,1 billing
        cell @20,7,8,1 degraded
        cell @31,7,5,1 412ms
      row @1,8,58,1
        cell @1,8,1,1
        cell @3,8,12,1 search-index
        cell @20,8,8,1 running
        cell @31,8,5,1 88ms
      row @1,9,58,1
        cell @1,9,1,1
        cell @3,9,12,1 mailer
        cell @20,9,8,1 stopped
        cell @31,9,5,1 -
      row @1,10,58,1
        cell @1,10,1,1
        cell @3,10,12,1 scheduler
        cell @20,10,8,1 running
        cell @31,10,5,1 5ms
    list "events" @60,3,40,23
      listitem @61,4,38,1 deploy started
      listitem @61,5,38,1 image pulled
      listitem @61,6,38,1 health check ok
      listitem @61,7,38,1 traffic shifted 10%
      listitem @61,8,38,1 traffic shifted 50%
    progressbar "rollout" @60,26,40,3 50%
  statusbar @0,29,100,1 q quit  j/k move  ENTER select
```

## After — toon

```
application
  text zellij-spike: Deploy Console  ::  cluster prod-eu-1
  panel services events
    table services[6]{,SERVICE,STATE,P99}:
      *>,api-gateway,running,12ms
       ,auth-service,running,31ms
       ,billing,degraded,412ms
       ,search-index,running,88ms
       ,mailer,stopped,-
       ,scheduler,running,5ms
    list events[5]:
       deploy started
       image pulled
       health check ok
       traffic shifted 10%
       traffic shifted 50%
    progressbar rollout: 50%
  statusbar: q quit  j/k move  ENTER select
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
      "role": "text",
      "name": "zellij-spike",
      "value": "Deploy Console  ::  cluster prod-eu-1",
      "rect": [
        0,
        0,
        100,
        3
      ]
    },
    {
      "role": "panel",
      "name": "services events",
      "rect": [
        0,
        3,
        100,
        26
      ],
      "children": [
        {
          "role": "table",
          "name": "services",
          "rect": [
            0,
            3,
            60,
            26
          ],
          "children": [
            {
              "role": "rowheader",
              "rect": [
                1,
                4,
                58,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "rect": [
                    1,
                    4,
                    1,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "SERVICE",
                  "rect": [
                    3,
                    4,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "STATE",
                  "rect": [
                    20,
                    4,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "P99",
                  "rect": [
                    31,
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
              "states": [
                "selected"
              ],
              "children": [
                {
                  "role": "cell",
                  "value": ">",
                  "rect": [
                    1,
                    5,
                    1,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "api-gateway",
                  "rect": [
                    3,
                    5,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "running",
                  "rect": [
                    20,
                    5,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "12ms",
                  "rect": [
                    31,
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
                  "rect": [
                    1,
                    6,
                    1,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "auth-service",
                  "rect": [
                    3,
                    6,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "running",
                  "rect": [
                    20,
                    6,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "31ms",
                  "rect": [
                    31,
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
              "children": [
                {
                  "role": "cell",
                  "rect": [
                    1,
                    7,
                    1,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "billing",
                  "rect": [
                    3,
                    7,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "degraded",
                  "rect": [
                    20,
                    7,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "412ms",
                  "rect": [
                    31,
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
                  "rect": [
                    1,
                    8,
                    1,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "search-index",
                  "rect": [
                    3,
                    8,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "running",
                  "rect": [
                    20,
                    8,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "88ms",
                  "rect": [
                    31,
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
                58,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "rect": [
                    1,
                    9,
                    1,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "mailer",
                  "rect": [
                    3,
                    9,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "stopped",
                  "rect": [
                    20,
                    9,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "-",
                  "rect": [
                    31,
                    9,
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
                10,
                58,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "rect": [
                    1,
                    10,
                    1,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "scheduler",
                  "rect": [
                    3,
                    10,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "running",
                  "rect": [
                    20,
                    10,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "5ms",
                  "rect": [
                    31,
                    10,
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
          "name": "events",
          "rect": [
            60,
            3,
            40,
            23
          ],
          "children": [
            {
              "role": "listitem",
              "value": "deploy started",
              "rect": [
                61,
                4,
                38,
                1
              ]
            },
            {
              "role": "listitem",
              "value": "image pulled",
              "rect": [
                61,
                5,
                38,
                1
              ]
            },
            {
              "role": "listitem",
              "value": "health check ok",
              "rect": [
                61,
                6,
                38,
                1
              ]
            },
            {
              "role": "listitem",
              "value": "traffic shifted 10%",
              "rect": [
                61,
                7,
                38,
                1
              ]
            },
            {
              "role": "listitem",
              "value": "traffic shifted 50%",
              "rect": [
                61,
                8,
                38,
                1
              ]
            }
          ]
        },
        {
          "role": "progressbar",
          "name": "rollout",
          "value": "50%",
          "rect": [
            60,
            26,
            40,
            3
          ]
        }
      ]
    },
    {
      "role": "statusbar",
      "value": "q quit  j/k move  ENTER select",
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

