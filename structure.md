# Project Structure Analysis

## Current Issues

### 1. `scope.rs` Is a Dumping Ground (High Priority)

`scope.rs` currently holds three unrelated things: `DataType` (the runtime value type), `Callable`, and `Scope` (the execution environment). `DataType` is the most widely imported type in the entire codebase, yet it lives in a file named after something else. Every module does `use crate::interpreter::scope::DataType`, which is unintuitive — you wouldn't expect a value type to live in a file called `scope`.

**Fix:** Split into two files:
- `value.rs` — `DataType`, `Callable`, and their `Display`/`PartialEq` impls
- `scope.rs` — `Scope` only, importing `DataType` from `value.rs`

---

### 2. `lexer/lexer.rs` Double-Nesting (Medium Priority)

The full import path for the lexer function is `crate::lexer::lexer::lexer` — module, file, and function all share the same name. `lexer/mod.rs` currently does nothing but re-export from `lexer/lexer.rs`. The inner file has no reason to exist separately.

**Fix:** Merge `lexer/lexer.rs` into `lexer/mod.rs`. While there, rename the `lexer()` free function to a `Lexer` struct with a `tokenize()` method, consistent with how `Parser` and `Interpreter` are structured:

```rust
// Before
let tokens = lexer(input);

// After
let tokens = Lexer::new(input).tokenize();
```

---

### 3. Circular Dependency: `parser/expressions.rs` → `interpreter` (High Priority)

`parser/expressions.rs` imports and calls `interpret_expression` from the interpreter via `Parameters::resolve()`. This means the parser layer depends on the interpreter layer — a fundamental layering violation. Parser AST types should be pure data structures with no runtime dependencies.

**Fix:** Move `Parameters::resolve()` out of `expressions.rs` and into the interpreter, where parameter resolution belongs. The `Parameters` type itself can stay in the parser as a plain data container; only the method that calls `interpret_expression` needs to move.

---

### 4. `builtin.rs` Will Become a Monolith (Medium Priority)

Three builtins fit comfortably in one file today. The long-term goal includes a task registry with filesystem, process, environment, and terminal operations. All of these in one file will become hard to navigate and maintain.

**Fix:** Convert to a `builtins/` subdirectory, grouping by concern:

```
interpreter/builtins/
    mod.rs          ← BUILTINS registry, Builtin struct, BuiltinFn type definition
    io.rs           ← print, read_line, etc.
    terminal.rs     ← spawn_terminal and future terminal operations
    collections.rs  ← dict_has, dict_delete, dict_clear, len
```

Each category file only exports its functions; `mod.rs` assembles the `BUILTINS` list and re-exports the `Builtin` and `BuiltinFn` types.

---

### 5. No Integration Test Layer (Medium Priority)

All tests currently live inside their modules as `#[cfg(test)]` blocks. This is correct for unit tests, but there is no test that exercises the full pipeline: DSL string → tokens → AST → runtime output. These end-to-end tests belong in a `tests/` directory at the crate root, which is Rust's built-in location for integration tests (they compile as a separate crate with access only to the public API).

**Fix:** Add `tests/` at the crate root with files that run `.dsl` fixtures through the full pipeline and assert on the resulting scope state or output:

```
tests/
    integration.rs      ← or split by feature: variables.rs, functions.rs, etc.
```

The existing `.dsl` files in `dsl/` are natural candidates to drive these tests.

---

### 6. `main.rs` Mixes Concerns (Low Priority)

`main.rs` currently handles argument parsing, the REPL loop, file reading, and the lex → parse → interpret pipeline all in one place. When the pipeline grows (error handling, a compilation step, task resolution), this file will accumulate unrelated changes.

**Fix:** Extract the pipeline into a dedicated `runner.rs` module. `main.rs` should only handle I/O and argument dispatch; the pipeline is an internal concern.

```rust
// main.rs after
fn main() {
    let arg = std::env::args().nth(1).expect("...");
    if arg == "repl" { repl(); return; }
    runner::run_file(std::path::PathBuf::from(arg));
}

// runner.rs
pub fn run(input: String) { ... }
pub fn run_file(path: PathBuf) { ... }
```

---

## Target Structure

This is the recommended layout as the project grows toward the task registry goal:

```
src/
│
├── main.rs                     ← argument parsing, REPL, file dispatch only
├── runner.rs                   ← pipeline: tokenize → parse → interpret
│
├── lexer/
│   ├── mod.rs                  ← Lexer struct + tokenize() (merged from lexer.rs)
│   └── tests.rs
│
├── parser/
│   ├── mod.rs                  ← Parser struct + token utilities (peek, next, expect, match)
│   ├── expressions.rs          ← AST expression types + expression parsing (no interpreter imports)
│   ├── statements.rs           ← AST statement types + statement parsing
│   └── tests/
│
├── interpreter/
│   ├── mod.rs                  ← Interpreter struct + interpret_statement / interpret_expression
│   ├── value.rs                ← DataType, Callable, Display/PartialEq impls (split from scope.rs)
│   ├── scope.rs                ← Scope only
│   ├── function.rs
│   ├── list.rs
│   ├── helpers.rs
│   ├── builtins/               ← split from single builtin.rs
│   │   ├── mod.rs              ← BUILTINS registry, Builtin struct, BuiltinFn type
│   │   ├── io.rs               ← print, read_line
│   │   ├── terminal.rs         ← spawn_terminal
│   │   └── collections.rs      ← len, dict_has, dict_delete, dict_clear
│   └── tests/
│
└── task/                       ← future: task registry and runner
    ├── mod.rs
    ├── registry.rs             ← task definitions and lookup
    └── runner.rs               ← task execution logic

tests/                          ← crate-level integration tests (full pipeline)
    integration.rs              ← or split by feature area
```

---

## Suggested Order of Changes

The issues above are not independent — some must come before others to avoid doing work twice.

1. **Fix the circular dependency first** (`expressions.rs` → `interpreter`) — everything else is easier once the layers are clean
2. **Split `scope.rs` into `value.rs` + `scope.rs`** — unblocks cleaner imports everywhere
3. **Merge `lexer/lexer.rs` into `lexer/mod.rs`** — self-contained, no downstream impact
4. **Extract `runner.rs` from `main.rs`** — small, enables integration tests
5. **Add integration tests** — validates that refactors haven't broken anything, gives confidence for the steps below
6. **Split `builtins/`** — do this when the number of builtins justifies it, or when the task registry work begins
7. **Add `task/` module** — once the above is clean, the task registry has a stable foundation to build on
