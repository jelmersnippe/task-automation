# Project Structure Analysis

## Current Issues

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

5. **Add integration tests** — validates that refactors haven't broken anything, gives confidence for the steps below
6. **Split `builtins/`** — do this when the number of builtins justifies it, or when the task registry work begins
