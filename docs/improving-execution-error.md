# Improving ExecutionError

This is a backburner project. The goal is to make error reporting across the interpreter, parser, and git module structured and useful rather than a mix of panics, string messages, and silently swallowed results.

---

## Current State Analysis

### What's working

**Clean error propagation chain**
A `From` conversion chain exists: `ArgumentError → ExecutionError → RuntimeError`, enabling idiomatic `?` usage throughout the interpreter and runner layers.

**Idiomatic iterator error handling**
`collect::<Result<Vec<_>, _>>()?` is used correctly in three places (`interpreter/mod.rs:332–333`, `:436–437`, `:456–459`) for short-circuiting errors in iterator pipelines.

**Structured error types**
`ArgumentError` is well-designed — it has distinct variants (`InvalidCount`, `InvalidRange`, `InvalidType`) that carry meaningful contextual data. This is better than stringly-typed errors.

**`main` returns `Result`**
Using `fn main() -> Result<(), RuntimeError>` is clean and idiomatic.

### What's broken

**The entire parser layer still panics**
`Parser::parse()` returns `Vec<StatementType>`, not a `Result`. Every syntax error (unexpected token, no more tokens, invalid operator, etc.) is a `panic!`. This means any user DSL syntax error crashes the process.
- `src/parser/statements.rs:76,83,92`
- `src/parser/expressions.rs:118,306,351,355,360,408,459`

**All git errors are unwrapped**
`run_git_command` correctly returns `Result<String, GitError>`, but every single call site (11 sites) immediately calls `.unwrap()`. `GitError` has no `From` impl into `ExecutionError`, so there's no path to surface git failures gracefully.
- `src/modules/git/mod.rs:79,92,109,126,143,175,187,199,240,252,289`

**`ExecutionError` is a stringly-typed struct, not an enum**
The current `{ call_info: CallInfo, reason: String }` struct serializes all error context into an unstructured string. This makes it impossible to pattern-match on error kinds or attach structured metadata like source spans.

**`receiver.expect()` in list and dictionary builtins**
`interpreter/builtin/list.rs` (lines 20, 50, 79, 111) and `interpreter/builtin/dictionary.rs` (lines 20, 47, 77, 106) panic instead of returning `ExecutionError`.

**User-defined function arity panics, builtin arity returns `Err`**
`src/interpreter/function.rs:75` panics on wrong argument count for user-defined functions, while builtins correctly return `Err`. These should behave the same way.

**Task execution errors are silently swallowed**
In `src/runner.rs:84–87`, the task execution result (`task.execute(...)`) is discarded with `_ =`. Any `ExecutionError` from a running task is silently dropped.

**`RuntimeError` doesn't implement `std::error::Error`**
It only has `Display` and `Debug`. Without the `Error` trait, it can't be used with `Box<dyn Error>` or the broader ecosystem.

**REPL crashes on first error**
`src/runner.rs:50` propagates any parse/interpret error with `?`, terminating the REPL entirely on the first mistake. It should catch errors, print them, and continue.

**Minor string bugs**
- `builtin/mod.rs:35` — `"Exeucting"` typo
- `interpreter/builtin/dictionary.rs:47` — `"has can only be called on a dictionary"` (wrong function name in message)
- `interpreter/builtin/list.rs:76` — `Args::new("clear", ...)` should be `Args::new("pop", ...)`
- `interpreter/coerce.rs` — `Args::any()` always reports `expected: Callable` in error messages

---

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
- Add `From<GitError> for ExecutionError` wrapping into `ExecutionError::GitError(e)`
- Update all existing construction sites across `scope.rs`, `interpreter/mod.rs`, `function.rs`, `builtin/`, and `git/mod.rs` to use the appropriate variant
- `RuntimeError`'s `From<ExecutionError>` can stay as `.to_string()` for now — it just benefits from better variant messages
- Fix string bugs (typos, wrong function names) as part of this pass

---

## Phase 2 — Git Module: Connect `GitError`

The infrastructure exists, it's just not wired up. With `ExecutionError::GitError` added in Phase 1, this is now mechanical.

- Replace all 11 `.unwrap()` on `run_git_command(...)` with `?`
- Fix the 2 `.unwrap()` calls in `in_directory` on `canonicalize()` / `into_string()` — return an appropriate `ExecutionError` variant for invalid paths
- Change `parse_worktree_line` from returning `WorktreeInfo` + `.expect(...)` to `Result<WorktreeInfo, ExecutionError>`, propagate up through `worktrees()`
- Fix wrong `Args::new("worktrees", ...)` in `delete_branch` → `Args::new("delete_branch", ...)`

---

## Phase 3 — Remaining Interpreter Panics

All of these are inside functions that already return `Result` — with the new enum variants from Phase 1 it's a clean swap.

- `function.rs:75-78` — argument count mismatch → `Err(ExecutionError::ArgumentError(ArgumentError::InvalidCount { ... }))` to stay consistent with builtins
- `builtin/list.rs` (lines 20, 50, 79, 111) — `receiver.expect()` → return `ExecutionError`
- `builtin/dictionary.rs` (lines 20, 47, 77, 106) — same

---

## Phase 4 — `Args` / `coerce.rs` Fixes

- Add `ArgumentError::Missing { fn_name: String, index: usize }` variant
- Fix `Args::any()` to use `ArgumentError::Missing` instead of the fake `DataKind::Callable` placeholder, remove the `TODO` comment
- Add `Display` impl for `DataKind` with friendly names (`"integer"`, `"string"`, `"boolean"` etc.) instead of relying on `{:?}` in `ArgumentError`'s display

---

## Phase 5 — `TaskRunError` + `runner.rs` Swallowed Error

- Add `From<TaskRunError> for RuntimeError` in `runner.rs`
- Change `runner.rs:85` from `println!("Error: {}", err); Ok(())` → `return Err(err.into())` so task-not-found exits non-zero instead of pretending everything is fine
- Have the REPL catch errors, print them, and continue rather than terminating on the first mistake

---

## Phase 6 — `main.rs` Entry Point Panics

- Replace `main.rs` `.expect(...)` on missing CLI arg with a `RuntimeError` return
- Replace `main.rs` `panic!(...)` for invalid subcommand with a `RuntimeError` return
- Replace `env::current_dir()` and `.into_string()` unwraps with `?`

---

## Relationship to Source Location

The source location plan (`source-location-plan.md`) depends on this project. Specifically, Phase 4 of that plan (adding `span: Option<SourceSpan>` to `ExecutionError`) requires `ExecutionError` to be an enum with structured variants. Complete this project first before starting the source location work.
