# fixtures/vendor-src/

Source for the apps `xtask vendor bubbletea` and `xtask vendor ink` build and
capture. Unlike `fixtures/vendor/textual/`, these are not third-party code —
`bubbletea/` and `ink/` are this project's own corpus apps (ported from the
original spike's `corpus/rat`-style instrumented apps), so there's no
upstream LICENSE to carry alongside them.

Neither bubbletea nor Ink ships an offline test backend the way ratatui's
`TestBackend` does, so generating these fixtures means building the real app
and capturing it through a real zellij pty session
(`xtask/src/vendor/zellij.rs`) rather than a cheaper reconstruction.

- `bubbletea/` — Go + Bubbletea + Lipgloss, built with `go build`.
- `ink/` — Node + Ink + React, run directly via `node app.mjs` after
  `npm install`.
