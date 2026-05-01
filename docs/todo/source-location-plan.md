# Source Location Plan

## Design Decision: Spans in Token + AST Nodes

Source spans are added to `Token` in the lexer, then embedded into AST nodes as the parser builds the tree. This gives full coverage — both parse errors and runtime errors point to the exact source line. The interpreter pulls the span off the AST node when generating an `ExecutionError`.

---

## Phase 1 — Lexer: Add `line` and `column` to `Token`

**Goal:** Every token knows exactly where it came from in the source.

- Add `line: usize` and `column: usize` fields to the `Token` struct in `lexer/mod.rs`
- Update `Token::new()` to accept `line` and `column`
- Update `Lexer::tokenize()` to track position as it iterates:
  - Initialize `line: usize = 1` and `column: usize = 1`
  - Increment `line` and reset `column` to 1 on `\n`
  - Increment `column` on every other character
  - Pass `line`/`column` when constructing tokens in all `tokens.push(...)` call sites (~10 across the method)
  - Multi-character tokens (`!=`, `==`, `>=`, `<=`, `&&`, `||`) should record the position of their first character
  - The `text_to_token` helper currently takes only a `String` — it will need `line`/`column` parameters passed through so flushed tokens also get the correct position
- Fix the `panic!` in `lexer/mod.rs:279` for invalid logical operators — change to emit a `TokenKind::Illegal` token with position instead of panicking (the parser will catch it as an error with location)
- Update all lexer tests in `lexer/tests.rs` that construct `Token` directly to include the new fields

---

## Phase 2 — Parser: `ParseError` with Source Location

**Goal:** Parser failures return typed errors with the exact line and column.

- Add a `ParseError` enum in `parser/mod.rs`:
  - `UnexpectedToken { expected: TokenKind, found: TokenKind, line: usize, column: usize }`
  - `UnexpectedEndOfInput { expected: TokenKind }`
  - `InvalidBreak { line: usize, column: usize }`
  - `InvalidContinue { line: usize, column: usize }`
  - `InvalidExpression { found: TokenKind, line: usize, column: usize }`
- Implement `Display` for `ParseError` with human-readable messages, e.g.:
  `"[line 4, col 12] Expected ')' but found '{'"`
- Change `Parser::expect()` from `→ Token` with `panic!` to `→ Result<Token, ParseError>` returning `UnexpectedToken` or `UnexpectedEndOfInput`
- Propagate `ParseError` with `?` through all parser methods — change every method in `statements.rs` and `expressions.rs` from `→ StatementType/ExpressionType/Block` to `→ Result<..., ParseError>`
- Replace all remaining `panic!` calls in the parser:
  - `statements.rs:76,83` — `break`/`continue` outside loop → `Err(ParseError::InvalidBreak/InvalidContinue { line, col })`
  - `statements.rs:92` — no more tokens → `Err(ParseError::UnexpectedEndOfInput)`
  - `expressions.rs:306` — invalid expression statement → `Err(ParseError::InvalidExpression)`
  - `expressions.rs:351` — invalid simple expression token → `Err(ParseError::InvalidExpression)`
  - `expressions.rs:355` — no next token → `Err(ParseError::UnexpectedEndOfInput)`
  - `expressions.rs:360` — non-identifier in `parse_identifier_expression` → `Err(ParseError::UnexpectedToken)`
  - `expressions.rs:408,417` — non-identifier in parameter list → `Err(ParseError::UnexpectedToken)`
  - `expressions.rs:459-462` — invalid dictionary key → `Err(ParseError::InvalidExpression)`
  - `expressions.rs:118` — invalid `TokenKind` → `BinaryOperator` conversion → `Err(ParseError::InvalidExpression)`
- Change `Parser::parse()` from `→ Vec<StatementType>` to `→ Result<Vec<StatementType>, ParseError>`
- Add `From<ParseError> for RuntimeError` in `runner.rs`
- Update `runner::interpret()` to propagate the parser error with `?`
- Update parser tests to handle the now-`Result`-returning parse methods

---

## Phase 3 — AST: Embed `SourceSpan` in AST Nodes

**Goal:** Every key AST node carries the source location of the token(s) that produced it, so the interpreter can report errors with location.

- Add a `SourceSpan` struct (in a new `src/span.rs`):
  ```rust
  pub struct SourceSpan {
      pub line: usize,
      pub column: usize,
  }
  ```
- Add `span: SourceSpan` to the AST node structs that are the interpreter's primary error sites:
  - `IdentifierExpression` — for undeclared variable errors
  - `CallExpression` — for not-callable / argument errors
  - `BinaryOperationExpression` — for type mismatch errors
  - `UnaryOperationExpression` — for unsupported operator errors
  - `AccessorExpression` — for invalid accessor errors
  - `IfStatement` and `While` — for non-boolean condition errors
  - `VariableDeclarationStatement` — for duplicate identifier errors
  - `AssignmentStatement` — for not-assignable errors
- Update each corresponding parser method to capture `line`/`column` from the relevant token (typically the first token of the construct) and set `span` when constructing the AST node

---

## Phase 4 — `ExecutionError`: Add Optional `SourceSpan`

> **Prerequisite:** This phase depends on `improving-execution-error.md` being completed first. Adding `span` to a stringly-typed struct is possible but wasted effort — wait until `ExecutionError` is a proper enum before doing this.

**Goal:** Interpreter errors carry source location when available.

- Add `span: Option<SourceSpan>` to `ExecutionError`
- Add `ExecutionError::at(call_info, reason, span: SourceSpan)` constructor alongside the existing `new()`
- Update `Display` for `ExecutionError` to print `[line X, col Y]` when a span is present:
  `"[line 4, col 3] Runtime error: Variable 'foo' is not declared"`

---

## Phase 5 — Interpreter: Thread Spans into Errors

**Goal:** Every `ExecutionError` the interpreter raises includes the location from the AST node.

- At each interpreter site that produces an `ExecutionError`, pull the `span` from the AST node and pass it to `ExecutionError::at(...)`:
  - `mod.rs:144` — `if` condition error → span from `IfStatement`
  - `mod.rs:186` — `while` condition error → span from `While`
  - `mod.rs:224,227` — accessor/assignment errors → span from `AccessorExpression` / `AssignmentStatement`
  - `mod.rs:252-317` — binary expression type errors → span from `BinaryOperationExpression`
  - `mod.rs:403` — accessor on wrong type → span from `AccessorExpression`
  - `scope.rs:40-44` — undeclared variable → span from `IdentifierExpression` (needs to be threaded into `get_variable`)
  - `scope.rs:52-56` — duplicate identifier → span from `VariableDeclarationStatement`
  - `builtin/mod.rs` — function execution errors → span from `CallExpression`
- For builtin and git module functions that generate `ExecutionError` with no AST context, `span` stays `None` — the interpreter call site has the `CallExpression` span and can wrap it

---

## Phase 6 — Update Tests

- Update lexer tests: `Token` construction includes `line`/`column`
- Update parser tests: `parse()` now returns `Result`, unwrap in happy-path tests
- Update interpreter tests: error assertions can optionally check `span` fields — at minimum ensure existing tests still pass

---

## Execution Order

Phases 1 → 2 → 3 → 4 → 5 are strictly sequential — each phase depends on the previous. Phase 6 is incremental, done alongside each phase as you go rather than all at the end.

The biggest effort is Phase 3 (touching every AST node struct and every parser method that constructs them) and Phase 5 (threading spans into every interpreter error site). Phase 2 is large in line-count but mechanical — the same `→ Result<..., ParseError>` change repeated across all parser methods.
