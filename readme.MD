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

