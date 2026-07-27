# vendor/textual/table

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **94%** (chrome-excluded; 2316 non-blank content cells)

## Sizes

| encoding | bytes | tokens | vs plain (bytes) | vs plain (tokens) |
|---|---:|---:|---:|---:|
| raw | 18127 | 10878 | 0.37x | 0.28x |
| plain | 6656 | 3074 | 1.00x | 1.00x |
| compact | 9498 | 5392 | 0.70x | 0.57x |
| nogeo | 6545 | 3232 | 1.02x | 0.95x |
| toon | 3945 | 2511 | 1.69x | 1.22x |
| json | 63197 | 16071 | 0.11x | 0.19x |

<details><summary>Before — raw ANSI (18127 bytes, escapes shown literally)</summary>

```
\x1b[0;48;2;36;47;56m \x1b[0;38;2;224;224;224;48;2;36;47;56m⭘\x1b[0;48;2;36;47;56m                                                      \x1b[0;38;2;224;224;224;48;2;36;47;56mMyApp\x1b[0;48;2;36;47;56m                                                           \x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m╭──────────────────╮╭──────────────────────────────────────────────────────────────────────────────────────────────────╮\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;38;2;224;224;224;48;2;18;18;18mok\x1b[0;48;2;18;18;18m                \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;18;18;18mtest\x1b[0;48;2;18;18;18m                                                                                              \x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;38;2;99;44;166;48;2;18;18;18m╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍\x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;99;44;166;48;2;18;18;18m╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m││╭─\x1b[0;38;2;224;224;224;48;2;18;18;18m 0 \x1b[0;38;2;1;120;212;48;2;18;18;18m────────────────────────────────────────╮╭─\x1b[0;38;2;224;224;224;48;2;18;18;18m 1 \x1b[0;38;2;1;120;212;48;2;18;18;18m────────────────────────────────────────╮╭─\x1b[0;38;2;224;224;224;48;2;18;18;18m 2 \x1b[0;38;2;1;120;212;48;2;18;18;18m─│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;48;2;18;18;18m                                            \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;48;2;18;18;18m                                            \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;48;2;18;18;18m     \x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;1;38;2;224;224;224;48;2;36;47;56m Foo       Bar         Baz                \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;1;38;2;224;224;224;48;2;36;47;56m Foo       Bar         Baz                \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;1;38;2;224;224;224;48;2;36;47;56m Foo \x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;48;84m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;38;2;0;0;0;48;2;0;48;84m▁▁\x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;38;2;0;0;0;48;2;0;48;84m▁▁\x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m│││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ \x1b[0;48;2;0;0;0m  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;38;2;224;224;224;48;2;30;30;30m ABCD\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m││╰────────────────────────────────────────────╯╰────────────────────────────────────────────╯╰─────│\x1b[0m
\x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0;48;2;18;18;18m                  \x1b[0;38;2;1;120;212;48;2;18;18;18m││\x1b[0;48;2;0;48;84m                                                    \x1b[0;38;2;0;48;84;48;2;0;0;0m▎\x1b[0;48;2;0;0;0m                                             \x1b[0;38;2;1;120;212;48;2;18;18;18m│\x1b[0m
```

</details>

## Before (color-stripped)

```
 ⭘                                                      MyApp                                                           
╭──────────────────╮╭──────────────────────────────────────────────────────────────────────────────────────────────────╮
│ok                ││test                                                                                              │
│╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍││╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍│
│                  ││╭─ 0 ────────────────────────────────────────╮╭─ 1 ────────────────────────────────────────╮╭─ 2 ─│
│                  │││                                            ││                                            ││     │
│                  │││ Foo       Bar         Baz                  ││ Foo       Bar         Baz                  ││ Foo │
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ ▁▁││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ ▁▁││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  │││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCDEFGH  0123456789  IJKLMNOPQRSTUVWXYZ   ││ ABCD│
│                  ││╰────────────────────────────────────────────╯╰────────────────────────────────────────────╯╰─────│
│                  ││                                                    ▎                                             │
```

## After — compact

```
application @0,0,120,39
  heading @0,0,120,4 ⭘                                                      MyApp ╭──────────────────╮╭──────────────────────────────────────────────────────────────────────────────────────────────────╮ │ok                ││test                                                                                              │ │╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍││╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍│
  panel "0 1" @21,4,92,34
    table "0" @21,4,46,34
      row [selected] @22,6,44,1
        cell @23,6,8,1 Foo
        cell @33,6,10,1 Bar
        cell @45,6,18,1 Baz
      row @22,7,44,1
        cell @23,7,8,1 ABCDEFGH
        cell @33,7,10,1 0123456789
        cell @45,7,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,8,44,1
        cell @23,8,8,1 ABCDEFGH
        cell @33,8,10,1 0123456789
        cell @45,8,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,9,44,1
        cell @23,9,8,1 ABCDEFGH
        cell @33,9,10,1 0123456789
        cell @45,9,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,10,44,1
        cell @23,10,8,1 ABCDEFGH
        cell @33,10,10,1 0123456789
        cell @45,10,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,11,44,1
        cell @23,11,8,1 ABCDEFGH
        cell @33,11,10,1 0123456789
        cell @45,11,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,12,44,1
        cell @23,12,8,1 ABCDEFGH
        cell @33,12,10,1 0123456789
        cell @45,12,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,13,44,1
        cell @23,13,8,1 ABCDEFGH
        cell @33,13,10,1 0123456789
        cell @45,13,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,14,44,1
        cell @23,14,8,1 ABCDEFGH
        cell @33,14,10,1 0123456789
        cell @45,14,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,15,44,1
        cell @23,15,8,1 ABCDEFGH
        cell @33,15,10,1 0123456789
        cell @45,15,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,16,44,1
        cell @23,16,8,1 ABCDEFGH
        cell @33,16,10,1 0123456789
        cell @45,16,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,17,44,1
        cell @23,17,8,1 ABCDEFGH
        cell @33,17,10,1 0123456789
        cell @45,17,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,18,44,1
        cell @23,18,8,1 ABCDEFGH
        cell @33,18,10,1 0123456789
        cell @45,18,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,19,44,1
        cell @23,19,8,1 ABCDEFGH
        cell @33,19,10,1 0123456789
        cell @45,19,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,20,44,1
        cell @23,20,8,1 ABCDEFGH
        cell @33,20,10,1 0123456789
        cell @45,20,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,21,44,1
        cell @23,21,8,1 ABCDEFGH
        cell @33,21,10,1 0123456789
        cell @45,21,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,22,44,1
        cell @23,22,8,1 ABCDEFGH
        cell @33,22,10,1 0123456789
        cell @45,22,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,23,44,1
        cell @23,23,8,1 ABCDEFGH
        cell @33,23,10,1 0123456789
        cell @45,23,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,24,44,1
        cell @23,24,8,1 ABCDEFGH
        cell @33,24,10,1 0123456789
        cell @45,24,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,25,44,1
        cell @23,25,8,1 ABCDEFGH
        cell @33,25,10,1 0123456789
        cell @45,25,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,26,44,1
        cell @23,26,8,1 ABCDEFGH
        cell @33,26,10,1 0123456789
        cell @45,26,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,27,44,1
        cell @23,27,8,1 ABCDEFGH
        cell @33,27,10,1 0123456789
        cell @45,27,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,28,44,1
        cell @23,28,8,1 ABCDEFGH
        cell @33,28,10,1 0123456789
        cell @45,28,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,29,44,1
        cell @23,29,8,1 ABCDEFGH
        cell @33,29,10,1 0123456789
        cell @45,29,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,30,44,1
        cell @23,30,8,1 ABCDEFGH
        cell @33,30,10,1 0123456789
        cell @45,30,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,31,44,1
        cell @23,31,8,1 ABCDEFGH
        cell @33,31,10,1 0123456789
        cell @45,31,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,32,44,1
        cell @23,32,8,1 ABCDEFGH
        cell @33,32,10,1 0123456789
        cell @45,32,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,33,44,1
        cell @23,33,8,1 ABCDEFGH
        cell @33,33,10,1 0123456789
        cell @45,33,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,34,44,1
        cell @23,34,8,1 ABCDEFGH
        cell @33,34,10,1 0123456789
        cell @45,34,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,35,44,1
        cell @23,35,8,1 ABCDEFGH
        cell @33,35,10,1 0123456789
        cell @45,35,18,1 IJKLMNOPQRSTUVWXYZ
      row @22,36,44,1
        cell @23,36,8,1 ABCDEFGH
        cell @33,36,10,1 0123456789
        cell @45,36,18,1 IJKLMNOPQRSTUVWXYZ
    table "1" @67,4,46,34
      row [selected] @68,6,44,1
        cell @69,6,8,1 Foo
        cell @79,6,10,1 Bar
        cell @91,6,18,1 Baz
      row @68,7,44,1
        cell @69,7,8,1 ABCDEFGH
        cell @79,7,10,1 0123456789
        cell @91,7,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,8,44,1
        cell @69,8,8,1 ABCDEFGH
        cell @79,8,10,1 0123456789
        cell @91,8,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,9,44,1
        cell @69,9,8,1 ABCDEFGH
        cell @79,9,10,1 0123456789
        cell @91,9,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,10,44,1
        cell @69,10,8,1 ABCDEFGH
        cell @79,10,10,1 0123456789
        cell @91,10,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,11,44,1
        cell @69,11,8,1 ABCDEFGH
        cell @79,11,10,1 0123456789
        cell @91,11,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,12,44,1
        cell @69,12,8,1 ABCDEFGH
        cell @79,12,10,1 0123456789
        cell @91,12,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,13,44,1
        cell @69,13,8,1 ABCDEFGH
        cell @79,13,10,1 0123456789
        cell @91,13,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,14,44,1
        cell @69,14,8,1 ABCDEFGH
        cell @79,14,10,1 0123456789
        cell @91,14,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,15,44,1
        cell @69,15,8,1 ABCDEFGH
        cell @79,15,10,1 0123456789
        cell @91,15,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,16,44,1
        cell @69,16,8,1 ABCDEFGH
        cell @79,16,10,1 0123456789
        cell @91,16,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,17,44,1
        cell @69,17,8,1 ABCDEFGH
        cell @79,17,10,1 0123456789
        cell @91,17,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,18,44,1
        cell @69,18,8,1 ABCDEFGH
        cell @79,18,10,1 0123456789
        cell @91,18,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,19,44,1
        cell @69,19,8,1 ABCDEFGH
        cell @79,19,10,1 0123456789
        cell @91,19,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,20,44,1
        cell @69,20,8,1 ABCDEFGH
        cell @79,20,10,1 0123456789
        cell @91,20,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,21,44,1
        cell @69,21,8,1 ABCDEFGH
        cell @79,21,10,1 0123456789
        cell @91,21,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,22,44,1
        cell @69,22,8,1 ABCDEFGH
        cell @79,22,10,1 0123456789
        cell @91,22,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,23,44,1
        cell @69,23,8,1 ABCDEFGH
        cell @79,23,10,1 0123456789
        cell @91,23,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,24,44,1
        cell @69,24,8,1 ABCDEFGH
        cell @79,24,10,1 0123456789
        cell @91,24,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,25,44,1
        cell @69,25,8,1 ABCDEFGH
        cell @79,25,10,1 0123456789
        cell @91,25,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,26,44,1
        cell @69,26,8,1 ABCDEFGH
        cell @79,26,10,1 0123456789
        cell @91,26,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,27,44,1
        cell @69,27,8,1 ABCDEFGH
        cell @79,27,10,1 0123456789
        cell @91,27,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,28,44,1
        cell @69,28,8,1 ABCDEFGH
        cell @79,28,10,1 0123456789
        cell @91,28,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,29,44,1
        cell @69,29,8,1 ABCDEFGH
        cell @79,29,10,1 0123456789
        cell @91,29,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,30,44,1
        cell @69,30,8,1 ABCDEFGH
        cell @79,30,10,1 0123456789
        cell @91,30,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,31,44,1
        cell @69,31,8,1 ABCDEFGH
        cell @79,31,10,1 0123456789
        cell @91,31,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,32,44,1
        cell @69,32,8,1 ABCDEFGH
        cell @79,32,10,1 0123456789
        cell @91,32,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,33,44,1
        cell @69,33,8,1 ABCDEFGH
        cell @79,33,10,1 0123456789
        cell @91,33,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,34,44,1
        cell @69,34,8,1 ABCDEFGH
        cell @79,34,10,1 0123456789
        cell @91,34,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,35,44,1
        cell @69,35,8,1 ABCDEFGH
        cell @79,35,10,1 0123456789
        cell @91,35,18,1 IJKLMNOPQRSTUVWXYZ
      row @68,36,44,1
        cell @69,36,8,1 ABCDEFGH
        cell @79,36,10,1 0123456789
        cell @91,36,18,1 IJKLMNOPQRSTUVWXYZ
  statusbar @0,38,120,1 │                  ││                                                    ▎                                             │
```

## After — toon

```
application
  heading: ⭘                                                      MyApp ╭──────────────────╮╭──────────────────────────────────────────────────────────────────────────────────────────────────╮ │ok                ││test                                                                                              │ │╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍││╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍│
  panel 0 1
    table 0[31]{c0,c1,c2}:
      *Foo,Bar,Baz
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
    table 1[31]{c0,c1,c2}:
      *Foo,Bar,Baz
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
       ABCDEFGH,0123456789,IJKLMNOPQRSTUVWXYZ
  statusbar: │                  ││                                                    ▎                                             │
```

<details><summary>After — json</summary>

```json
{
  "role": "application",
  "rect": [
    0,
    0,
    120,
    39
  ],
  "children": [
    {
      "role": "heading",
      "value": "⭘                                                      MyApp ╭──────────────────╮╭──────────────────────────────────────────────────────────────────────────────────────────────────╮ │ok                ││test                                                                                              │ │╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍││╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍╍│",
      "rect": [
        0,
        0,
        120,
        4
      ]
    },
    {
      "role": "panel",
      "name": "0 1",
      "rect": [
        21,
        4,
        92,
        34
      ],
      "children": [
        {
          "role": "table",
          "name": "0",
          "rect": [
            21,
            4,
            46,
            34
          ],
          "children": [
            {
              "role": "row",
              "rect": [
                22,
                6,
                44,
                1
              ],
              "states": [
                "selected"
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "Foo",
                  "rect": [
                    23,
                    6,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "Bar",
                  "rect": [
                    33,
                    6,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "Baz",
                  "rect": [
                    45,
                    6,
                    18,
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
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    7,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    7,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    7,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                8,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    8,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    8,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    8,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                9,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    9,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    9,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    9,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                10,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    10,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    10,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    10,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                11,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    11,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    11,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    11,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                12,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    12,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    12,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    12,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                13,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    13,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    13,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    13,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                14,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    14,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    14,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    14,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                15,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    15,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    15,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    15,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                16,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    16,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    16,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    16,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                17,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    17,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    17,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    17,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                18,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    18,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    18,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    18,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                19,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    19,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    19,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    19,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                20,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    20,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    20,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    20,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                21,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    21,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    21,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    21,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                22,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    22,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    22,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    22,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                23,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    23,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    23,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    23,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                24,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    24,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    24,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    24,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                25,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    25,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    25,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    25,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                26,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    26,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    26,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    26,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                27,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    27,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    27,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    27,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                28,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    28,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    28,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    28,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                29,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    29,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    29,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    29,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                30,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    30,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    30,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    30,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                31,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    31,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    31,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    31,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                32,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    32,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    32,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    32,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                33,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    33,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    33,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    33,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                34,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    34,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    34,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    34,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                35,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    35,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    35,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    35,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                22,
                36,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    23,
                    36,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    33,
                    36,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    45,
                    36,
                    18,
                    1
                  ]
                }
              ]
            }
          ]
        },
        {
          "role": "table",
          "name": "1",
          "rect": [
            67,
            4,
            46,
            34
          ],
          "children": [
            {
              "role": "row",
              "rect": [
                68,
                6,
                44,
                1
              ],
              "states": [
                "selected"
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "Foo",
                  "rect": [
                    69,
                    6,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "Bar",
                  "rect": [
                    79,
                    6,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "Baz",
                  "rect": [
                    91,
                    6,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                7,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    7,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    7,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    7,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                8,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    8,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    8,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    8,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                9,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    9,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    9,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    9,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                10,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    10,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    10,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    10,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                11,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    11,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    11,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    11,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                12,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    12,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    12,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    12,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                13,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    13,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    13,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    13,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                14,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    14,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    14,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    14,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                15,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    15,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    15,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    15,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                16,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    16,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    16,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    16,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                17,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    17,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    17,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    17,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                18,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    18,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    18,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    18,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                19,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    19,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    19,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    19,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                20,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    20,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    20,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    20,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                21,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    21,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    21,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    21,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                22,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    22,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    22,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    22,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                23,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    23,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    23,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    23,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                24,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    24,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    24,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    24,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                25,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    25,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    25,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    25,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                26,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    26,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    26,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    26,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                27,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    27,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    27,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    27,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                28,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    28,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    28,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    28,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                29,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    29,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    29,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    29,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                30,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    30,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    30,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    30,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                31,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    31,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    31,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    31,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                32,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    32,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    32,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    32,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                33,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    33,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    33,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    33,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                34,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    34,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    34,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    34,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                35,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    35,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    35,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    35,
                    18,
                    1
                  ]
                }
              ]
            },
            {
              "role": "row",
              "rect": [
                68,
                36,
                44,
                1
              ],
              "children": [
                {
                  "role": "cell",
                  "value": "ABCDEFGH",
                  "rect": [
                    69,
                    36,
                    8,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "0123456789",
                  "rect": [
                    79,
                    36,
                    10,
                    1
                  ]
                },
                {
                  "role": "cell",
                  "value": "IJKLMNOPQRSTUVWXYZ",
                  "rect": [
                    91,
                    36,
                    18,
                    1
                  ]
                }
              ]
            }
          ]
        }
      ]
    },
    {
      "role": "statusbar",
      "value": "│                  ││                                                    ▎                                             │",
      "rect": [
        0,
        38,
        120,
        1
      ]
    }
  ]
}
```

</details>

