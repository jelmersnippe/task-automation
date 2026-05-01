# Project Structure Analysis

## Current Issues

### No Integration Test Layer (Medium Priority)

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
│   ├── mod.rs                  ← Lexer struct + tokenize()
│   └── tests.rs
│
├── parser/
│   ├── mod.rs                  ← Parser struct + token utilities (peek, next, expect, match)
│   ├── expressions.rs          ← AST expression types + expression parsing
│   ├── statements.rs           ← AST statement types + statement parsing
│   └── tests/
│
├── interpreter/
│   ├── mod.rs                  ← Interpreter struct + interpret_statement / interpret_expression
│   ├── datatype.rs             ← DataType, Callable, Display/PartialEq impls
│   ├── scope.rs                ← Scope only
│   ├── coerce.rs               ← Args validation, type coercion helpers
│   ├── function.rs
│   ├── list.rs
│   ├── dictionary.rs
│   ├── builtin/                ← split from single builtin.rs (done)
│   │   ├── mod.rs              ← BuiltinFn type, ExecutionError, BUILTINS registry
│   │   ├── global.rs           ← print, len, run, register_task, spawn_terminal
│   │   ├── list.rs             ← list methods
│   │   └── dictionary.rs       ← dict methods
│   └── tests/
│
├── modules/
│   └── git/                    ← git command wrappers
│
└── task_management/            ← task registry and execution

tests/                          ← crate-level integration tests (full pipeline)
    integration.rs              ← or split by feature area
```

---

## Suggested Order of Changes

1. **Add integration tests** — validates that refactors haven't broken anything, gives confidence for the steps below
