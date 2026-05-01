# Task Automation DSL — Todo

## Bugs

- [ ] **[Critical]** Fix unary expression panic in `interpreter/mod.rs` `interpret_unary_expression` — the computed values `DataType::Number(-x)` and `DataType::Boolean(!x)` are never returned; a `panic!` executes unconditionally after them, making unary `-` and `!` completely broken at runtime.
- [ ] **[Medium]** Fix integer validation in `interpreter/coerce.rs` `expect_int` — the current check compares two truncated values so non-integers like `1.9` silently pass as `1`. Use `number.fract() != 0.0` instead.
- [ ] **[Medium]** Fix `spawn_terminal` argument handling in `interpreter/builtin/global.rs` — it documents 1–2 arguments but panics on 1 argument due to an irrefutable `let` pattern binding the second argument.
- [ ] **[Medium]** Fix silent stdin failure in REPL loop — `runner.rs:63` discards the `Result` of `stdin().read_line(...)` with `let _ = ...`; if the read fails, `dsl` stays empty and the loop continues silently with blank input forever.
- [x] Fix grammatical error message in `interpreter/scope.rs` `update_variable` — `"Identifier '{}' has not declared"` should read `"Identifier '{}' has not been declared"`.

## Code Quality

- [ ] Replace `f32` with `f64` for number storage to reduce precision loss, or introduce separate `Int` and `Float` variants in `DataType` for more principled numeric handling.
- [x] Replace `panic!` in the interpreter core with `Result<T, E>` — `ExecutionError`, `ArgumentError`, and `RuntimeError` types are defined with `From` conversions enabling `?` propagation throughout the interpreter and runner layers.
- [ ] Convert user-defined function arity mismatch in `interpreter/function.rs:75` from `panic!` to `ExecutionError` — builtins already return `Err` for wrong argument count via `Args::exact()`, this should match.
- [ ] Propagate task execution errors in `runner.rs` — `_ = task.execute(...)` at line 94 silently discards the `Result`; task errors are never surfaced to the caller.
- [ ] Extend `Result` handling to the parser layer — `Parser::parse()` still returns `Vec<StatementType>` with all syntax errors as `panic!`. Introduce a `ParseError` type and change the return to `Result<Vec<StatementType>, ParseError>`. Parse errors should collect multiple issues before reporting and include source location.
- [x] Return an error instead of `DataType::Undefined()` from `scope.get_variable` when a variable doesn't exist — silent undefined values cause confusing panics far from the actual mistake.
- [ ] Remove dead code in `lexer/mod.rs` — the string-accumulation guard at the top of the main loop makes a second identical check further down unreachable.
- [ ] Document or rewrite `insert_new_right` in `parser/expressions.rs` — the custom iterative precedence algorithm is undocumented and hard to reason about; inline comments explaining the algorithm are the minimum, a rewrite to Pratt parsing is preferred (see Architecture section).
- [ ] Clarify `PartialEq` behavior for `Callable` variants — two callables should compare equal if they refer to the same underlying function.

## Architecture

- [x] **[High]** Move `Parameters::resolve()` out of `parser/expressions.rs` and into the interpreter — the parser currently imports and calls `interpret_expression`, which is a circular dependency and a layering violation. The parser should produce unevaluated AST nodes only; argument evaluation belongs in the interpreter when a call expression is executed.
- [x] **[High]** Fix closure semantics in `interpreter/function.rs` — functions currently receive the call-site scope instead of the definition-time scope, meaning they don't truly close over their defining environment. Capture the defining scope in `FunctionDeclaration` as an `Rc<RefCell<Scope>>` and use it as the parent when creating the execution scope.
- [ ] Add source location (`line`, `column`) to `Token` in `lexer/mod.rs` and thread it through to error messages — do this early, retrofitting it later is significantly harder.
- [ ] Rewrite binary expression parsing in `parser/expressions.rs` as a Pratt parser — the current `insert_new_right` approach is non-standard and difficult to extend. A Pratt parser uses a single recursive `parse_expression(min_precedence)` function and is the conventional approach for recursive-descent parsers.
- [ ] Separate parse errors from runtime errors — parse errors should ideally collect multiple issues before reporting; runtime errors should include source location and allow the REPL to continue.

## Missing Language Features

- [ ] Implement `else` / `else if` — the `Else` token is already lexed but `parse_if_statement` never checks for it.
- [x] Implement `while` loop.
- [ ] Implement comments (`//`) — currently tokenized as two `Divide` tokens; needs to be stripped in the lexer.
- [ ] Implement the `global` keyword — referenced in the spec but absent from the lexer and parser.
- [ ] Implement `for` / iteration over collections — without iteration, lists are limited to indexed access; a `for item in list {}` construct or a `forEach` builtin would unlock much more expressive scripts.

## Missing Builtins / Standard Library

- [ ] Allow `print` to accept multiple arguments — currently limited to one, forcing string concatenation for multi-value output.
- [ ] Add a `typeof(x)` builtin returning the type name as a string — enables defensive scripting and type checking in DSL code.

## Platform / Environment

- [ ] Abstract `spawn_terminal` in `interpreter/builtin/global.rs` behind platform-conditional logic — currently hardcoded to `wt.exe` (Windows Terminal + WSL); add macOS terminal support.
- [ ] Propagate `main.rs` startup errors rather than panicking — `env::current_dir()` (line 27) and `.into_string()` (line 30) both `.unwrap()`; replace with `?` now that `main` returns `Result`. The CLI argument check at line 42 should also return a clean error instead of calling `.expect()`.
