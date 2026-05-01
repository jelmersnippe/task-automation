## while loop

`while (condition) { ... }` runs its block repeatedly as long as the condition is true.
The condition must evaluate to a boolean. `break` exits the loop early; `continue` skips
to the next iteration.

```dsl
var i = 0
while (i < 5) {
    print(i)
    i = i + 1
}
```

The DSL has no `for` loop yet, so `while` with a counter is the standard way to iterate
over a list:

```dsl
var items = ["a", "b", "c"]
var i = 0

while (i < items.len()) {
    print(items[i])
    i = i + 1
}
```

---

## parallel

`parallel(fns)` takes a list of zero-argument functions and runs them concurrently on real
OS threads. All functions start at the same time and the call blocks until every one of them
has finished. Errors from individual functions are isolated — one failing task does not
cancel the others.

Each thread receives a **clone of the runtime context**, which includes a copy of the current
variable scope. Any variables written inside a parallel task are local to that thread and are
dropped when it finishes. If you need results out of a parallel block, collect them into a
shared data structure before calling `parallel`.

```dsl
var results = []

parallel([
    fn() { shell.run({ cwd: "~/dev/api",      cmd: "cargo build" }) },
    fn() { shell.run({ cwd: "~/dev/frontend", cmd: "pnpm build" }) },
    fn() { shell.run({ cwd: "~/dev/docs",     cmd: "pnpm build" }) },
])
```

---

## Git module

The `git` module wraps common git operations. All functions operate on the runtime's current
working directory. Use `git.in_directory(path)` to target a specific repo — it returns a new
`git` module instance scoped to that path, so operations can be chained.

```dsl
// Run in the current working directory
git.fetch()

// Run in a specific directory
git.in_directory("~/dev/my-project").pull()

// Chain multiple operations in the same directory
var dir = "~/dev/my-project"
git.in_directory(dir).fetch()
git.in_directory(dir).rebase()
git.in_directory(dir).push()
```

### Functions

| Function | Returns | Description |
|---|---|---|
| `git.in_directory(path)` | `git` | Returns a git module instance scoped to `path`. Supports `~` expansion. |
| `git.current_branch()` | `string` | Returns the name of the current branch. |
| `git.local_branches()` | `list` | Returns a list of local branch name strings. |
| `git.remote_branches()` | `list` | Returns a list of remote branch name strings. |
| `git.worktrees()` | `list` | Returns a list of dictionaries with `"directory"` and `"branch"` keys. |
| `git.fetch()` | — | Runs `git fetch`. |
| `git.pull()` | — | Runs `git pull`. |
| `git.push()` | — | Pushes the current branch to `origin`. |
| `git.push("--force")` | — | Force-pushes using `--force-with-lease`. |
| `git.rebase()` | — | Rebases the current branch onto `origin/master`. |
| `git.prune()` | — | Runs `git gc` to prune loose objects. |
| `git.delete_branch(name)` | — | Force-deletes the named local branch (`git branch -D`). |

### Example — update all worktrees

```dsl
git.fetch()

var worktrees = git.worktrees()
var actions = []
var i = 0

while (i < worktrees.len()) {
    var dir = worktrees[i]["directory"]
    actions.push(fn() {
        git.in_directory(dir).pull()
    })
    i = i + 1
}

parallel(actions)
```

---

## tmux module

The `tmux` module manages tmux sessions, windows, and panes. It uses a two-tier design:
the top-level `tmux` module has session factory functions, and `new_session` returns a
session module for all per-session operations. Session methods return `self` for chaining.

### Session management

| Function | Returns | Description |
|---|---|---|
| `tmux.new_session("name")` | session module | Create a detached session and return its module |
| `tmux.kill_session("name")` | — | Kill a session by name |
| `tmux.has_session("name")` | bool | Check whether a session exists |

### Window management

Called on the session module returned by `new_session`.

| Method | Returns | Description |
|---|---|---|
| `session.new_window("name", {cwd?, cmd?})` | session | Create a window, optionally running `cmd` in `cwd` |
| `session.kill_window("name")` | session | Kill a window by name or index |
| `session.select_window("name")` | session | Make a window the active window |

### Pane management

| Method | Returns | Description |
|---|---|---|
| `session.split_pane("window", {cwd?, cmd?})` | session | Vertical split in `window` |
| `session.split_pane_h("window", {cwd?, cmd?})` | session | Horizontal split in `window` |
| `session.kill_pane("window", pane_index)` | session | Kill a pane by index |
| `session.set_layout("window", "layout")` | session | Apply a tmux layout |

Built-in layouts: `tiled`, `even-horizontal`, `even-vertical`, `main-horizontal`, `main-vertical`.

### Interactive use

| Method | Returns | Description |
|---|---|---|
| `session.send_keys("window", pane_index, "cmd")` | session | Send a command to a pane (appends Enter) |
| `session.interrupt("window", pane_index)` | session | Send Ctrl-C to a pane |

### Attaching

| Method | Returns | Description |
|---|---|---|
| `session.attach()` | — | Attach the current terminal to the session |
| `session.attach_cmd()` | string | Returns `"tmux attach -t <name>"` for use with `shell.open` |

### Command dict

`new_window`, `split_pane`, and `split_pane_h` accept `{cwd?, cmd?}`:

- `cwd` — working directory; falls back to the runtime `cwd` if omitted
- `cmd` — startup command; falls back to `exec $SHELL` if omitted

`new_session` creates a default unnamed window at index `"0"`. Call `.kill_window("0")`
after adding your named windows to clean it up.

### Philosophy

Commands are passed directly to panes at creation time — no `send_keys` for startup.
Restarting a session means killing and recreating it, guaranteeing a clean state.
`send_keys` and `interrupt` exist for ad-hoc interactive use only.

### Example — watcher session with restart

```dsl
fn setup_watchers(dir) {
    if (tmux.has_session("watchers")) {
        tmux.kill_session("watchers")
    }

    tmux.new_session("watchers")
        .new_window("main", {cwd: dir, cmd: "npm run build:watch"})
        .split_pane("main", {cwd: dir, cmd: "npm run test:watch"})
        .split_pane("main", {cwd: dir, cmd: "npm run dev"})
        .set_layout("main", "main-vertical")
        .kill_window("0")

    shell.open({cwd: dir, cmd: "tmux attach -t watchers"})
}

register_task("setup_watchers", fn(dir) {
    setup_watchers(dir)
})
```

### Example — full dev setup

```dsl
fn setup(primary, secondary) {
    if (tmux.has_session("work") == false) {
        tmux.new_session("work")
            .new_window("editor",   {cwd: primary, cmd: "nvim ."})
            .new_window("shell",    {cwd: primary})
            .new_window("opencode", {cwd: primary, cmd: "opencode ."})
            .kill_window("0")
    }

    if (tmux.has_session("watchers") == false) {
        tmux.new_session("watchers")
            .new_window("main", {cwd: primary})
            .split_pane("main", {cwd: primary})
            .split_pane("main", {cwd: secondary})
            .set_layout("main", "main-vertical")
            .kill_window("0")
    }

    shell.open({cwd: primary, cmd: "tmux attach -t work"})
    shell.open({cwd: primary, cmd: "tmux attach -t watchers"})
}

register_task("setup", fn(primary) {
    setup(primary, primary)
})

register_task("setup_dual", fn(primary, secondary) {
    setup(primary, secondary)
})
```

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

