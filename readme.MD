Optimizations:
- Clean up lexer
    - Simplify main function loop
    - Simplify TokenKind enum and add values
- Simplify operation parsing
    - Recursive-descent parsing (every precedence calls next, then performs while, start from highest)

## TODO:
### Core:
- .function calls (clear, has, delete) on dictionary
- while loop
- Task registry -> register_task + run

### Native functionality:
- Window positioning (track pid?)
- MacOS support

### Other
- Symlink / alias for running
- Daemon

---

## Shell module

The `shell` module opens terminal windows from DSL tasks. It has two functions: `open` for
interactive windows that stay alive, and `run` for windows that close when the command
finishes.

Both functions accept a single dictionary argument with a `cwd` key (the directory to open
the terminal in) and an optional `cmd` key (a command to run on startup). `cwd` defaults to
the runtime's current working directory if omitted. Paths support `~` expansion.

```dsl
// Open an interactive terminal in a directory, drop into shell
shell.open({ cwd: "~/dev/my-project" })

// Open an interactive terminal and run a command first, then keep the shell alive
shell.open({ cwd: "~/dev/my-project", cmd: "nvim ." })

// Open a terminal, run a command, and close the window when it finishes
shell.run({ cwd: "~/dev/my-project", cmd: "pnpm build:watch" })
```

### `shell.open`

Opens an iTerm2 window at `cwd`. If `cmd` is provided it runs first, after which the shell
stays alive — equivalent to ending with `exec $SHELL`. Use this for interactive windows:
editors, REPLs, or any process you want to interact with directly.

### `shell.run`

Opens an iTerm2 window at `cwd`, runs `cmd`, and closes the window automatically when the
process exits. The window closes regardless of whether the command succeeds or fails, and
regardless of iTerm2's "When the session ends" profile setting. Use this for builds,
installs, watchers, or any fire-and-forget process where you want visibility while it runs.

Both calls return immediately — they do not wait for the terminal window to close.

### Platform behaviour

**macOS** — uses iTerm2 via `osascript`. The window is driven with AppleScript: a new window
is created with the default iTerm2 profile, and the startup command is sent to it via
`write text`. `shell.run` appends a self-closing AppleScript call to the command so the
window closes by window ID (not "front window") when the process exits, meaning it targets
the correct window even if focus changes while the command is running.

**Windows** — uses Windows Terminal (`wt.exe`) with WSL. `shell.open` runs
`wt.exe wsl bash -lc "cd '...' && exec bash"`. `shell.run` runs `wsl bash -lc` directly
without Windows Terminal — the command runs in the background with no visible window.

### Known limitations

- Paths containing single quotes are not supported (e.g. `/Users/o'brien/dev`).

