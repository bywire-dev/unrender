# legacy/ptk

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **47%** (chrome-excluded; 208 non-blank content cells)

## Before (color-stripped)

```
┌──────────────────────────────────────────────────────────────────────────────────────────────────┐
│Deploy Console :: cluster prod-eu-1                                                               │
└──────────────────────────────────────────────────────────────────────────────────────────────────┘
┌─────────────| services |─────────────┐┌────────────────────────| detail |────────────────────────┐
│(*) api-gateway   running            ^││name:    api-gateway                                      │
│( ) auth-service  running             ││state:   running                                          │
│( ) billing       degraded            ││region:  eu-1                                             │
│( ) search-index  running             ││replicas: 3                                               │
│( ) mailer        stopped            v││                                                          │
└──────────────────────────────────────┘└──────────────────────────────────────────────────────────┘
 j/k move   q quit
```

## After — compact

```
application @0,0,100,30
  text @0,0,100,3 Deploy Console :: cluster prod-eu-1
  panel "services | | detail" @0,3,100,7
    table "services" @0,3,40,7
      row [selected] @1,4,38,1
        cell @1,4,3,1 (*)
        cell @5,4,12,1 api-gateway
        cell @19,4,8,1 running
        cell @38,4,1,1 ^
      row @1,5,38,1
        cell @1,5,3,1 ( )
        cell @5,5,12,1 auth-service
        cell @19,5,8,1 running
        cell @38,5,1,1
      row @1,6,38,1
        cell @1,6,3,1 ( )
        cell @5,6,12,1 billing
        cell @19,6,8,1 degraded
        cell @38,6,1,1
      row @1,7,38,1
        cell @1,7,3,1 ( )
        cell @5,7,12,1 search-index
        cell @19,7,8,1 running
        cell @38,7,1,1
      row @1,8,38,1
        cell @1,8,3,1 ( )
        cell @5,8,12,1 mailer
        cell @19,8,8,1 stopped
        cell @38,8,1,1 v
    list "detail" @40,3,60,7
      property "name" @41,4,58,1 api-gateway
      property "state" @41,5,58,1 running
      property "region" @41,6,58,1 eu-1
      property "replicas" @41,7,58,1 3
  statusbar @0,10,100,1 j/k move   q quit
```

## After — toon

```
application
  text: Deploy Console :: cluster prod-eu-1
  panel services | | detail
    table services[5]{c0,c1,c2,c3}:
      *(*),api-gateway,running,^
       ( ),auth-service,running,
       ( ),billing,degraded,
       ( ),search-index,running,
       ( ),mailer,stopped,v
    list detail[4]:
       api-gateway
       running
       eu-1
       3
  statusbar: j/k move   q quit
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
      "value": "Deploy Console :: cluster prod-eu-1",
      "rect": [
        0,
        0,
        100,
        3
      ]
    },
    {
      "role": "panel",
      "name": "services | | detail",
      "rect": [
        0,
        3,
        100,
        7
      ],
      "children": [
        {
          "role": "table",
          "name": "services",
          "rect": [
            0,
            3,
            40,
            7
          ],
          "children": [
            {
              "role": "row",
              "rect": [
                1,
                4,
                38,
                1
              ],
              "states": [
                "selected"
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "(*)",
                  "rect": [
                    1,
                    4,
                    3,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "api-gateway",
                  "rect": [
                    5,
                    4,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "running",
                  "rect": [
                    19,
                    4,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "^",
                  "rect": [
                    38,
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
                1,
                5,
                38,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "( )",
                  "rect": [
                    1,
                    5,
                    3,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "auth-service",
                  "rect": [
                    5,
                    5,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "running",
                  "rect": [
                    19,
                    5,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "rect": [
                    38,
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
                1,
                6,
                38,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "( )",
                  "rect": [
                    1,
                    6,
                    3,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "billing",
                  "rect": [
                    5,
                    6,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "degraded",
                  "rect": [
                    19,
                    6,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "rect": [
                    38,
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
                1,
                7,
                38,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "( )",
                  "rect": [
                    1,
                    7,
                    3,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "search-index",
                  "rect": [
                    5,
                    7,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "running",
                  "rect": [
                    19,
                    7,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "rect": [
                    38,
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
                1,
                8,
                38,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "( )",
                  "rect": [
                    1,
                    8,
                    3,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "mailer",
                  "rect": [
                    5,
                    8,
                    12,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "stopped",
                  "rect": [
                    19,
                    8,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "v",
                  "rect": [
                    38,
                    8,
                    1,
                    1
                  ]
                }
              ]
            }
          ]
        },
        {
          "role": "list",
          "name": "detail",
          "rect": [
            40,
            3,
            60,
            7
          ],
          "children": [
            {
              "role": "property",
              "name": "name",
              "value": "api-gateway",
              "rect": [
                41,
                4,
                58,
                1
              ]
            },
            {
              "role": "property",
              "name": "state",
              "value": "running",
              "rect": [
                41,
                5,
                58,
                1
              ]
            },
            {
              "role": "property",
              "name": "region",
              "value": "eu-1",
              "rect": [
                41,
                6,
                58,
                1
              ]
            },
            {
              "role": "property",
              "name": "replicas",
              "value": "3",
              "rect": [
                41,
                7,
                58,
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
        10,
        100,
        1
      ]
    }
  ]
}
```

</details>

