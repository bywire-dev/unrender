# legacy/dialog-moved

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **71%** (chrome-excluded; 90 non-blank content cells)

## Before (color-stripped)

```
                   ┌──────────────────────────────────────────────────────────┐                     
                   │ Choose a service                                         │                     
                   │ ┌──────────────────────────────────────────────────────┐ │                     
                   │ │                  api   API Gateway                   │ │                     
                   │ │                  auth  Auth Service                  │ │                     
                   │ │                  bill  Billing                       │ │                     
                   │ │                  srch  Search Index                  │ │                     
                   │ │                  mail  Mailer                        │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ │                                                      │ │                     
                   │ └──────────────────────────────────────────────────────┘ │                     
                   ├──────────────────────────────────────────────────────────┤                     
                   │               <  OK  >        <Cancel>                   │                     
                   └──────────────────────────────────────────────────────────┘
```

## After — compact

```
application @0,0,100,30
  table @19,0,60,27
    rowheader @22,3,54,1
      cell @40,3,4,1 api
      cell @46,3,12,1 API Gateway
    row [selected] @22,4,54,1
      cell @40,4,4,1 auth
      cell @46,4,12,1 Auth Service
    row @22,5,54,1
      cell @40,5,4,1 bill
      cell @46,5,12,1 Billing
    row @22,6,54,1
      cell @40,6,4,1 srch
      cell @46,6,12,1 Search Index
    row @22,7,54,1
      cell @40,7,4,1 mail
      cell @46,7,12,1 Mailer
  text @19,26,60,3 <  OK  >        <Cancel>
```

## After — toon

```
application
  table [4]{api,API Gateway}:
    *auth,Auth Service
     bill,Billing
     srch,Search Index
     mail,Mailer
  text: <  OK  >        <Cancel>
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
        19,
        0,
        60,
        27
      ],
      "children": [
        {
          "role": "rowheader",
          "rect": [
            22,
            3,
            54,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "api",
              "rect": [
                40,
                3,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "API Gateway",
              "rect": [
                46,
                3,
                12,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            22,
            4,
            54,
            1
          ],
          "states": [
            "selected"
          ],
          "children": [
            {
              "role": "cell",
              "value": "auth",
              "rect": [
                40,
                4,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "Auth Service",
              "rect": [
                46,
                4,
                12,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            22,
            5,
            54,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "bill",
              "rect": [
                40,
                5,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "Billing",
              "rect": [
                46,
                5,
                12,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            22,
            6,
            54,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "srch",
              "rect": [
                40,
                6,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "Search Index",
              "rect": [
                46,
                6,
                12,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            22,
            7,
            54,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "mail",
              "rect": [
                40,
                7,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "Mailer",
              "rect": [
                46,
                7,
                12,
                1
              ]
            }
          ]
        }
      ]
    },
    {
      "role": "text",
      "value": "<  OK  >        <Cancel>",
      "rect": [
        19,
        26,
        60,
        3
      ]
    }
  ]
}
```

</details>

