# Docs Overview & Recommended Work Order

This file maps out the dependency graph between all open refactors and todos, and gives a
recommended order to work through them without doing anything twice.

---

## Dependency Graph

```
Fix critical bugs (unary, spawn_terminal, REPL stdin)
        │
        ▼
improving-execution-error  ──────────────────────────────┐
  (ExecutionError enum, git unwraps, remaining panics)   │
        │                                                 │
        ├──────────────────────────────────────────┐      │
        ▼                                          ▼      │
source-location-plan                    int-float-split   │
  (Token spans, ParseError,             (Int/Float        │
   AST nodes, error threading)           DataType split)  │
        │                                          │      │
        └──────────────┬────────────────────────── ┘      │
                       ▼                                  │
              code-quality cleanup  ◄─────────────────────┘
                (Rust idioms,
                 dead code,
                 duplication)
                       │
                       ▼
              integration tests
              (structure.md)
                       │
                       ▼
              language features
              (else, comments,
               for, global)
```

`window-management` is independent and can be done at any point.

---

## Phase 1 — Fix Critical Bugs

**From `todo.md`**

These are quick, isolated fixes with no dependencies. Do them first so the runtime is not
broken while doing the larger refactors.

- Fix unary expression panic — `interpreter/mod.rs` `interpret_unary_expression`
- Fix `spawn_terminal` single-argument panic — `interpreter/builtin/global.rs`
- Fix silent stdin failure in REPL — `runner.rs:63`
- Fix integer validation in `expect_int` — `interpreter/coerce.rs`
- Propagate `main.rs` startup errors instead of panicking

---

## Phase 2 — Refactor `ExecutionError` into an Enum

**`docs/refactor/improving-execution-error.md`**

This is the most important structural prerequisite. Everything downstream (source locations,
the Pratt parser, `ParseError`) benefits from having a proper enum instead of a stringly-typed
struct. Do this before touching the parser or interpreter in any deep way.

Work through the phases in order:

1. Replace `ExecutionError` struct with enum — scope, interpreter, builtins, git module
2. Wire up git errors — replace all `.unwrap()` on `run_git_command`
3. Fix remaining interpreter panics — `function.rs`, `builtin/list.rs`, `builtin/dictionary.rs`
4. Fix `Args` / `coerce.rs` — `ArgumentError::Missing`, `DataKind` display names
5. Fix `TaskRunError` propagation and REPL error recovery — `runner.rs`
6. Fix `main.rs` entry point panics

Also from `todo.md`, fold in while you're in these files:
- Convert function arity mismatch (`function.rs:75`) to `ExecutionError`
- Propagate task execution errors in `runner.rs`
- Fix `PartialEq` for `Callable` variants

---

## Phase 3 — Source Location

**`docs/todo/source-location-plan.md`**

Requires Phase 2 complete — Phase 4 of the source location plan explicitly depends on
`ExecutionError` being an enum before adding `span: Option<SourceSpan>` to it.

Work through the phases in order:

1. Add `line`/`column` to `Token` in the lexer
2. Introduce `ParseError` enum — replace all parser `panic!` calls
3. Embed `SourceSpan` in key AST nodes
4. Add `span: Option<SourceSpan>` to `ExecutionError`
5. Thread spans through all interpreter error sites
6. Update tests incrementally alongside each phase

This also covers these `todo.md` architecture items:
- Add source location to `Token` and thread through errors
- Separate parse errors from runtime errors

---

## Phase 4 — Int/Float Split

**`docs/refactor/int-float-split-plan.md`**

Independent of source location but touches many of the same files as Phase 2. Doing it after
Phase 2 means the error infrastructure is in place, so type errors in binary expressions can
return structured `ExecutionError` variants rather than panics.

Work through the phases in order:

1. Replace `DataType::Number(f32)` with `Int(i64)` / `Float(f64)` in `datatype.rs`
2. No lexer changes required
3. Update `LiteralType` in the AST and parser
4. Update `interpret_binary_expression` for all numeric combinations
5. Update `coerce.rs` — `expect_int`, add `expect_float`, `Args::float()`
6. Add `to_int()` and `to_float()` cast builtins
7. Update all tests

This covers the `todo.md` code quality item:
- Replace `f32` with `f64` / introduce `Int` and `Float` variants

---

## Phase 5 — Code Quality Cleanup

**`docs/refactor/code-quality.md`**

After Phases 2–4, the core data structures and error types are stable. This is the right time
to do the Rust idiom cleanup — changing signatures (e.g. `&String` → `&str`) won't conflict
with in-progress structural changes.

Work through the sections in order (high priority first within each):

**Dead code & correctness**
- Remove dead `executed_statements` in `execute_statements`
- Fix `is_valid_string` called twice in `text_to_token`

**Rust idioms**
- `&String` → `&str` across lexer, scope, and list functions
- Remove explicit `return` at end of functions (Clippy)
- `TokenKind` derive `Copy`
- Unnecessary `.clone()` on owned data — four specific sites
- Audit remaining `RefCell` usage
- Verbose `match` on `Option<char>` in lexer → `matches!`
- `iter().nth(i)` → direct indexing in `list.rs`
- Struct field shorthand in `scope.rs` and `expressions.rs`
- Remove redundant `.into_iter()` on `chars()`
- Helper constructor for `Rc<RefCell<T>>` where still used

**Duplication & patterns**
- Helper for `Rc::new(DataType::Undefined)` in void builtins
- Dict key resolution — call `expect_string` instead of re-implementing
- Intermediate `Vec` when building a dictionary — use `FromIterator`
- String concatenation branches in `interpret_binary_expression`
- Duplicate `Display` implementations for `DataType` and `LiteralType`

Also from `todo.md`:
- Remove dead code in `lexer/mod.rs` (unreachable string-accumulation guard)
- Document or rewrite `insert_new_right` — at minimum add inline comments

---

## Phase 6 — Pratt Parser

**From `todo.md`**

Once the code is clean and `ParseError` exists (Phase 3), rewrite binary expression parsing
as a Pratt parser. This replaces the non-standard `insert_new_right` approach and makes the
parser significantly easier to extend with new operators.

---

## Phase 7 — Integration Tests

**`docs/notes/structure.md`**

Add a `tests/` directory at the crate root with end-to-end tests that run `.dsl` source
through the full pipeline and assert on scope state or output. The `.dsl` files in `dsl/`
are natural fixtures to drive these.

Do this after the major structural changes (Phases 2–5) are stable, so the tests don't need
to be rewritten mid-refactor. The test improvements analysis in
`docs/notes/test-improvements.md` describes the gaps to fill at each layer.

---

## Phase 8 — Language Features

**From `todo.md`**

With a solid interpreter, clean error handling, and a test suite in place, add the missing
language features in rough order of user impact:

1. `else` / `else if` — token already lexed, parser just needs the branch
2. Comments (`//`) — strip in the lexer before tokenizing
3. `for` / iteration over collections — biggest user impact after `while`
4. `global` keyword
5. `print` with multiple arguments
6. `typeof(x)` builtin

---

## Anytime — Window Management Module

**`docs/todo/window-management.md`**

Fully self-contained new module with no dependencies on any of the above. Can be picked up
between any two phases or done in parallel.
