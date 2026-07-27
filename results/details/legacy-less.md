# legacy/less

[← back to REPORT.md](../REPORT.md)

truth source: `none`

content preservation: **76%** (chrome-excluded; 753 non-blank content cells)

## Before (color-stripped)

```
#!/usr/bin/env python3
"""Textual corpus app, instrumented to emit GROUND TRUTH.

Textual knows exactly where every widget landed (`widget.region`). We dump that
real DOM to JSON on a timer, so the inferred accessibility tree can be scored
against what the framework actually laid out rather than against our own
impression of the screenshot.

Keys: j/k or Down/Up move the DataTable cursor, q quits.
"""
import json
import os
import sys

from textual.app import App, ComposeResult
from textual.containers import Horizontal, Vertical
from textual.widgets import DataTable, Footer, Header, Static, Tree

TRUTH_PATH = os.environ.get("TRUTH_PATH", "")

SERVICES = [
    ("api-gateway", "running", "12ms"),
    ("auth-service", "running", "31ms"),
    ("billing", "degraded", "412ms"),
    ("search-index", "running", "88ms"),
    ("mailer", "stopped", "-"),
    ("scheduler", "running", "5ms"),
]

:
```

## After — compact

```
application @0,0,79,30
  list @0,0,79,2
    listitem @0,0,79,1 #!/usr/bin/env python3
    listitem @0,1,79,1 """Textual corpus app, instrumented to emit GROUND TRUTH.
  log @0,3,79,4 Textual knows exactly where every widget landed (`widget.region`). We dump that\nreal DOM to JSON on a timer, so the inferred accessibility tree can be scored\nagainst what the framework actually laid out rather than against our own\nimpression of the screenshot.
  list @0,8,79,5
    listitem @0,8,79,1 Keys: j/k or Down/Up move the DataTable cursor, q quits.
    listitem @0,9,79,1 """
    listitem @0,10,79,1 import json
    listitem @0,11,79,1 import os
    listitem @0,12,79,1 import sys
  table @0,14,79,3
    row @0,14,79,1
      cell @0,14,4,1 from
      cell @5,14,49,1 textual.app import App, ComposeResult
      cell @55,14,7,1
      cell @63,14,4,1
    row @0,15,79,1
      cell @0,15,4,1 from
      cell @5,15,49,1 textual.containers import Horizontal, Vertical
      cell @55,15,7,1
      cell @63,15,4,1
    row @0,16,79,1
      cell @0,16,4,1 from
      cell @5,16,49,1 textual.widgets import DataTable, Footer, Header,
      cell @55,16,7,1 Static,
      cell @63,16,4,1 Tree
  statusbar @0,18,79,1 TRUTH_PATH = os.environ.get("TRUTH_PATH", "")
  list @0,20,79,8
    listitem @0,20,79,1 SERVICES = [
    listitem @0,21,79,1 ("api-gateway", "running", "12ms"),
    listitem @0,22,79,1 ("auth-service", "running", "31ms"),
    listitem @0,23,79,1 ("billing", "degraded", "412ms"),
    listitem @0,24,79,1 ("search-index", "running", "88ms"),
    listitem @0,25,79,1 ("mailer", "stopped", "-"),
    listitem @0,26,79,1 ("scheduler", "running", "5ms"),
    listitem @0,27,79,1 ]
  statusbar @0,29,79,1 :
```

## After — toon

```
application
  list [2]:
     #!/usr/bin/env python3
     """Textual corpus app, instrumented to emit GROUND TRUTH.
  log: Textual knows exactly where every widget landed (`widget.region`). We dump that\nreal DOM to JSON on a timer, so the inferred accessibility tree can be scored\nagainst what the framework actually laid out rather than against our own\nimpression of the screenshot.
  list [5]:
     Keys: j/k or Down/Up move the DataTable cursor, q quits.
     """
     import json
     import os
     import sys
  table [3]{c0,c1,c2,c3}:
     from,textual.app import App, ComposeResult,,
     from,textual.containers import Horizontal, Vertical,,
     from,textual.widgets import DataTable, Footer, Header,,Static,,Tree
  statusbar: TRUTH_PATH = os.environ.get("TRUTH_PATH", "")
  list [8]:
     SERVICES = [
     ("api-gateway", "running", "12ms"),
     ("auth-service", "running", "31ms"),
     ("billing", "degraded", "412ms"),
     ("search-index", "running", "88ms"),
     ("mailer", "stopped", "-"),
     ("scheduler", "running", "5ms"),
     ]
  statusbar: :
```

<details><summary>After — json</summary>

```json
{
  "role": "application",
  "rect": [
    0,
    0,
    79,
    30
  ],
  "children": [
    {
      "role": "list",
      "rect": [
        0,
        0,
        79,
        2
      ],
      "children": [
        {
          "role": "listitem",
          "value": "#!/usr/bin/env python3",
          "rect": [
            0,
            0,
            79,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "\"\"\"Textual corpus app, instrumented to emit GROUND TRUTH.",
          "rect": [
            0,
            1,
            79,
            1
          ]
        }
      ]
    },
    {
      "role": "log",
      "value": "Textual knows exactly where every widget landed (`widget.region`). We dump that\nreal DOM to JSON on a timer, so the inferred accessibility tree can be scored\nagainst what the framework actually laid out rather than against our own\nimpression of the screenshot.",
      "rect": [
        0,
        3,
        79,
        4
      ]
    },
    {
      "role": "list",
      "rect": [
        0,
        8,
        79,
        5
      ],
      "children": [
        {
          "role": "listitem",
          "value": "Keys: j/k or Down/Up move the DataTable cursor, q quits.",
          "rect": [
            0,
            8,
            79,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "\"\"\"",
          "rect": [
            0,
            9,
            79,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "import json",
          "rect": [
            0,
            10,
            79,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "import os",
          "rect": [
            0,
            11,
            79,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "import sys",
          "rect": [
            0,
            12,
            79,
            1
          ]
        }
      ]
    },
    {
      "role": "table",
      "rect": [
        0,
        14,
        79,
        3
      ],
      "children": [
        {
          "role": "row",
          "rect": [
            0,
            14,
            79,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "from",
              "rect": [
                0,
                14,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "textual.app import App, ComposeResult",
              "rect": [
                5,
                14,
                49,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                55,
                14,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                63,
                14,
                4,
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
            79,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "from",
              "rect": [
                0,
                15,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "textual.containers import Horizontal, Vertical",
              "rect": [
                5,
                15,
                49,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                55,
                15,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "rect": [
                63,
                15,
                4,
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
            79,
            1
          ],
          "children": [
            {
              "role": "cell",
              "value": "from",
              "rect": [
                0,
                16,
                4,
                1
              ]
            },
            {
              "role": "cell",
              "value": "textual.widgets import DataTable, Footer, Header,",
              "rect": [
                5,
                16,
                49,
                1
              ]
            },
            {
              "role": "cell",
              "value": "Static,",
              "rect": [
                55,
                16,
                7,
                1
              ]
            },
            {
              "role": "cell",
              "value": "Tree",
              "rect": [
                63,
                16,
                4,
                1
              ]
            }
          ]
        }
      ]
    },
    {
      "role": "statusbar",
      "value": "TRUTH_PATH = os.environ.get(\"TRUTH_PATH\", \"\")",
      "rect": [
        0,
        18,
        79,
        1
      ]
    },
    {
      "role": "list",
      "rect": [
        0,
        20,
        79,
        8
      ],
      "children": [
        {
          "role": "listitem",
          "value": "SERVICES = [",
          "rect": [
            0,
            20,
            79,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "(\"api-gateway\", \"running\", \"12ms\"),",
          "rect": [
            0,
            21,
            79,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "(\"auth-service\", \"running\", \"31ms\"),",
          "rect": [
            0,
            22,
            79,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "(\"billing\", \"degraded\", \"412ms\"),",
          "rect": [
            0,
            23,
            79,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "(\"search-index\", \"running\", \"88ms\"),",
          "rect": [
            0,
            24,
            79,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "(\"mailer\", \"stopped\", \"-\"),",
          "rect": [
            0,
            25,
            79,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "(\"scheduler\", \"running\", \"5ms\"),",
          "rect": [
            0,
            26,
            79,
            1
          ]
        },
        {
          "role": "listitem",
          "value": "]",
          "rect": [
            0,
            27,
            79,
            1
          ]
        }
      ]
    },
    {
      "role": "statusbar",
      "value": ":",
      "rect": [
        0,
        29,
        79,
        1
      ]
    }
  ]
}
```

</details>

