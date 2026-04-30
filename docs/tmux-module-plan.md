# tmux Module Plan

## Core philosophy

- Commands are passed **directly to panes at creation time** — no `send_keys` for startup
- Restart = **kill and recreate** — guaranteed clean state
- `send_keys` exists but is explicitly for interactive/one-off use only

---

## API

### Session management

| Method | tmux command | Notes |
|---|---|---|
| `tmux.new_session("name")` | `new-session -d -s name` | `-d` = detached |
| `tmux.kill_session("name")` | `kill-session -t name` | |
| `tmux.has_session("name")` → bool | `has-session -t name` | For idempotent scripts |
| `tmux.attach("name")` | `attach-session -t name` | Attach current terminal |

### Window management

| Method | tmux command | Notes |
|---|---|---|
| `tmux.new_window("session", "name", "cmd")` | `new-window -t session -n name "cmd"` | Command runs immediately on open |
| `tmux.kill_window("session", "name")` | `kill-window -t session:name` | |
| `tmux.select_window("session", "name")` | `select-window -t session:name` | |

### Pane management

| Method | tmux command | Notes |
|---|---|---|
| `tmux.split_pane("session", "window", "cmd")` | `split-window -t session:window -d "cmd"` | Vertical split, runs command immediately |
| `tmux.split_pane_h("session", "window", "cmd")` | `split-window -h -t session:window -d "cmd"` | Horizontal split |
| `tmux.kill_pane("session", "window", pane)` | `kill-pane -t session:window.pane` | |
| `tmux.set_layout("session", "window", "layout")` | `select-layout -t session:window layout` | See layouts below |

**Built-in layouts**: `tiled`, `even-horizontal`, `even-vertical`, `main-horizontal`, `main-vertical`

### Interactive use (one-off only)

| Method | tmux command | Notes |
|---|---|---|
| `tmux.send_keys("session", "window", pane, "cmd")` | `send-keys -t session:window.pane "cmd" Enter` | Ad-hoc commands only, not for setup |
| `tmux.interrupt("session", "window", pane)` | `send-keys -t session:window.pane C-c` | No Enter — stops running process |

---

## Watcher restart pattern

The recommended pattern for restarting watchers when switching project directories.
Kill the entire session and recreate it — guaranteed clean state, no `send_keys` needed.

```dsl
fn setup_watchers(dir) {
    if tmux.has_session("watchers") {
        tmux.kill_session("watchers")
    }

    tmux.new_session("watchers")
    tmux.new_window("watchers", "watches", "cd " + dir + " && npm run build:watch")
    tmux.split_pane("watchers", "watches", "cd " + dir + " && npm run test:watch")
    tmux.split_pane("watchers", "watches", "cd " + dir + " && npm run dev")
    tmux.set_layout("watchers", "watches", "tiled")
}

register_task("setup_watchers", setup_watchers)
```

---

## Full desktop setup

Two tmux sessions across two monitors, preserving the two-desktop swipe workflow.

```
Main monitor
├── Desktop 1  →  iTerm2 window  →  tmux attach "work"
│                                    ├── window: editor-1  [nvim, full size]
│                                    ├── window: editor-2  [nvim, full size]
│                                    └── window: editor-3  [nvim, full size]
│
└── Desktop 2  →  iTerm2 window  →  tmux attach "watchers"
                                     └── window: watches
                                         ├── pane 0  [build:watch]
                                         ├── pane 1  [test:watch]
                                         └── pane 2  [dev server]

Secondary monitor
├── Desktop 1  →  regular use (browser, Slack etc — no tmux needed)
│
└── Desktop 2  →  iTerm2 window  →  tmux attach "app"
                                     └── window: running
                                         └── pane 0  [app process]
```

Swiping to Desktop 2 on both monitors simultaneously = "debugging mode":
`watchers` on the left, `app` on the right.

```dsl
fn setup() {
    if tmux.has_session("work") == false {
        tmux.new_session("work")
        tmux.new_window("work", "editor-1", "nvim .")
        tmux.new_window("work", "editor-2", "nvim .")
        tmux.new_window("work", "editor-3", "nvim .")
    }

    if tmux.has_session("watchers") == false {
        tmux.new_session("watchers")
        tmux.new_window("watchers", "watches", "cd ~/project && npm run build:watch")
        tmux.split_pane("watchers", "watches", "cd ~/project && npm run test:watch")
        tmux.split_pane("watchers", "watches", "cd ~/project && npm run dev")
        tmux.set_layout("watchers", "watches", "tiled")
    }

    if tmux.has_session("app") == false {
        tmux.new_session("app")
        tmux.new_window("app", "running", "")
    }

    // spawn iTerm2 windows, each attaching to a session
    spawn_terminal("~/", "tmux attach -t work")       // main monitor, desktop 1
    spawn_terminal("~/", "tmux attach -t watchers")   // main monitor, desktop 2
    spawn_terminal("~/", "tmux attach -t app")        // secondary monitor, desktop 2
}

register_task("setup", setup)
register_task("setup_watchers", setup_watchers)
```

---

## Implementation notes

- All methods run `tmux <subcommand>` via `std::process::Command`
- Follow the git module pattern in `src/modules/git/mod.rs`
- Register in `src/main.rs` alongside the git module
- `has_session` should check `output.status.success()` — tmux exits non-zero if session does not exist
- `send_keys` appends `Enter`; `interrupt` does not — implement as two separate functions
