# field/htop

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **70%** (chrome-excluded; 1410 non-blank content cells)

## Sizes

| encoding | bytes | tokens | vs plain (bytes) | vs plain (tokens) |
|---|---:|---:|---:|---:|
| raw | 11794 | 6978 | 0.19x | 0.17x |
| plain | 2292 | 1166 | 1.00x | 1.00x |
| compact | 8273 | 4673 | 0.28x | 0.25x |
| nogeo | 5052 | 2229 | 0.45x | 0.52x |
| toon | 2012 | 1195 | 1.14x | 0.98x |
| json | 60256 | 15073 | 0.04x | 0.08x |

<details><summary>Before — raw ANSI (11794 bytes, escapes shown literally)</summary>

```

\x1b[m  \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  0\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m[\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m|||||||\x1b[31m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m||\x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                            18.1%\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m]\x1b[m \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mTasks: \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m100\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m, \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m318\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m thr\x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m, 133 kthr\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m; \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m1\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m running
\x1b[m  \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  1\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m[\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m||||\x1b[31m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m||\x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                               12.4%\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m]\x1b[m \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mLoad average: \x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m1.13 \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m1.10 \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m0.97
\x1b[m  \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  2\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m[\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m||||||\x1b[31m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m||\x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                             16.9%\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m]\x1b[m \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mUptime: \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m07:21:05
\x1b[m  \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  3\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m[\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m|||||\x1b[31m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m||\x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                              14.5%\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m]
\x1b[m  \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mMem\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m[\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m||||||||||\x1b[35m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m|||\x1b[34m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m|\x1b[33m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m|||||||||||||||||2.09G/7.62G\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m]
\x1b[m  \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mSwp\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m[\x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m                                  0K/7.62G\x1b[39m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m]

\x1b[m  \x1b[32m\x1b[42m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m[\x1b[30m\x1b[42m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mMain\x1b[32m\x1b[42m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m]\x1b[m \x1b[34m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m[\x1b[30m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mI/O\x1b[34m\x1b[44m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m]
\x1b[30m\x1b[42m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m    PID USER       PRI  NI  VIRT   RES   SHR S \x1b[30m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m CPU%▽\x1b[30m\x1b[42m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mMEM%   TIME+  Command\x1b[39m\x1b[42m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                          
\x1b[30m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m  38532 jake        20   0 5882M  615M  149M S  23.5  7.9 19:15.70 claude --teleport\x1b[39m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                
\x1b[m   2639 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m2\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m852M  187M 93\x1b[m664 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  4.7  2.4  7:45.83 ghostty
\x1b[m  20635 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m2\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m852M  187M \x1b[m    0 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  3.8  2.4  3:28.99 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mghostty
\x1b[m 541338 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m5\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m882M  614M \x1b[m    0 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  2.6  7.9  0:50.18 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mclaude --teleport
\x1b[m 541339 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m5\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m882M  614M \x1b[m    0 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  2.6  7.9  0:49.58 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mclaude --teleport
\x1b[m 541340 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m5\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m882M  614M \x1b[m    0 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  2.1  7.9  0:48.21 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mclaude --teleport
\x1b[m   2617 \x1b[35m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mjake       \x1b[m -2 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m 782M 83\x1b[m196 \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m59\x1b[m740 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  1.7  1.0  3:42.61 sway
\x1b[m  38533 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m5\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m882M  614M \x1b[m    0 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  1.3  7.9  1:35.15 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mclaude --teleport
\x1b[m1052694 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m 121M 66\x1b[m068     0 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  1.3  0.8  0:00.06 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m/home/jake/.local/bin/zellij --se
\x1b[m1052818 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m 229M  6\x1b[m016 \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m 4\x1b[m584 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mR \x1b[m  1.3  0.1  0:00.11 htop
\x1b[m1052917 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[31m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m1\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m450G \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m98\x1b[m028 \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m74\x1b[m524 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  1.3  1.2  0:00.03 /opt/google/chrome/chrome --type=
\x1b[m   1092 \x1b[35m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mroot       \x1b[m 20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m1\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m322M 47\x1b[m016     0 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  0.4  0.6  0:00.76 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m/usr/sbin/tailscaled --state=/var
\x1b[m 946307 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m1\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m269M 56\x1b[m784     0 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  0.4  0.7  0:01.60 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mwaybar -b bar-0
\x1b[m 946309 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m1\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m269M 56\x1b[m784     0 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  0.4  0.7  0:01.59 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mwaybar -b bar-0
\x1b[m 953619 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m53\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m.4\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mG \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m 379M  261M \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  0.4  4.9  0:11.98 /opt/google/chrome/chrome
\x1b[m 953673 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m52\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m.6\x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mG \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m 132M \x1b[m    0 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  0.4  1.7  0:03.44 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m/opt/google/chrome/chrome --type=
\x1b[m1010921 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m5\x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m882M  614M \x1b[m    0 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  0.4  7.9  0:01.62 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mclaude --teleport
\x1b[m1052690 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m 120M 64\x1b[m908 \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m33\x1b[m188 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  0.4  0.8  0:00.07 /home/jake/.local/bin/zellij --se
\x1b[m1052706 jake        20 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23m  0 \x1b[36m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m 121M 66\x1b[m068     0 \x1b[90m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[1m\x1b[24m\x1b[22m\x1b[1m\x1b[23mS \x1b[m  0.4  0.8  0:00.06 \x1b[32m\x1b[49m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m/home/jake/.local/bin/zellij --se
\x1b[mF1\x1b[30m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mHelp  \x1b[mF2\x1b[30m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mSetup \x1b[mF3\x1b[30m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mSearch\x1b[mF4\x1b[30m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mFilter\x1b[mF5\x1b[30m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mTree  \x1b[mF6\x1b[30m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mSortBy\x1b[mF7\x1b[30m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mNice -\x1b[mF8\x1b[30m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mNice +\x1b[mF9\x1b[30m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mKill  \x1b[mF10\x1b[30m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23mQuit\x1b[39m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m \x1b[30m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m \x1b[39m\x1b[46m\x1b[29m\x1b[28m\x1b[27m\x1b[25m\x1b[25m\x1b[22m\x1b[24m\x1b[22m\x1b[23m                   \x1b[m
```

</details>

## Before (color-stripped)

```

    0[|||||||||                            18.1%] Tasks: 100, 318 thr, 133 kthr; 1 running
    1[||||||                               12.4%] Load average: 1.13 1.10 0.97
    2[||||||||                             16.9%] Uptime: 07:21:05
    3[|||||||                              14.5%]
  Mem[|||||||||||||||||||||||||||||||2.09G/7.62G]
  Swp[                                  0K/7.62G]

  [Main] [I/O]
    PID USER       PRI  NI  VIRT   RES   SHR S  CPU%▽MEM%   TIME+  Command                          
  38532 jake        20   0 5882M  615M  149M S  23.5  7.9 19:15.70 claude --teleport                
   2639 jake        20   0 2852M  187M 93664 S   4.7  2.4  7:45.83 ghostty
  20635 jake        20   0 2852M  187M     0 S   3.8  2.4  3:28.99 ghostty
 541338 jake        20   0 5882M  614M     0 S   2.6  7.9  0:50.18 claude --teleport
 541339 jake        20   0 5882M  614M     0 S   2.6  7.9  0:49.58 claude --teleport
 541340 jake        20   0 5882M  614M     0 S   2.1  7.9  0:48.21 claude --teleport
   2617 jake        -2   0  782M 83196 59740 S   1.7  1.0  3:42.61 sway
  38533 jake        20   0 5882M  614M     0 S   1.3  7.9  1:35.15 claude --teleport
1052694 jake        20   0  121M 66068     0 S   1.3  0.8  0:00.06 /home/jake/.local/bin/zellij --se
1052818 jake        20   0  229M  6016  4584 R   1.3  0.1  0:00.11 htop
1052917 jake        20   0 1450G 98028 74524 S   1.3  1.2  0:00.03 /opt/google/chrome/chrome --type=
   1092 root        20   0 1322M 47016     0 S   0.4  0.6  0:00.76 /usr/sbin/tailscaled --state=/var
 946307 jake        20   0 1269M 56784     0 S   0.4  0.7  0:01.60 waybar -b bar-0
 946309 jake        20   0 1269M 56784     0 S   0.4  0.7  0:01.59 waybar -b bar-0
 953619 jake        20   0 53.4G  379M  261M S   0.4  4.9  0:11.98 /opt/google/chrome/chrome
 953673 jake        20   0 52.6G  132M     0 S   0.4  1.7  0:03.44 /opt/google/chrome/chrome --type=
1010921 jake        20   0 5882M  614M     0 S   0.4  7.9  0:01.62 claude --teleport
1052690 jake        20   0  120M 64908 33188 S   0.4  0.8  0:00.07 /home/jake/.local/bin/zellij --se
1052706 jake        20   0  121M 66068     0 S   0.4  0.8  0:00.06 /home/jake/.local/bin/zellij --se
F1Help  F2Setup F3SearchF4FilterF5Tree  F6SortByF7Nice -F8Nice +F9Kill  F10Quit
```

## After — compact

```
application @0,0,100,30
  list @0,1,100,6
    listitem @0,1,100,1 0[|||||||||                            18.1%] Tasks: 100, 318 thr, 133 kthr; 1 running
    listitem @0,2,100,1 1[||||||                               12.4%] Load average: 1.13 1.10 0.97
    listitem @0,3,100,1 2[||||||||                             16.9%] Uptime: 07:21:05
    listitem @0,4,100,1 3[|||||||                              14.5%]
    listitem @0,5,100,1 Mem[|||||||||||||||||||||||||||||||2.09G/7.62G]
    listitem @0,6,100,1 Swp[                                  0K/7.62G]
  table @0,8,100,22
    rowheader @0,8,100,1
      cell @0,8,7,1 [Main
      cell @8,8,4,1 [I/
      cell @20,8,2,1
      cell @25,8,1,1
      cell @27,8,5,1
      cell @33,8,5,1
      cell @39,8,5,1
      cell @45,8,1,1
      cell @48,8,4,1
      cell @54,8,3,1
      cell @59,8,7,1
      cell @67,8,33,1
    row [selected] @0,9,100,1
      cell @0,9,7,1 PID
      cell @8,9,4,1 USER
      cell @20,9,2,1 RI
      cell @25,9,1,1 I
      cell @27,9,5,1 VIRT
      cell @33,9,5,1 RES
      cell @39,9,5,1 SHR
      cell @45,9,1,1 S
      cell @48,9,4,1 CPU%
      cell @54,9,3,1 EM%
      cell @59,9,7,1 TIME+
      cell @67,9,33,1 Command
    row [selected] @0,10,100,1
      cell @0,10,7,1 38532
      cell @8,10,4,1 jake
      cell @20,10,2,1 20
      cell @25,10,1,1 0
      cell @27,10,5,1 5882M
      cell @33,10,5,1 615M
      cell @39,10,5,1 149M
      cell @45,10,1,1 S
      cell @48,10,4,1 23.5
      cell @54,10,3,1 7.9
      cell @59,10,7,1 9:15.70
      cell @67,10,33,1 claude --teleport
    row @0,11,100,1
      cell @0,11,7,1 2639
      cell @8,11,4,1 jake
      cell @20,11,2,1 20
      cell @25,11,1,1 0
      cell @27,11,5,1 2852M
      cell @33,11,5,1 187M
      cell @39,11,5,1 93664
      cell @45,11,1,1 S
      cell @48,11,4,1 4.7
      cell @54,11,3,1 2.4
      cell @59,11,7,1 7:45.83
      cell @67,11,33,1 ghostty
    row @0,12,100,1
      cell @0,12,7,1 20635
      cell @8,12,4,1 jake
      cell @20,12,2,1 20
      cell @25,12,1,1 0
      cell @27,12,5,1 2852M
      cell @33,12,5,1 187M
      cell @39,12,5,1 0
      cell @45,12,1,1 S
      cell @48,12,4,1 3.8
      cell @54,12,3,1 2.4
      cell @59,12,7,1 3:28.99
      cell @67,12,33,1 ghostty
    row @0,13,100,1
      cell @0,13,7,1 541338
      cell @8,13,4,1 jake
      cell @20,13,2,1 20
      cell @25,13,1,1 0
      cell @27,13,5,1 5882M
      cell @33,13,5,1 614M
      cell @39,13,5,1 0
      cell @45,13,1,1 S
      cell @48,13,4,1 2.6
      cell @54,13,3,1 7.9
      cell @59,13,7,1 0:50.18
      cell @67,13,33,1 claude --teleport
    row @0,14,100,1
      cell @0,14,7,1 541339
      cell @8,14,4,1 jake
      cell @20,14,2,1 20
      cell @25,14,1,1 0
      cell @27,14,5,1 5882M
      cell @33,14,5,1 614M
      cell @39,14,5,1 0
      cell @45,14,1,1 S
      cell @48,14,4,1 2.6
      cell @54,14,3,1 7.9
      cell @59,14,7,1 0:49.58
      cell @67,14,33,1 claude --teleport
    row @0,15,100,1
      cell @0,15,7,1 541340
      cell @8,15,4,1 jake
      cell @20,15,2,1 20
      cell @25,15,1,1 0
      cell @27,15,5,1 5882M
      cell @33,15,5,1 614M
      cell @39,15,5,1 0
      cell @45,15,1,1 S
      cell @48,15,4,1 2.1
      cell @54,15,3,1 7.9
      cell @59,15,7,1 0:48.21
      cell @67,15,33,1 claude --teleport
    row @0,16,100,1
      cell @0,16,7,1 2617
      cell @8,16,4,1 jake
      cell @20,16,2,1 -2
      cell @25,16,1,1 0
      cell @27,16,5,1 782M
      cell @33,16,5,1 83196
      cell @39,16,5,1 59740
      cell @45,16,1,1 S
      cell @48,16,4,1 1.7
      cell @54,16,3,1 1.0
      cell @59,16,7,1 3:42.61
      cell @67,16,33,1 sway
    row @0,17,100,1
      cell @0,17,7,1 38533
      cell @8,17,4,1 jake
      cell @20,17,2,1 20
      cell @25,17,1,1 0
      cell @27,17,5,1 5882M
      cell @33,17,5,1 614M
      cell @39,17,5,1 0
      cell @45,17,1,1 S
      cell @48,17,4,1 1.3
      cell @54,17,3,1 7.9
      cell @59,17,7,1 1:35.15
      cell @67,17,33,1 claude --teleport
    row @0,18,100,1
      cell @0,18,7,1 1052694
      cell @8,18,4,1 jake
      cell @20,18,2,1 20
      cell @25,18,1,1 0
      cell @27,18,5,1 121M
      cell @33,18,5,1 66068
      cell @39,18,5,1 0
      cell @45,18,1,1 S
      cell @48,18,4,1 1.3
      cell @54,18,3,1 0.8
      cell @59,18,7,1 0:00.06
      cell @67,18,33,1 /home/jake/.local/bin/zellij --se
    row @0,19,100,1
      cell @0,19,7,1 1052818
      cell @8,19,4,1 jake
      cell @20,19,2,1 20
      cell @25,19,1,1 0
      cell @27,19,5,1 229M
      cell @33,19,5,1 6016
      cell @39,19,5,1 4584
      cell @45,19,1,1 R
      cell @48,19,4,1 1.3
      cell @54,19,3,1 0.1
      cell @59,19,7,1 0:00.11
      cell @67,19,33,1 htop
    row @0,20,100,1
      cell @0,20,7,1 1052917
      cell @8,20,4,1 jake
      cell @20,20,2,1 20
      cell @25,20,1,1 0
      cell @27,20,5,1 1450G
      cell @33,20,5,1 98028
      cell @39,20,5,1 74524
      cell @45,20,1,1 S
      cell @48,20,4,1 1.3
      cell @54,20,3,1 1.2
      cell @59,20,7,1 0:00.03
      cell @67,20,33,1 /opt/google/chrome/chrome --type=
    row @0,21,100,1
      cell @0,21,7,1 1092
      cell @8,21,4,1 root
      cell @20,21,2,1 20
      cell @25,21,1,1 0
      cell @27,21,5,1 1322M
      cell @33,21,5,1 47016
      cell @39,21,5,1 0
      cell @45,21,1,1 S
      cell @48,21,4,1 0.4
      cell @54,21,3,1 0.6
      cell @59,21,7,1 0:00.76
      cell @67,21,33,1 /usr/sbin/tailscaled --state=/var
    row @0,22,100,1
      cell @0,22,7,1 946307
      cell @8,22,4,1 jake
      cell @20,22,2,1 20
      cell @25,22,1,1 0
      cell @27,22,5,1 1269M
      cell @33,22,5,1 56784
      cell @39,22,5,1 0
      cell @45,22,1,1 S
      cell @48,22,4,1 0.4
      cell @54,22,3,1 0.7
      cell @59,22,7,1 0:01.60
      cell @67,22,33,1 waybar -b bar-0
    row @0,23,100,1
      cell @0,23,7,1 946309
      cell @8,23,4,1 jake
      cell @20,23,2,1 20
      cell @25,23,1,1 0
      cell @27,23,5,1 1269M
      cell @33,23,5,1 56784
      cell @39,23,5,1 0
      cell @45,23,1,1 S
      cell @48,23,4,1 0.4
      cell @54,23,3,1 0.7
      cell @59,23,7,1 0:01.59
      cell @67,23,33,1 waybar -b bar-0
    row @0,24,100,1
      cell @0,24,7,1 953619
      cell @8,24,4,1 jake
      cell @20,24,2,1 20
      cell @25,24,1,1 0
      cell @27,24,5,1 53.4G
      cell @33,24,5,1 379M
      cell @39,24,5,1 261M
      cell @45,24,1,1 S
      cell @48,24,4,1 0.4
      cell @54,24,3,1 4.9
      cell @59,24,7,1 0:11.98
      cell @67,24,33,1 /opt/google/chrome/chrome
    row @0,25,100,1
      cell @0,25,7,1 953673
      cell @8,25,4,1 jake
      cell @20,25,2,1 20
      cell @25,25,1,1 0
      cell @27,25,5,1 52.6G
      cell @33,25,5,1 132M
      cell @39,25,5,1 0
      cell @45,25,1,1 S
      cell @48,25,4,1 0.4
      cell @54,25,3,1 1.7
      cell @59,25,7,1 0:03.44
      cell @67,25,33,1 /opt/google/chrome/chrome --type=
    row @0,26,100,1
      cell @0,26,7,1 1010921
      cell @8,26,4,1 jake
      cell @20,26,2,1 20
      cell @25,26,1,1 0
      cell @27,26,5,1 5882M
      cell @33,26,5,1 614M
      cell @39,26,5,1 0
      cell @45,26,1,1 S
      cell @48,26,4,1 0.4
      cell @54,26,3,1 7.9
      cell @59,26,7,1 0:01.62
      cell @67,26,33,1 claude --teleport
    row @0,27,100,1
      cell @0,27,7,1 1052690
      cell @8,27,4,1 jake
      cell @20,27,2,1 20
      cell @25,27,1,1 0
      cell @27,27,5,1 120M
      cell @33,27,5,1 64908
      cell @39,27,5,1 33188
      cell @45,27,1,1 S
      cell @48,27,4,1 0.4
      cell @54,27,3,1 0.8
      cell @59,27,7,1 0:00.07
      cell @67,27,33,1 /home/jake/.local/bin/zellij --se
    row @0,28,100,1
      cell @0,28,7,1 1052706
      cell @8,28,4,1 jake
      cell @20,28,2,1 20
      cell @25,28,1,1 0
      cell @27,28,5,1 121M
      cell @33,28,5,1 66068
      cell @39,28,5,1 0
      cell @45,28,1,1 S
      cell @48,28,4,1 0.4
      cell @54,28,3,1 0.8
      cell @59,28,7,1 0:00.06
      cell @67,28,33,1 /home/jake/.local/bin/zellij --se
    row [selected] @0,29,100,1
      cell @0,29,7,1 F1Help
      cell @8,29,4,1 F2Se
      cell @20,29,2,1 ar
      cell @25,29,1,1 4
      cell @27,29,5,1 ilter
      cell @33,29,5,1 5Tree
      cell @39,29,5,1 F6So
      cell @45,29,1,1 t
      cell @48,29,4,1 F7Ni
      cell @54,29,3,1 -F
      cell @59,29,7,1 ice +F9
      cell @67,29,33,1 ill  F10Quit
```

## After — toon

```
application
  list [6]:
     0[|||||||||                            18.1%] Tasks: 100, 318 thr, 133 kthr; 1 running
     1[||||||                               12.4%] Load average: 1.13 1.10 0.97
     2[||||||||                             16.9%] Uptime: 07:21:05
     3[|||||||                              14.5%]
     Mem[|||||||||||||||||||||||||||||||2.09G/7.62G]
     Swp[                                  0K/7.62G]
  table [21]{[Main,[I/,,,,,,,,,,}:
    *PID,USER,RI,I,VIRT,RES,SHR,S,CPU%,EM%,TIME+,Command
    *38532,jake,20,0,5882M,615M,149M,S,23.5,7.9,9:15.70,claude --teleport
     2639,jake,20,0,2852M,187M,93664,S,4.7,2.4,7:45.83,ghostty
     20635,jake,20,0,2852M,187M,0,S,3.8,2.4,3:28.99,ghostty
     541338,jake,20,0,5882M,614M,0,S,2.6,7.9,0:50.18,claude --teleport
     541339,jake,20,0,5882M,614M,0,S,2.6,7.9,0:49.58,claude --teleport
     541340,jake,20,0,5882M,614M,0,S,2.1,7.9,0:48.21,claude --teleport
     2617,jake,-2,0,782M,83196,59740,S,1.7,1.0,3:42.61,sway
     38533,jake,20,0,5882M,614M,0,S,1.3,7.9,1:35.15,claude --teleport
     1052694,jake,20,0,121M,66068,0,S,1.3,0.8,0:00.06,/home/jake/.local/bin/zellij --se
     1052818,jake,20,0,229M,6016,4584,R,1.3,0.1,0:00.11,htop
     1052917,jake,20,0,1450G,98028,74524,S,1.3,1.2,0:00.03,/opt/google/chrome/chrome --type=
     1092,root,20,0,1322M,47016,0,S,0.4,0.6,0:00.76,/usr/sbin/tailscaled --state=/var
     946307,jake,20,0,1269M,56784,0,S,0.4,0.7,0:01.60,waybar -b bar-0
     946309,jake,20,0,1269M,56784,0,S,0.4,0.7,0:01.59,waybar -b bar-0
     953619,jake,20,0,53.4G,379M,261M,S,0.4,4.9,0:11.98,/opt/google/chrome/chrome
     953673,jake,20,0,52.6G,132M,0,S,0.4,1.7,0:03.44,/opt/google/chrome/chrome --type=
     1010921,jake,20,0,5882M,614M,0,S,0.4,7.9,0:01.62,claude --teleport
     1052690,jake,20,0,120M,64908,33188,S,0.4,0.8,0:00.07,/home/jake/.local/bin/zellij --se
     1052706,jake,20,0,121M,66068,0,S,0.4,0.8,0:00.06,/home/jake/.local/bin/zellij --se
    *F1Help,F2Se,ar,4,ilter,5Tree,F6So,t,F7Ni,-F,ice +F9,ill  F10Quit
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
      "role": "list",
      "rect": [
        0,
        1,
        100,
        6
      ],
      "children": [
        {
          "role": "listitem",
          "value": "0[|||||||||                            18.1%] Tasks: 100, 318 thr, 133 kthr; 1 running",
          "rect": [
            0,
            1,
            100,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "1[||||||                               12.4%] Load average: 1.13 1.10 0.97",
          "rect": [
            0,
            2,
            100,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "2[||||||||                             16.9%] Uptime: 07:21:05",
          "rect": [
            0,
            3,
            100,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "3[|||||||                              14.5%]",
          "rect": [
            0,
            4,
            100,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "Mem[|||||||||||||||||||||||||||||||2.09G/7.62G]",
          "rect": [
            0,
            5,
            100,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "Swp[                                  0K/7.62G]",
          "rect": [
            0,
            6,
            100,
            1
          ]
        }
      ]
    },
    {
      "role": "table",
      "rect": [
        0,
        8,
        100,
        22
      ],
      "children": [
        {
          "role": "rowheader",
          "rect": [
            0,
            8,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "[Main",
              "rect": [
                0,
                8,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "[I/",
              "rect": [
                8,
                8,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                20,
                8,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                25,
                8,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                27,
                8,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                33,
                8,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                39,
                8,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                45,
                8,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                48,
                8,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                54,
                8,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                59,
                8,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                67,
                8,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            9,
            100,
            1
          ],
          "states": [
            "selected"
          ],
          "children": [
            {
              "role": "cell",
              "value": "PID",
              "rect": [
                0,
                9,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "USER",
              "rect": [
                8,
                9,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "RI",
              "rect": [
                20,
                9,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "I",
              "rect": [
                25,
                9,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "VIRT",
              "rect": [
                27,
                9,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "RES",
              "rect": [
                33,
                9,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "SHR",
              "rect": [
                39,
                9,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                9,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "CPU%",
              "rect": [
                48,
                9,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "EM%",
              "rect": [
                54,
                9,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "TIME+",
              "rect": [
                59,
                9,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "Command",
              "rect": [
                67,
                9,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            10,
            100,
            1
          ],
          "states": [
            "selected"
          ],
          "children": [
            {
              "role": "cell",
              "value": "38532",
              "rect": [
                0,
                10,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                10,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                10,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                10,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "5882M",
              "rect": [
                27,
                10,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "615M",
              "rect": [
                33,
                10,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "149M",
              "rect": [
                39,
                10,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                10,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "23.5",
              "rect": [
                48,
                10,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "7.9",
              "rect": [
                54,
                10,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "9:15.70",
              "rect": [
                59,
                10,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "claude --teleport",
              "rect": [
                67,
                10,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            11,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "2639",
              "rect": [
                0,
                11,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                11,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                11,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                11,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "2852M",
              "rect": [
                27,
                11,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "187M",
              "rect": [
                33,
                11,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "93664",
              "rect": [
                39,
                11,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                11,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "4.7",
              "rect": [
                48,
                11,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "2.4",
              "rect": [
                54,
                11,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "7:45.83",
              "rect": [
                59,
                11,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "ghostty",
              "rect": [
                67,
                11,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            12,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "20635",
              "rect": [
                0,
                12,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                12,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                12,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                12,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "2852M",
              "rect": [
                27,
                12,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "187M",
              "rect": [
                33,
                12,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                12,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                12,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "3.8",
              "rect": [
                48,
                12,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "2.4",
              "rect": [
                54,
                12,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "3:28.99",
              "rect": [
                59,
                12,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "ghostty",
              "rect": [
                67,
                12,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            13,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "541338",
              "rect": [
                0,
                13,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                13,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                13,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                13,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "5882M",
              "rect": [
                27,
                13,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "614M",
              "rect": [
                33,
                13,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                13,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                13,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "2.6",
              "rect": [
                48,
                13,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "7.9",
              "rect": [
                54,
                13,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:50.18",
              "rect": [
                59,
                13,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "claude --teleport",
              "rect": [
                67,
                13,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            14,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "541339",
              "rect": [
                0,
                14,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                14,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                14,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                14,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "5882M",
              "rect": [
                27,
                14,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "614M",
              "rect": [
                33,
                14,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                14,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                14,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "2.6",
              "rect": [
                48,
                14,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "7.9",
              "rect": [
                54,
                14,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:49.58",
              "rect": [
                59,
                14,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "claude --teleport",
              "rect": [
                67,
                14,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            15,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "541340",
              "rect": [
                0,
                15,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                15,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                15,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                15,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "5882M",
              "rect": [
                27,
                15,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "614M",
              "rect": [
                33,
                15,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                15,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                15,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "2.1",
              "rect": [
                48,
                15,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "7.9",
              "rect": [
                54,
                15,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:48.21",
              "rect": [
                59,
                15,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "claude --teleport",
              "rect": [
                67,
                15,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            16,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "2617",
              "rect": [
                0,
                16,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                16,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "-2",
              "rect": [
                20,
                16,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                16,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "782M",
              "rect": [
                27,
                16,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "83196",
              "rect": [
                33,
                16,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "59740",
              "rect": [
                39,
                16,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                16,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "1.7",
              "rect": [
                48,
                16,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "1.0",
              "rect": [
                54,
                16,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "3:42.61",
              "rect": [
                59,
                16,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "sway",
              "rect": [
                67,
                16,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            17,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "38533",
              "rect": [
                0,
                17,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                17,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                17,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                17,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "5882M",
              "rect": [
                27,
                17,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "614M",
              "rect": [
                33,
                17,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                17,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                17,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "1.3",
              "rect": [
                48,
                17,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "7.9",
              "rect": [
                54,
                17,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "1:35.15",
              "rect": [
                59,
                17,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "claude --teleport",
              "rect": [
                67,
                17,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            18,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "1052694",
              "rect": [
                0,
                18,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                18,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                18,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                18,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "121M",
              "rect": [
                27,
                18,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "66068",
              "rect": [
                33,
                18,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                18,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                18,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "1.3",
              "rect": [
                48,
                18,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.8",
              "rect": [
                54,
                18,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.06",
              "rect": [
                59,
                18,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "/home/jake/.local/bin/zellij --se",
              "rect": [
                67,
                18,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            19,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "1052818",
              "rect": [
                0,
                19,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                19,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                19,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                19,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "229M",
              "rect": [
                27,
                19,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "6016",
              "rect": [
                33,
                19,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "4584",
              "rect": [
                39,
                19,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "R",
              "rect": [
                45,
                19,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "1.3",
              "rect": [
                48,
                19,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.1",
              "rect": [
                54,
                19,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.11",
              "rect": [
                59,
                19,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "htop",
              "rect": [
                67,
                19,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            20,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "1052917",
              "rect": [
                0,
                20,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                20,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                20,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                20,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "1450G",
              "rect": [
                27,
                20,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "98028",
              "rect": [
                33,
                20,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "74524",
              "rect": [
                39,
                20,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                20,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "1.3",
              "rect": [
                48,
                20,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "1.2",
              "rect": [
                54,
                20,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.03",
              "rect": [
                59,
                20,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "/opt/google/chrome/chrome --type=",
              "rect": [
                67,
                20,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            21,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "1092",
              "rect": [
                0,
                21,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "root",
              "rect": [
                8,
                21,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                21,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                21,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "1322M",
              "rect": [
                27,
                21,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "47016",
              "rect": [
                33,
                21,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                21,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                21,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.4",
              "rect": [
                48,
                21,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.6",
              "rect": [
                54,
                21,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.76",
              "rect": [
                59,
                21,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "/usr/sbin/tailscaled --state=/var",
              "rect": [
                67,
                21,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            22,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "946307",
              "rect": [
                0,
                22,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                22,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                22,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                22,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "1269M",
              "rect": [
                27,
                22,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "56784",
              "rect": [
                33,
                22,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                22,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                22,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.4",
              "rect": [
                48,
                22,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.7",
              "rect": [
                54,
                22,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:01.60",
              "rect": [
                59,
                22,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "waybar -b bar-0",
              "rect": [
                67,
                22,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            23,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "946309",
              "rect": [
                0,
                23,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                23,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                23,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                23,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "1269M",
              "rect": [
                27,
                23,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "56784",
              "rect": [
                33,
                23,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                23,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                23,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.4",
              "rect": [
                48,
                23,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.7",
              "rect": [
                54,
                23,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:01.59",
              "rect": [
                59,
                23,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "waybar -b bar-0",
              "rect": [
                67,
                23,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            24,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "953619",
              "rect": [
                0,
                24,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                24,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                24,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                24,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "53.4G",
              "rect": [
                27,
                24,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "379M",
              "rect": [
                33,
                24,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "261M",
              "rect": [
                39,
                24,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                24,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.4",
              "rect": [
                48,
                24,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "4.9",
              "rect": [
                54,
                24,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:11.98",
              "rect": [
                59,
                24,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "/opt/google/chrome/chrome",
              "rect": [
                67,
                24,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            25,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "953673",
              "rect": [
                0,
                25,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                25,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                25,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                25,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "52.6G",
              "rect": [
                27,
                25,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "132M",
              "rect": [
                33,
                25,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                25,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                25,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.4",
              "rect": [
                48,
                25,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "1.7",
              "rect": [
                54,
                25,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:03.44",
              "rect": [
                59,
                25,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "/opt/google/chrome/chrome --type=",
              "rect": [
                67,
                25,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            26,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "1010921",
              "rect": [
                0,
                26,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                26,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                26,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                26,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "5882M",
              "rect": [
                27,
                26,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "614M",
              "rect": [
                33,
                26,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                26,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                26,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.4",
              "rect": [
                48,
                26,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "7.9",
              "rect": [
                54,
                26,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:01.62",
              "rect": [
                59,
                26,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "claude --teleport",
              "rect": [
                67,
                26,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            27,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "1052690",
              "rect": [
                0,
                27,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                27,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                27,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                27,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "120M",
              "rect": [
                27,
                27,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "64908",
              "rect": [
                33,
                27,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "33188",
              "rect": [
                39,
                27,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                27,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.4",
              "rect": [
                48,
                27,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.8",
              "rect": [
                54,
                27,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.07",
              "rect": [
                59,
                27,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "/home/jake/.local/bin/zellij --se",
              "rect": [
                67,
                27,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            28,
            100,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "1052706",
              "rect": [
                0,
                28,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "jake",
              "rect": [
                8,
                28,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "20",
              "rect": [
                20,
                28,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                25,
                28,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "121M",
              "rect": [
                27,
                28,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "66068",
              "rect": [
                33,
                28,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0",
              "rect": [
                39,
                28,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "S",
              "rect": [
                45,
                28,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.4",
              "rect": [
                48,
                28,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0.8",
              "rect": [
                54,
                28,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "0:00.06",
              "rect": [
                59,
                28,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "/home/jake/.local/bin/zellij --se",
              "rect": [
                67,
                28,
                33,
                1
              ]
            }
          ]
        },
        {
          "role": "row",
          "rect": [
            0,
            29,
            100,
            1
          ],
          "states": [
            "selected"
          ],
          "children": [
            {
              "role": "cell",
              "value": "F1Help",
              "rect": [
                0,
                29,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "F2Se",
              "rect": [
                8,
                29,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "ar",
              "rect": [
                20,
                29,
                2,
                1
              ]
            },
            {
              "role": "cell",
              "value": "4",
              "rect": [
                25,
                29,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "ilter",
              "rect": [
                27,
                29,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "5Tree",
              "rect": [
                33,
                29,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "F6So",
              "rect": [
                39,
                29,
                5,
                1
              ]
            },
            {
              "role": "cell",
              "value": "t",
              "rect": [
                45,
                29,
                1,
                1
              ]
            },
            {
              "role": "cell",
              "value": "F7Ni",
              "rect": [
                48,
                29,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "-F",
              "rect": [
                54,
                29,
                3,
                1
              ]
            },
            {
              "role": "cell",
              "value": "ice +F9",
              "rect": [
                59,
                29,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "ill  F10Quit",
              "rect": [
                67,
                29,
                33,
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

