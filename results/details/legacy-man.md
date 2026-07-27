# legacy/man

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **100%** (chrome-excluded; 228 non-blank content cells)

## Before (color-stripped)

```
This system has been minimized by removing packages and content that are
not required on a system that users do not log into.

To restore this content, including manpages, you can run the 'unminimize'
command. You will still need to ensure the 'man-db' package is installed.
```

## After — compact

```
application @0,0,73,6
  log @0,0,73,2 This system has been minimized by removing packages and content that are\nnot required on a system that users do not log into.
  log @0,3,73,2 To restore this content, including manpages, you can run the 'unminimize'\ncommand. You will still need to ensure the 'man-db' package is installed.
```

## After — toon

```
application
  log: This system has been minimized by removing packages and content that are\nnot required on a system that users do not log into.
  log: To restore this content, including manpages, you can run the 'unminimize'\ncommand. You will still need to ensure the 'man-db' package is installed.
```

<details><summary>After — json</summary>

```json
{
  "role": "application",
  "rect": [
    0,
    0,
    73,
    6
  ],
  "children": [
    {
      "role": "log",
      "value": "This system has been minimized by removing packages and content that are\nnot required on a system that users do not log into.",
      "rect": [
        0,
        0,
        73,
        2
      ]
    },
    {
      "role": "log",
      "value": "To restore this content, including manpages, you can run the 'unminimize'\ncommand. You will still need to ensure the 'man-db' package is installed.",
      "rect": [
        0,
        3,
        73,
        2
      ]
    }
  ]
}
```

</details>

