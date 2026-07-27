# vendor/textual/progress-bar

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **0%** (chrome-excluded; 11 non-blank content cells)

## Before (color-stripped)

```
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                                                                                
                 ━━━━━━━━━━━━╺━━━━━━━━━━━━━━━━━━━  39% 00:00:07
```

## After — compact

```
application @0,0,80,23
  statusbar @0,11,80,1 ━━━━━━━━━━━━╺━━━━━━━━━━━━━━━━━━━  39% 00:00:07
```

## After — toon

```
application
  statusbar: ━━━━━━━━━━━━╺━━━━━━━━━━━━━━━━━━━  39% 00:00:07
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
      "value": "━━━━━━━━━━━━╺━━━━━━━━━━━━━━━━━━━  39% 00:00:07",
      "rect": [
        0,
        11,
        80,
        1
      ]
    }
  ]
}
```

</details>

