# OS-Specific Features Plan

## Overview

`parallel()` is already implemented — it takes a list of callables and runs them on real OS
threads using `RuntimeContext` clones, with per-task error isolation via `mpsc::channel`.

Three phases remain, all independent of each other.

---

## Phase 1 — Shell module

Full design and implementation steps are in `shell-module-plan.md`.

This replaces the global `spawn_terminal` builtin with a `shell` module exposing `shell.open(path, cmd?)` and `shell.run(path, cmd)`, with macOS (iTerm2 via `osascript`) and Windows (`wt.exe wsl bash`) backends.

---

## Phase 2 — tmux module

Full API design, philosophy, usage examples, and implementation notes are in `tmux-module-plan.md`.

Implementation steps:

| Step | Description | Files |
|------|-------------|-------|
| 2a | Create `src/modules/tmux/mod.rs` following the git module pattern | new `src/modules/tmux/mod.rs` |
| 2b | Implement the API defined in `tmux-module-plan.md` | `src/modules/tmux/mod.rs` |
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
