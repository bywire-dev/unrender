# field/man

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **100%** (chrome-excluded; 228 non-blank content cells)

## Sizes

| encoding | bytes | tokens | vs plain (bytes) | vs plain (tokens) |
|---|---:|---:|---:|---:|
| raw | 291 | 97 | 0.95x | 0.84x |
| plain | 275 | 81 | 1.00x | 1.00x |
| compact | 330 | 118 | 0.83x | 0.69x |
| nogeo | 300 | 92 | 0.92x | 0.88x |
| toon | 302 | 94 | 0.91x | 0.86x |
| json | 608 | 226 | 0.45x | 0.36x |

<details><summary>Before — raw ANSI (291 bytes, escapes shown literally)</summary>

```
\x1b[mThis system has been minimized by removing packages and content that are
\x1b[mnot required on a system that users do not log into.

\x1b[mTo restore this content, including manpages, you can run the 'unminimize'
\x1b[mcommand. You will still need to ensure the 'man-db' package is installed.
\x1b[m
```

</details>

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

