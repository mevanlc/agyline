# Substatus: Exec Component

A user-defined statusline component that shells out to an arbitrary command
and renders its stdout as a segment in the statusline.

## Overview

A new component type (e.g. `Exec` / `Substatus`) that runs a user-configured
command, pipes it the Claude Code JSON (plus ccxline metadata), and renders
the output as a segment alongside the built-in components.

## Design Decisions

### 1. Input to the subprocess

The subprocess receives on stdin:
- The full Claude Code JSON (same payload ccxline itself gets)
- Plus ccxline-specific metadata (enabled components, colors, icons, theme info)
  in case the subprocess wants to use it

### 2. Cardinality

Start with **one** exec component. The user can fan out internally using `tee`,
their own pipeline, or even nested statusline renderers (matroshka statuslines).

### 3. Timing / Caching Strategy

Two complementary ideas:

#### 3a. Execution budget (rate limiting)

- Cap (configurable) the real time spent waiting for subprocess output per
  sliding window of time.
- Track a moving average of `sp_elapsed_ms = sp_exit_ts - sp_start_ts`.
- Space out executions to hover around the cap most of the time.
- Prevents a slow subprocess from dragging down the entire statusline.

#### 3b. Async with cached fallback

- Wait for up to N millis for the subprocess to finalize output.
- If it doesn't finish in time, ccxline's main thread moves on and renders
  using the last cached output from the subprocess.
- A background thread picks up the "late" output, which becomes the new
  cached value for the next render cycle.
- Fast subprocesses update synchronously; slow ones update eventually.
- Composes well with 3a.

### 4. Command configuration

Store the command as a string in the theme TOML (in the component's `options`):

```toml
[[components]]
id = "exec"
enabled = true

[components.options]
command = "my-statusline-helper --format=short"
```

At exec time, split the string with a `shlex`-style parser (like Python's
`shlex.split`). Provide a way to control shell vs. direct exec:

- Default: split and exec directly (no shell, safer)
- Option: `shell = true` to run via `sh -c "..."` (for pipes, redirects, etc.)

```toml
[components.options]
command = "kubectl config current-context | cut -d/ -f2"
shell = true
```

## Use Cases

- `date +%H:%M` — clock
- `kubectl config current-context` — k8s context
- `git stash list | wc -l` — stash count
- Custom project-specific scripts
- Nested/matroshka statusline renderers

## Open Questions

- Icon: what default icon for the exec component? Configurable per usual.
- Separator behavior: does it participate in the normal separator flow?
- Error handling: what to show when the command fails / exits non-zero?
- Max output length: truncate at N chars?
- Multiple exec components: future extension, not v1.
