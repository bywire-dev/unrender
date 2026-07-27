# field/dialog

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **71%** (chrome-excluded; 90 non-blank content cells)

## Sizes

| encoding | bytes | tokens | vs plain (bytes) | vs plain (tokens) |
|---|---:|---:|---:|---:|
| raw | 16078 | 8000 | 0.24x | 0.07x |
| plain | 3805 | 533 | 1.00x | 1.00x |
| compact | 486 | 270 | 7.83x | 1.97x |
| toon | 167 | 84 | 22.78x | 6.35x |
| json | 3401 | 939 | 1.12x | 0.57x |

<details><summary>Before — raw ANSI (16078 bytes, escapes shown literally)</summary>

```
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m┌──────────────────────────────────────────────────────────\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m┐\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                     
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m Choose a service                                         │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m ┌──────────────────────────────────────────────────────\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m┐\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │                  \x1b[31m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23ma\x1b[33m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mpi\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m   \x1b[37m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mAPI Gateway\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │                  \x1b[31m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23ma\x1b[34m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23muth\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  Auth Service                  \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │                  \x1b[31m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mb\x1b[34m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mill\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  Billing                       \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │                  \x1b[31m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23ms\x1b[34m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mrch\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  Search Index                  \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │                  \x1b[31m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mm\x1b[34m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mail\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  Mailer                        \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                      │\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m └\x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m──────────────────────────────────────────────────────┘\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m├──────────────────────────────────────────────────────────\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m┤\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m│\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m               \x1b[37m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m<\x1b[33m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  OK  \x1b[37m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m>\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m        <\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mCancel\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m>                   │\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[37m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m└\x1b[30m\x1b[47m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m──────────────────────────────────────────────────────────┘\x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   
\x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                     \x1b[30m\x1b[40m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                                            \x1b[39m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[m
```

</details>

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
    row [selected] @22,3,54,1
      cell @40,3,4,1 api
      cell @46,3,12,1 API Gateway
    row @22,4,54,1
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
  table [5]{c0,c1}:
    *api,API Gateway
     auth,Auth Service
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
          "role": "row",
          "rect": [
            22,
            3,
            54,
            1
          ],
          "states": [
            "selected"
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

