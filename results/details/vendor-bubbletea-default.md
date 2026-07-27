# vendor/bubbletea/default

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **66%** (chrome-excluded; 184 non-blank content cells)

## Before (color-stripped)

```
╭────────────────────────────────────╮╭──────────────────────────────╮
│ SERVICE        STATE      REGION   ││ name:    api-gateway         │
│ api-gateway    running    eu-1     ││ state:   running             │
│ auth-service   running    eu-1     ││ region:  eu-1                │
│ billing        degraded   us-1     ││ replicas: 3                  │
│ search-index   running    us-1     │╰──────────────────────────────╯
│ mailer         stopped    eu-1     │
╰────────────────────────────────────╯
 j/k move   q quit
```

## After — compact

```
application @0,0,78,10
  table @0,0,38,8
    rowheader @1,1,36,1
      cell @2,1,12,1 SERVICE
      cell @17,1,8,1 STATE
      cell @28,1,6,1 REGION
    row [selected] @1,2,36,1
      cell @2,2,12,1 api-gateway
      cell @17,2,8,1 running
      cell @28,2,6,1 eu-1
    row @1,3,36,1
      cell @2,3,12,1 auth-service
      cell @17,3,8,1 running
      cell @28,3,6,1 eu-1
    row @1,4,36,1
      cell @2,4,12,1 billing
      cell @17,4,8,1 degraded
      cell @28,4,6,1 us-1
    row @1,5,36,1
      cell @2,5,12,1 search-index
      cell @17,5,8,1 running
      cell @28,5,6,1 us-1
    row @1,6,36,1
      cell @2,6,12,1 mailer
      cell @17,6,8,1 stopped
      cell @28,6,6,1 eu-1
  list @38,0,32,6
    property "name" @39,1,30,1 api-gateway
    property "state" @39,2,30,1 running
    property "region" @39,3,30,1 eu-1
    property "replicas" @39,4,30,1 3
  statusbar @0,8,78,1 j/k move   q quit
```

## After — toon

```
application
  table [5]{SERVICE,STATE,REGION}:
    *api-gateway,running,eu-1
     auth-service,running,eu-1
     billing,degraded,us-1
     search-index,running,us-1
     mailer,stopped,eu-1
  list [4]:
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
    78,
    10
  ],
  "children": [
    {
      "role": "table",
      "rect": [
        0,
        0,
        38,
        8
      ],
      "children": [
        {
          "role": "rowheader",
          "rect": [
            1,
            1,
            36,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "SERVICE",
              "rect": [
                2,
                1,
                12,
                1
              ]
            },
            {
              "role": "cell",
              "value": "STATE",
              "rect": [
                17,
                1,
                8,
                1
              ]
            },
            {
              "role": "cell",
              "value": "REGION",
              "rect": [
                28,
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
            1,
            2,
            36,
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
                2,
                2,
                12,
                1
              ]
            },
            {
              "role": "cell",
              "value": "running",
              "rect": [
                17,
                2,
                8,
                1
              ]
            },
            {
              "role": "cell",
              "value": "eu-1",
              "rect": [
                28,
                2,
                6,
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
            36,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "auth-service",
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
                17,
                3,
                8,
                1
              ]
            },
            {
              "role": "cell",
              "value": "eu-1",
              "rect": [
                28,
                3,
                6,
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
            36,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "billing",
              "rect": [
                2,
                4,
                12,
                1
              ]
            },
            {
              "role": "cell",
              "value": "degraded",
              "rect": [
                17,
                4,
                8,
                1
              ]
            },
            {
              "role": "cell",
              "value": "us-1",
              "rect": [
                28,
                4,
                6,
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
            36,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "search-index",
              "rect": [
                2,
                5,
                12,
                1
              ]
            },
            {
              "role": "cell",
              "value": "running",
              "rect": [
                17,
                5,
                8,
                1
              ]
            },
            {
              "role": "cell",
              "value": "us-1",
              "rect": [
                28,
                5,
                6,
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
            36,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "mailer",
              "rect": [
                2,
                6,
                12,
                1
              ]
            },
            {
              "role": "cell",
              "value": "stopped",
              "rect": [
                17,
                6,
                8,
                1
              ]
            },
            {
              "role": "cell",
              "value": "eu-1",
              "rect": [
                28,
                6,
                6,
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
        38,
        0,
        32,
        6
      ],
      "children": [
        {
          "role": "property",
          "name": "name",
          "value": "api-gateway",
          "rect": [
            39,
            1,
            30,
            1
          ]
        },
        {
          "role": "property",
          "name": "state",
          "value": "running",
          "rect": [
            39,
            2,
            30,
            1
          ]
        },
        {
          "role": "property",
          "name": "region",
          "value": "eu-1",
          "rect": [
            39,
            3,
            30,
            1
          ]
        },
        {
          "role": "property",
          "name": "replicas",
          "value": "3",
          "rect": [
            39,
            4,
            30,
            1
          ]
        }
      ]
    },
    {
      "role": "statusbar",
      "value": "j/k move   q quit",
      "rect": [
        0,
        8,
        78,
        1
      ]
    }
  ]
}
```

</details>

