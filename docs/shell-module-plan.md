# Shell Module Plan

## Overview

Replace the global `spawn_terminal` builtin with a `shell` module that exposes two distinct
operations: opening an interactive terminal window (optionally running a command first, then
keeping the shell alive), and running a command in a terminal window that exits when the
command finishes.

`spawn_terminal` is removed from `BUILTINS` in `global.rs` once the module is in place.

---

## API

```dsl
// Open an interactive terminal at path, drop into shell
shell.open("/path/to/project")

// Open an interactive terminal, run a command, then keep the shell alive
shell.open("/path/to/project", "nvim .")

// Open a terminal, run a command, close when done
shell.run("/path/to/project", "cargo build")
```

### `shell.open(path)` / `shell.open(path, cmd)`

Opens a new terminal window at `path`. If `cmd` is provided, it runs first. The shell stays
alive after the command exits — equivalent to ending with `exec $SHELL`. Use this for
interactive windows: editors, REPLs, long-lived processes you want to interact with.

### `shell.run(path, cmd)`

Opens a new terminal window at `path`, runs `cmd`, and exits when it finishes. Use this for
fire-and-forget operations: builds, installs, one-off scripts.

---

## Platform behaviour

### macOS — iTerm2 via `osascript`

`osascript` accepts multiple `-e` flags, each treated as one line of AppleScript. This is
the cleanest way to drive it from Rust — no multiline string escaping, each line is a
separate `&str`.

The approach: create a window with the default iTerm2 profile (which opens the user's
default shell), then immediately send a command to it via `write text`. This is equivalent
to typing the command yourself the moment the window opens.

**Rust `Command` for `shell.open(path)`:**
```rust
Command::new("osascript")
    .args(["-e", "tell application \"iTerm2\""])
    .args(["-e", "set w to (create window with default profile)"])
    .args(["-e", "tell current session of w"])
    .args(["-e", &format!("write text \"cd '{}' && exec $SHELL\"", path)])
    .args(["-e", "end tell"])
    .args(["-e", "end tell"])
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
```

**Rust `Command` for `shell.open(path, cmd)`:**
```rust
// write text line becomes:
&format!("write text \"cd '{}' && {} && exec $SHELL\"", path, cmd)
```

**Rust `Command` for `shell.run(path, cmd)`:**
```rust
// write text line becomes:
&format!("write text \"cd '{}' && {}\"", path, cmd)
```
The window closes when the command exits if the iTerm2 profile has
"When the session ends: Close the tab" enabled. This is a user iTerm2 setting — it cannot
be forced from AppleScript, so document it as a requirement.

**Quoting note:** paths are single-quoted in the bash command (`cd '/path/to/dir'`) to
handle spaces. Paths containing single quotes are unsupported for now — document as a known
limitation rather than adding full shell escaping on day one.

### Windows — Windows Terminal + WSL

**Rust `Command` for `shell.open(path)`:**
```rust
Command::new("wt.exe")
    .args(["wsl", "bash", "-lc", &format!("cd '{}' && exec bash", path)])
    .stdin(Stdio::null())
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
```

**`shell.open(path, cmd)`:**
```rust
.args(["wsl", "bash", "-lc", &format!("cd '{}' && {} && exec bash", path, cmd)])
```

**`shell.run(path, cmd)`:**
```rust
.args(["wsl", "bash", "-lc", &format!("cd '{}' && {}", path, cmd)])
```
Windows Terminal closes the tab automatically when the shell exits.

---

## Platform-specific code in Rust

There are two ways to branch on OS in Rust. Use whichever fits the situation.

### Option A — `#[cfg]` attributes (compile-time)

The compiler includes only the branch that matches the target OS. Dead branches are not
compiled at all, so you get a compile error if you accidentally use a macOS-only API on
Windows.

```rust
fn spawn_open(path: &str, cmd: Option<&str>) -> Result<(), ExecutionError> {
    #[cfg(target_os = "macos")]
    return spawn_open_macos(path, cmd);

    #[cfg(target_os = "windows")]
    return spawn_open_windows(path, cmd);

    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    return Err(ExecutionError::new(
        CallInfo::new("shell.open"),
        "shell module is not supported on this platform",
    ));
}
```

Or at the function definition level, define the function body differently per platform:

```rust
#[cfg(target_os = "macos")]
fn spawn_open(path: &str, cmd: Option<&str>) -> Result<(), ExecutionError> {
    // osascript logic
}

#[cfg(target_os = "windows")]
fn spawn_open(path: &str, cmd: Option<&str>) -> Result<(), ExecutionError> {
    // wt.exe logic
}
```

The downside: you can only compile and test the branch that matches your current machine.
Cross-platform bugs are only caught when running on the target OS.

### Option B — `std::env::consts::OS` (runtime)

A plain string match at runtime. Both branches are compiled into the binary regardless of
the target.

```rust
fn spawn_open(path: &str, cmd: Option<&str>) -> Result<(), ExecutionError> {
    match std::env::consts::OS {
        "macos"   => spawn_open_macos(path, cmd),
        "windows" => spawn_open_windows(path, cmd),
        other => Err(ExecutionError::new(
            CallInfo::new("shell.open"),
            &format!("shell module is not supported on '{}'", other),
        )),
    }
}
```

The upside: both backends compile on every platform, so you catch type errors and API
mismatches in CI even when building on macOS. The downside: the binary always includes
both backends, and platform-specific dependencies (if any were added later) would need
to be feature-flagged separately.

### Which to use here

Use **Option A** (`#[cfg]`). It is the idiomatic Rust approach for platform-specific code
and the runtime dispatch benefit of Option B (both backends compile everywhere) only matters
if you have CI running on all target platforms — which this project doesn't.

Define each backend function with its `#[cfg]` attribute, then add a fallback for any other
platform so callers get a clean error message rather than a compile error:

```rust
#[cfg(target_os = "macos")]
fn spawn_open(path: &str, cmd: Option<&str>) -> Result<(), ExecutionError> {
    // osascript logic
}

#[cfg(target_os = "windows")]
fn spawn_open(path: &str, cmd: Option<&str>) -> Result<(), ExecutionError> {
    // wt.exe logic
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn spawn_open(_path: &str, _cmd: Option<&str>) -> Result<(), ExecutionError> {
    Err(ExecutionError::new(
        CallInfo::new("shell.open"),
        "shell module is not supported on this platform",
    ))
}
```

Apply the same pattern to `spawn_run`.

---

## Implementation steps

| Step | Description | Files |
|------|-------------|-------|
| 1 | Create `src/modules/shell/mod.rs` | new file |
| 2 | Implement `open_macos(path, cmd?)` and `run_macos(path, cmd)` using the `osascript` `-e` pattern above | `src/modules/shell/mod.rs` |
| 3 | Implement `open_windows(path, cmd?)` and `run_windows(path, cmd)` using the `wt.exe wsl bash -lc` pattern | `src/modules/shell/mod.rs` |
| 4 | Implement public `open` and `run` functions that dispatch via `std::env::consts::OS` | `src/modules/shell/mod.rs` |
| 5 | Implement `create_shell_module()` following the git module pattern, wiring `open` and `run` as module functions | `src/modules/shell/mod.rs` |
| 6 | Register module in `main.rs` alongside the git module | `src/main.rs`, `src/modules/mod.rs` |
| 7 | Remove `spawn_terminal` from `BUILTINS` in `global.rs` | `src/interpreter/builtin/global.rs` |

---

## Notes

- Both functions are fire-and-forget — `.spawn()` not `.output()`. The DSL call returns
  immediately without waiting for the terminal window to close
- If `osascript` or `wt.exe` is not found, the `Command::spawn()` call returns an `Err`
  — map this to an `ExecutionError` with a clear message rather than silently ignoring it
  (unlike the current `spawn_terminal`)
- `open` takes 1–2 arguments; `run` takes exactly 2 — use `Args::range(1, 2)` and
  `Args::exact(2)` respectively
