# Boilerplate & Duplication

This file documents repeated patterns in the codebase that could be extracted into helpers to reduce noise and make the code easier to maintain.

---

## 1. Builtin argument validation

**Where:** Every function in `interpreter/builtin.rs`

Every builtin starts with the same shape:

```rust
let [arg] = data.as_slice() else {
    panic!("my_builtin expects 1 argument, got {}", data.len());
};
```

Two-argument variants repeat the same thing with `[a, b]`. This is 6–8 lines of near-identical code per builtin.

**Fix idea:** A helper that takes the `Vec<DataType>` and the expected count, and either returns an array or panics with a consistent message:

```rust
fn expect_args<const N: usize>(name: &str, data: Vec<DataType>) -> [DataType; N] {
    data.try_into().unwrap_or_else(|v: Vec<_>| {
        panic!("{} expects {} argument(s), got {}", name, N, v.len())
    })
}
```

Usage becomes a single line per builtin:

```rust
let [arg] = expect_args::<1>("print", data);
```

> This uses a const generic (`<const N: usize>`) — a Rust feature that lets you bake a number into a type. `[DataType; N]` is an array of exactly `N` elements known at compile time.

---

## 2. Test setup pipeline

**Where:** `interpreter/tests/mod.rs`, `interpreter/tests/list.rs`, `interpreter/tests/dictionary.rs`

The same three-step pipeline appears 30+ times:

```rust
let tokens = Lexer::new(/* dsl string */).tokenize();
let ast = Parser::new(tokens).parse();
let mut interpreter = Interpreter::new(ast);
interpreter.interpret();
```

**Fix idea:** A `run` helper in the test module:

```rust
fn run(dsl: &str) -> Interpreter {
    let tokens = Lexer::new(dsl.to_string()).tokenize();
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();
    interpreter
}
```

Every test then becomes:

```rust
let interpreter = run("let x = 1 + 2");
// assert on interpreter.scope ...
```

This also means if the pipeline ever changes (e.g. a new compilation step is added), you update one place instead of 30.

---

## 3. `Rc::new(DataType::Undefined())` at the end of void builtins

**Where:** Every void builtin in `interpreter/builtin.rs`

Every builtin that doesn't produce a meaningful value ends with:

```rust
return Rc::new(DataType::Undefined());
```

**Fix idea:** A small helper or module-level constant:

```rust
fn undefined() -> Rc<DataType> {
    Rc::new(DataType::Undefined())
}
```

Or, once `DataType::Undefined` is changed from a tuple variant to a unit variant (see `rust-refactor.md`), a `lazy_static` / `OnceLock` shared constant could avoid allocating a new `Rc` each time. For now the function is the simpler improvement.

> Note: This also ties into the `&str` / implicit return refactors — the `return` keyword would go away too.

---

## 4. Dict key resolution duplicated inline

**Where:** `interpreter/mod.rs` — `interpret_dictionary_expression`

The function resolves a dict key by matching on the expression and extracting a string. This is exactly what `expect_string` in `interpreter/helpers.rs` already does, but `interpret_dictionary_expression` re-implements the check inline instead of calling the helper.

**Fix:** Call `expect_string` (or whatever the helper is named) instead of duplicating the match logic.

---

## 5. Intermediate `Vec` when building the dictionary

**Where:** `interpreter/mod.rs` — `interpret_dictionary_expression`

The dictionary is built by first collecting into a `Vec`, then iterating that `Vec` to insert into a `HashMap`. The intermediate `Vec` is not needed.

**Fix idea:** Collect directly into the `HashMap` in one iterator chain:

```rust
let map: HashMap<String, Rc<DataType>> = pairs
    .iter()
    .map(|(key_expr, val_expr)| {
        let key = resolve_key(key_expr);
        let val = interpret_expression(val_expr, scope);
        (key, val)
    })
    .collect();
```

> `HashMap` implements `FromIterator<(K, V)>`, so `.collect()` works here directly.

---

## 6. `DataType::Function(Callable::User(...))` constructor

**Where:** `interpreter/mod.rs`, `interpreter/function.rs`

Constructing a user-defined function value requires three levels of nesting:

```rust
DataType::Function(Callable::User(UserFunction {
    parameters: ...,
    body: ...,
    scope: ...,
}))
```

This is verbose and the internals (`Callable`, `UserFunction`) are an implementation detail callers shouldn't have to know about.

**Fix idea:** A static constructor method on `DataType`:

```rust
impl DataType {
    pub fn user_function(parameters: Vec<String>, body: Vec<StatementType>, scope: Rc<Scope>) -> Self {
        DataType::Function(Callable::User(UserFunction { parameters, body, scope }))
    }
}
```

---

## 7. String concatenation duplicated in `interpret_binary_expression`

**Where:** `interpreter/mod.rs` — `interpret_binary_expression`

The expression `format!("{}{}", l, r)` (or equivalent string concatenation logic) appears three times for different type combinations: `String + String`, `String + Number`, `Number + String`.

**Fix idea:** Resolve both sides to strings first, then concatenate once:

```rust
let result = format!("{}{}", l.to_string(), r.to_string());
```

This works if `DataType` implements `Display` (which it does). One branch replaces three.

---

## 8. Near-identical `Display` implementations

**Where:** `DataType` in `interpreter/mod.rs` (or wherever it's defined), `LiteralType` in the parser

Both types implement `Display` with a `match` that maps each variant to a string. The structure is essentially identical.

This one is harder to de-duplicate mechanically, but it's worth knowing they exist as a pair — if you add a new literal type, you need to update both `Display` implementations, which is easy to forget.

A longer-term fix would be to unify `LiteralType` and `DataType` or derive one from the other, but that's an architectural change. For now, just be aware of the coupling.

---

## Summary

| # | Pattern | Occurrences | Suggested fix |
|---|---------|-------------|---------------|
| 1 | Builtin arg validation | ~10 builtins | `expect_args::<N>()` helper |
| 2 | Lex → parse → interpret pipeline in tests | 30+ | `run(dsl: &str) -> Interpreter` helper |
| 3 | `Rc::new(DataType::Undefined())` | ~10 builtins | `undefined()` helper |
| 4 | Dict key resolution reimplemented inline | 1 | Call existing `expect_string` helper |
| 5 | Intermediate `Vec` before `HashMap` | 1 | Collect directly into `HashMap` |
| 6 | Triple-nested function constructor | 2–3 | `DataType::user_function(...)` |
| 7 | String concat match arms | 3 | Single `format!` after `to_string()` |
| 8 | Duplicate `Display` impls | 2 | Awareness; unify types long-term |
