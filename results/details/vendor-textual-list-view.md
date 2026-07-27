# vendor/textual/list-view

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **0%** (chrome-excluded; 11 non-blank content cells)

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

