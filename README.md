# unrender

Turn a terminal screen dump into a structured accessibility tree, so an agent can read a TUI as
data instead of raw bytes.

```console
$ unrender screen.ansi --format toon
table services[6]{SERVICE,STATE,P99}:
  *api-gateway,running,12ms
   auth-service,running,31ms
   billing,degraded,412ms
   search-index,running,88ms
   mailer,stopped,-
   scheduler,running,5ms
list events[5]:
   deploy started
   image pulled
   health check ok
```

Piping a TUI's output straight into a model gives it a wall of ANSI escape sequences. `unrender`
parses that screen the way a screen reader parses a GUI — recovering panels, tables, lists, trees,
selection state and progress bars — and emits them in an ARIA-style vocabulary the model already
has priors about.

## Status

Early. The parser works and is measured (see [`results/`](results/)), but the corpus is still
small and there are known failures recorded in [`results/baseline.json`](results/baseline.json).
Nothing here is stable yet.

## What it is not

Not a terminal emulator, and not a multiplexer. `unrender` takes a screen dump you already have
and turns it into structure. Getting that dump — running the app, capturing the screen, injecting
keys — is somebody else's job.

## The claim this repo exists to test

**That this representation serves an LLM better than raw terminal output does.**

That is an empirical claim, so the repo is built to check it rather than assert it. Five
complementary measurements, because no single number is honest on its own:

| Axis | Question |
|---|---|
| Round-trip fidelity | Did we lose content? (`tree → grid → diff`) |
| Structural accuracy | Did we get the layout right? (IoU vs the framework's real widget tree) |
| State tracking | Does the tree follow state changes across a before/after pair? |
| **LLM benchmark** | **Does it actually help a model, and which models?** |
| Human legibility | Would you actually send this to a model? (before/after, read by eye) |

Round-trip fidelity in particular is easy to misread: it measures *information preservation*, not
correctness. A tree that flattens an entire screen into one `log` block preserves 100% of the
characters and has zero structure. It is only meaningful next to the structural score.

## Testing

The suite is pure and offline. No zellij, no Go, no Node, no Python, no network, no API key.

```console
cargo test --workspace                                    # gate on drift from the baseline
cargo test -p unrender-eval -- --nocapture                # see the per-fixture table
UNRENDER_UPDATE_BASELINE=1 cargo test -p unrender-eval    # re-record after a deliberate change
```

It gates on **drift from `results/baseline.json`**, not on absolute quality. Known failures are
recorded there as facts rather than hidden, so fixing one shows up as a visible improvement
instead of a check quietly going green. Regressions fail the build; improvements also fail, asking
you to re-record — an unrecorded improvement means the baseline is lying about where the project
is.

Generating or refreshing fixtures is a separate, deliberate act (`cargo run -p xtask -- …`) and
*does* need external tools. That asymmetry is on purpose: a flaky capture should produce a diff a
human reviews, never a red build nobody can diagnose.

## Fixtures

Two categories, because they answer different questions.

- **`vendor/`** — a framework's own examples and snapshot corpora (ratatui, Textual, bubbletea,
  Ink). Cheap, and some carry real widget-tree ground truth. But they are *idealized* usage,
  written by the people who built the layout engine.
- **`field/`** — real, independently-built applications. Captured launch-only at a pinned release:
  one default screen, no keystrokes, no per-app knowledge. Mostly no ground truth.

`field/` exists to keep the corpus honest. The worst results so far are all real-world tools —
`vim`, `less`, `man`, `top` — at or below the plain-text baseline. A corpus of only framework
examples would systematically flatter the tool.

Fixtures are frozen `.ansi` files pinned to a release, so an upstream app changing its layout
cannot move a test. Drift appears only when someone deliberately refreshes, producing a reviewed
diff.

## Origin

Extracted from a spike that asked whether a multiplexer could act as a substrate for agents
driving arbitrary TUIs. That research measured the substrate itself — screen-dump tearing rates,
push-stream latency, key-injection reliability — and is not carried over here; this repo is only
the translation layer.

One correction worth stating plainly: the spike published token-savings numbers measured with
`tiktoken`, which is an OpenAI tokenizer and materially undercounts Claude — worse on exactly the
box-drawing and code-like content a terminal dump is made of. Those figures are not valid and are
being recounted against the provider's own `count_tokens`. No token claim in this repo should be
trusted until it appears in `results/`.

## License

MIT OR Apache-2.0
