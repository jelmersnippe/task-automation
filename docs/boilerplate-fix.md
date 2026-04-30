# Boilerplate & Duplication

This file documents repeated patterns in the codebase that could be extracted into helpers to reduce noise and make the code easier to maintain.

---

## 1. ~~Builtin argument validation~~ (Resolved)

**Where:** `interpreter/builtin/mod.rs`

Argument validation is now handled by the `Args` struct with `exact()`, `range()`, and `any()` methods, all returning `Result<_, ArgumentError>`. This replaces the old panic-based `let [arg] = data.as_slice() else { panic!(...) }` pattern. The `Args` approach is more ergonomic and integrates with the `?` propagation chain.

Note: `Args::any()` still has a bug where it hardcodes `expected_type: DataKind::Callable` in its error — there is a `// TODO` comment at `interpreter/coerce.rs:305`.

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

## 6. ~~Unified Callable Construction~~ (Resolved)

The function unification plan has been fully executed. All callable types (user functions, builtins, module functions) now share a unified internal representation and construction interface. The old nested constructor approach (`DataType::Function(Callable::User(UserFunction { ... }))`) no longer applies.

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

**Where:** `DataType` in `interpreter/datatype.rs`, `LiteralType` in the parser

Both types implement `Display` with a `match` that maps each variant to a string. The structure is essentially identical. If you add a new literal type, you need to update both `Display` implementations, which is easy to forget.

A longer-term fix would be to unify `LiteralType` and `DataType` or derive one from the other, but that's an architectural change. For now, just be aware of the coupling.

---

## Summary

| # | Pattern | Occurrences | Suggested fix |
|---|---------|-------------|---------------|
| 1 | Builtin arg validation | Resolved — `Args` struct with `?` | — |
| 3 | `Rc::new(DataType::Undefined())` | ~10 builtins | `undefined()` helper |
| 4 | Dict key resolution reimplemented inline | 1 | Call existing `expect_string` helper |
| 5 | Intermediate `Vec` before `HashMap` | 1 | Collect directly into `HashMap` |
| 6 | Unified callable construction | Resolved — function unification done | — |
| 7 | String concat match arms | 3 | Single `format!` after `to_string()` |
| 8 | Duplicate `Display` impls | 2 | Awareness; unify types long-term |
