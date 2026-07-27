# vendor/textual/log

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **100%** (chrome-excluded; 24 non-blank content cells)

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

