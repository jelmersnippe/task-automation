# OS-Specific Features Plan

## Overview

`parallel()` and the `shell` module are complete — see the readme for their APIs.

Two phases remain, both independent of each other.

---

## Phase 1 — tmux module

Full API design, philosophy, usage examples, and implementation notes are in `tmux-module-plan.md`.

Implementation steps:

| Step | Description | Files |
|------|-------------|-------|
| 1a | Create `src/modules/tmux/mod.rs` following the git module pattern | new `src/modules/tmux/mod.rs` |
| 1b | Implement the API defined in `tmux-module-plan.md` | `src/modules/tmux/mod.rs` |
| 1c | All functions execute `tmux <subcommand>` via `std::process::Command` | `src/modules/tmux/mod.rs` |
| 1d | Register module in `main.rs` | `src/main.rs`, `src/modules/mod.rs` |

---

## Phase 2 — macOS window management module

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
| 2a | Create `src/modules/window_management/mod.rs` | new `src/modules/window_management/mod.rs` |
| 2b | Implement `move(app, position)` — named positions to coordinates via AppleScript | `src/modules/window_management/mod.rs` |
| 2c | Implement `resize(app, width, height)` | `src/modules/window_management/mod.rs` |
| 2d | Implement `set_bounds(app, x, y, width, height)` for full manual control | `src/modules/window_management/mod.rs` |
| 2e | Screen resolution detection via `system_profiler SPDisplaysDataType` | `src/modules/window_management/mod.rs` |
| 2f | Register module in `main.rs` | `src/main.rs`, `src/modules/mod.rs` |
