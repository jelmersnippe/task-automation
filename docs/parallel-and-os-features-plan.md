# OS-Specific Features Plan

## Overview

`parallel()` is already implemented — it takes a list of callables and runs them on real OS
threads using `RuntimeContext` clones, with per-task error isolation via `mpsc::channel`.

Three phases remain, all independent of each other.

---

## Phase 1 — macOS / iTerm2 terminal spawning

**Goal**: Replace the hardcoded `wt.exe` (Windows Terminal/WSL) logic in `spawn_terminal`
with cross-platform support. macOS target is iTerm2 via `osascript`.

### Design

```dsl
spawn_terminal("/path/to/project")
spawn_terminal("/path/to/project", "cargo build && nvim .")
```

macOS spawns an iTerm2 window using AppleScript:

```applescript
tell application "iTerm2"
  create window with default profile
  tell current session of current window
    write text "cd /path && cmd"
  end tell
end tell
```

### Steps

| Step | Description | Files |
|------|-------------|-------|
| 1a | Extract terminal spawning into `src/modules/terminal/mod.rs` with an OS-specific backend enum | new `src/modules/terminal/mod.rs` |
| 1b | macOS backend: build AppleScript string, run via `osascript -e "..."` | `src/modules/terminal/mod.rs` |
| 1c | Windows backend: existing `wt.exe wsl bash` logic moved here | `src/modules/terminal/mod.rs` |
| 1d | OS detection via `std::env::consts::OS` at runtime | `src/modules/terminal/mod.rs` |
| 1e | Update `spawn_terminal` in `global.rs` to delegate to the terminal module | `src/interpreter/builtin/global.rs` |

---

## Phase 2 — tmux module

**Goal**: A `tmux` DSL module for programmatic terminal session/window/pane layout.

### Design

```dsl
tmux.new_session("work")
tmux.new_window("editor")
tmux.split_pane("horizontal")
tmux.send_keys("nvim .")
tmux.select_pane(0)
tmux.set_layout("even-horizontal")
tmux.attach()
```

### Steps

| Step | Description | Files |
|------|-------------|-------|
| 2a | Create `src/modules/tmux/mod.rs` following the git module pattern | new `src/modules/tmux/mod.rs` |
| 2b | Implement functions: `new_session`, `new_window`, `split_pane(direction)`, `send_keys(text)`, `select_pane(index)`, `set_layout(layout)`, `attach` | `src/modules/tmux/mod.rs` |
| 2c | All functions execute `tmux <subcommand>` via `std::process::Command` | `src/modules/tmux/mod.rs` |
| 2d | Register module in `main.rs` | `src/main.rs`, `src/modules/mod.rs` |

---

## Phase 3 — macOS window management module

**Goal**: Programmatically move and resize application windows using AppleScript.
No third-party tools required.

### Design

```dsl
windows.move("iTerm2", "top-left")
windows.move("iTerm2", "top-right")
windows.move("iTerm2", "center")
windows.resize("iTerm2", 1280, 800)
windows.set_bounds("iTerm2", 0, 0, 1280, 800)
```

Named positions (`"top-left"`, `"top-right"`, `"bottom-left"`, `"bottom-right"`, `"center"`)
map to coordinate calculations based on screen resolution.

### Steps

| Step | Description | Files |
|------|-------------|-------|
| 3a | Create `src/modules/window_management/mod.rs` | new `src/modules/window_management/mod.rs` |
| 3b | Implement `move(app, position)` — named positions to coordinates via AppleScript | `src/modules/window_management/mod.rs` |
| 3c | Implement `resize(app, width, height)` | `src/modules/window_management/mod.rs` |
| 3d | Implement `set_bounds(app, x, y, width, height)` for full manual control | `src/modules/window_management/mod.rs` |
| 3e | Screen resolution detection via `system_profiler SPDisplaysDataType` | `src/modules/window_management/mod.rs` |
| 3f | Register module in `main.rs` | `src/main.rs`, `src/modules/mod.rs` |
