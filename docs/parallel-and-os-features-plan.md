# Parallel Task Running & OS-Specific Features Plan

## Overview

Four phases, ordered by priority. Phases 3 and 4 are independent of Phase 1 and can be
started at any time.

---

## Phase 1 — Parallel `run()` with error isolation

**Goal**: `run("task")` runs a single task; `run(["task1", "task2"])` runs a list of tasks
sequentially but with isolated error handling — one task failing does not crash the others.

**Why not real threads?**
Current scope data uses `Rc<RefCell<...>>` which is `!Send`. Since all current task builtins
(`spawn_terminal`, git commands) already fire non-blocking subprocesses and return immediately,
a sequential loop achieves the same real-world parallelism with zero refactoring risk.

When a concrete blocking use case arises, migrate `FunctionDeclaration::defining_scope` from
`Rc<RefCell<Scope>>` to `Arc<Mutex<Scope>>` (Option D) — the `RuntimeContext` clone
infrastructure from this phase makes that migration straightforward.

**Scope semantics**: Each task receives a cloned `RuntimeContext`. Scope changes made during
task execution are discarded after the task completes. Tasks are fire-and-forget.

### Steps

| Step | Description | Files |
|------|-------------|-------|
| 1a | Wrap `GitRunner` trait object in `Arc` so `RuntimeContext` is cheaply cloneable | `src/modules/mod.rs`, `src/main.rs` |
| 1b | Add `Clone` to `TaskRegistry` | `src/task_management/mod.rs` |
| 1c | Add `Clone` to `RuntimeContext` | `src/main.rs` |
| 1d | Update `run()` builtin: detect `List` arg, loop sequentially, print errors to stderr per task without panicking | `src/interpreter/builtin/global.rs` |

---

## Phase 2 — macOS / iTerm2 terminal spawning

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
| 2a | Extract terminal spawning into `src/modules/terminal/mod.rs` with an OS-specific backend enum | new `src/modules/terminal/mod.rs` |
| 2b | macOS backend: build AppleScript string, run via `osascript -e "..."` | `src/modules/terminal/mod.rs` |
| 2c | Windows backend: existing `wt.exe wsl bash` logic moved here | `src/modules/terminal/mod.rs` |
| 2d | OS detection via `std::env::consts::OS` at runtime | `src/modules/terminal/mod.rs` |
| 2e | Update `spawn_terminal` in `global.rs` to delegate to the terminal module | `src/interpreter/builtin/global.rs` |

---

## Phase 3 — tmux module

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
| 3a | Create `src/modules/tmux/mod.rs` following the git module pattern | new `src/modules/tmux/mod.rs` |
| 3b | Implement functions: `new_session`, `new_window`, `split_pane(direction)`, `send_keys(text)`, `select_pane(index)`, `set_layout(layout)`, `attach` | `src/modules/tmux/mod.rs` |
| 3c | All functions execute `tmux <subcommand>` via `std::process::Command` | `src/modules/tmux/mod.rs` |
| 3d | Register module in `main.rs` | `src/main.rs`, `src/modules/mod.rs` |

---

## Phase 4 — macOS window management module

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
| 4a | Create `src/modules/window_management/mod.rs` | new `src/modules/window_management/mod.rs` |
| 4b | Implement `move(app, position)` — named positions to coordinates via AppleScript | `src/modules/window_management/mod.rs` |
| 4c | Implement `resize(app, width, height)` | `src/modules/window_management/mod.rs` |
| 4d | Implement `set_bounds(app, x, y, width, height)` for full manual control | `src/modules/window_management/mod.rs` |
| 4e | Screen resolution detection via `system_profiler SPDisplaysDataType` | `src/modules/window_management/mod.rs` |
| 4f | Register module in `main.rs` | `src/main.rs`, `src/modules/mod.rs` |

---

## Future: True thread parallelism (Option D)

When a blocking task use case arises (e.g. a module that waits on a long-running process
before continuing), migrate to real OS threads:

1. Change `FunctionDeclaration::defining_scope` from `Rc<RefCell<Scope>>` to `Arc<Mutex<Scope>>`
2. Propagate `Arc<Mutex<...>>` through `Scope`, `Interpreter`, and `Callable`
3. `RuntimeContext` clone infrastructure (Phase 1) remains unchanged
4. `run([...])` can then use `std::thread::spawn` per task

This is intentionally deferred — the `Rc` → `Arc` migration touches many files and should
be done when there is a concrete use case to validate against.
