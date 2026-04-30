# Result Handling Analysis

## Pros

**Clean error propagation chain**
A well-structured `From` conversion chain exists: `ArgumentError → ExecutionError → RuntimeError`, enabling idiomatic `?` usage throughout the interpreter and runner layers.

**Idiomatic iterator error handling**
`collect::<Result<Vec<_>, _>>()?` is used correctly in three places (`interpreter/mod.rs:332–333`, `:436–437`, `:456–459`) for short-circuiting errors in iterator pipelines.

**Structured error types**
`ArgumentError` is well-designed — it has distinct variants (`InvalidCount`, `InvalidRange`, `InvalidType`) that carry meaningful contextual data. This is better than stringly-typed errors.

**`main` returns `Result`**
Using `fn main() -> Result<(), RuntimeError>` is clean and idiomatic.

---

## Cons

### High Severity

**The entire parser layer still panics**
`Parser::parse()` returns `Vec<StatementType>`, not a `Result`. Every syntax error (unexpected token, no more tokens, invalid operator, etc.) is a `panic!`. This means any user DSL syntax error crashes the process. This is likely the most impactful missing piece.
- `src/parser/statements.rs:76,83,92`
- `src/parser/expressions.rs:118,306,351,355,360,408,459`

**All git errors are unwrapped**
`run_git_command` correctly returns `Result<String, GitError>`, but every single call site (11 sites) immediately calls `.unwrap()`. `GitError` has no `From` impl into `ExecutionError`, so there's no path to surface git failures gracefully.
- `src/modules/git/mod.rs:79,92,109,126,143,175,187,199,240,252,289`

### Medium Severity

**Interpreter type/operator mismatches still panic (14+ sites)**
Binary expression type mismatches, invalid accessor uses, and method-not-found are all `panic!` in `src/interpreter/mod.rs` and `src/interpreter/datatype.rs`. These should return `ExecutionError`.

**Inconsistency: user-defined function arity panics, builtin arity returns `Err`**
`src/interpreter/function.rs:75` panics on wrong argument count for user-defined functions, while builtins correctly return `Err(ExecutionError::new(...))` via `Args::exact()`. These should behave the same way.

**Task execution errors are silently swallowed**
In `src/runner.rs:84–87`, the task execution result (`task.execute(...)`) is discarded with `_ =`. Any `ExecutionError` from a running task is silently dropped. The `Err` branch only prints and does not propagate.

### Low-Medium Severity

**`RuntimeError` doesn't implement `std::error::Error`**
It only has `Display` and `Debug`. Without the `Error` trait, it can't be used with `Box<dyn Error>` or the broader ecosystem.

**REPL crashes on first error**
`src/runner.rs:50` propagates any parse/interpret error with `?`, terminating the REPL entirely on the first mistake. It should catch errors, print them, and continue.

### Low Severity

- **Wrong error message**: `while` condition check at `src/interpreter/mod.rs:186` says "if statement" in the panic message
- **Copy-paste bug**: `src/interpreter/builtin/dictionary.rs:47,77` — `delete` and `clear` methods use `expect("has can only be called on a dictionary")`
- **`Args::any()` hardcodes wrong `expected_type`**: `src/interpreter/coerce.rs:305` always reports `expected: Callable` — there is a `// TODO` comment there acknowledging this
- **`main.rs` unwraps OS calls**: `current_dir()` and `into_string()` at `src/main.rs:26,28` could be propagated with `?`

---

## Overall Assessment

The foundation is solid — the conversion chain, structured error types, and `?` usage in the interpreter core are all good patterns. However, the two highest-impact areas (the parser and git module) are largely untouched. Most user-facing errors — syntax errors and git failures — still produce crashes rather than clean error messages.
