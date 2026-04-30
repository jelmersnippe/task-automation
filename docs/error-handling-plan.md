# Error Handling Improvement Plan

## Phase 1 — Refactor `ExecutionError` into an Enum

**Goal:** Replace the overloaded `{ call_info: CallInfo, reason: String }` struct with a proper enum where each variant carries exactly the fields it needs. This eliminates all `CallInfo::new("")` abuse and makes error information structured rather than serialized into a string.

Replace the current `ExecutionError` struct and `CallInfo` struct in `builtin/mod.rs` with an enum:

```rust
pub enum ExecutionError {
    UndeclaredVariable { name: String },
    DuplicateIdentifier { name: String },
    NotDeclared { name: String },          // update_variable: identifier not yet declared
    TypeError { expected: DataKind, found: DataKind, context: String },
    NotCallable,
    NotAssignable,
    InvalidAccessor,
    ArgumentError(ArgumentError),
    GitError(GitError),                    // enables ? in git module
    Custom { fn_name: String, reason: String }, // for builtins that need freeform messages
}
```

- Remove `CallInfo` and `ExecutionError::new(call_info, reason)` entirely
- Implement `Display` for `ExecutionError` with a tailored message per variant
- Update `From<ArgumentError> for ExecutionError` to wrap into `ExecutionError::ArgumentError(...)`
- Update all existing construction sites across `scope.rs`, `interpreter/mod.rs`, `function.rs`, `builtin/`, and `git/mod.rs` to use the appropriate variant
- `RuntimeError`'s `From<ExecutionError>` can stay as `.to_string()` for now — it just benefits from better variant messages

---

## Phase 2 — Git Module: Connect `GitError`

The infrastructure exists, it's just not wired up. With `ExecutionError::GitError` added in Phase 1, this is now mechanical.

- Add `From<GitError> for ExecutionError` wrapping into `ExecutionError::GitError(e)` (enabled by Phase 1)
- Replace all 10 `.unwrap()` on `run_git_command(...)` with `?`
- Fix the 2 `.unwrap()` calls in `in_directory` on `canonicalize()` / `into_string()` — return an appropriate `ExecutionError` variant for invalid paths
- Change `parse_worktree_line` from returning `WorktreeInfo` + `.expect(...)` to `Result<WorktreeInfo, ExecutionError>`, propagate up through `worktrees()`
- Fix wrong `Args::new("worktrees", ...)` in `delete_branch` → `Args::new("delete_branch", ...)`

---

## Phase 3 — Interpreter: Replace All `panic!` with `Err(...)`

All of these are inside functions that already return `Result` — with the new enum variants from Phase 1 it's a clean swap.

- `mod.rs:144` — `if` condition not boolean → `Err(ExecutionError::TypeError { ... })`
- `mod.rs:186` — `while` condition not boolean → same, and fix the copy-pasted message that says "if statement" instead of "while statement"
- `mod.rs:224` — invalid accessor target → `Err(ExecutionError::InvalidAccessor)`
- `mod.rs:227` — expression not assignable → `Err(ExecutionError::NotAssignable)`
- `mod.rs:252-317` — all panics inside `interpret_binary_expression` → `Err(ExecutionError::TypeError { ... })`
- `mod.rs:403` — accessor on wrong type → `Err(ExecutionError::InvalidAccessor)`
- `mod.rs:429` — invalid dictionary key → `Err(ExecutionError::TypeError { ... })`
- `function.rs:75-78` — argument count mismatch → `Err(ExecutionError::ArgumentError(ArgumentError::InvalidCount { ... }))` to stay consistent with builtins

---

## Phase 4 — `TaskRunError` + `runner.rs` Swallowed Error

- Add `From<TaskRunError> for RuntimeError` in `runner.rs`
- Change `runner.rs:85` from `println!("Error: {}", err); Ok(())` → `return Err(err.into())` so task-not-found exits non-zero instead of pretending everything is fine

---

## Phase 5 — `main.rs` Entry Point Panics

- Replace `main.rs:40` `.expect(...)` on missing CLI arg with a `RuntimeError` return
- Replace `main.rs:48` `panic!(...)` for invalid subcommand with a `RuntimeError` return
- Remove debug `println!("{}", arg)` from `runner.rs:103`

---

## Phase 6 — `Args` / `coerce.rs` Fixes

- Add `ArgumentError::Missing { fn_name: String, index: usize }` variant
- Fix `Args::any()` to use `ArgumentError::Missing` instead of the fake `DataKind::Callable` placeholder, remove the `TODO` comment
- Add `Display` impl for `DataKind` with friendly names (`"integer"`, `"string"`, `"boolean"` etc.) instead of relying on `{:?}` in `ArgumentError`'s display

---

## Phase 7 — String Fixes

Fast pass, no logic changes.

- `builtin/mod.rs` — `"Exeucting"` → `"Executing"` (will likely move during Phase 1 refactor)
- `scope.rs:74` — `"has not declared"` → `"has not been declared"` (superseded by Phase 1 if `NotDeclared` variant gets a proper Display message)
- `builtin/list.rs:76` — `Args::new("clear", ...)` → `Args::new("pop", ...)`
- `builtin/dictionary.rs:47` — `"has can only be called on a dictionary"` → `"delete can only be called on a dictionary"`
