# Code Quality

Cleanup items grouped by theme. High-priority items first within each section.

---

## Dead Code & Correctness

**Dead code in `execute_statements`** *(High)*
`interpreter/mod.rs` — `executed_statements` is populated on every iteration but never returned, read, or used. It allocates and clones every executed statement for nothing. Remove the declaration and the `push` call.

**`is_valid_string` called twice in `text_to_token`** *(Low)*
`lexer/lexer.rs` — `is_valid_string` is called once to assign `is_string`, then called a second time inside the `kind` chain instead of reusing the variable. Replace the second call with `is_string`.

---

## Rust Idioms

**`&String` vs `&str`** *(High)*
When a function only needs to read a string, it should accept `&str`. `&str` is strictly more flexible — a `&String` coerces to `&str` but not vice versa. Affected:
- `lexer/mod.rs`: `lookup_keyword`, `is_valid_number`, `is_valid_string`, `is_valid_identifier`
- `interpreter/scope.rs`: `get_variable`, `update_variable`
- `interpreter/list.rs`: `has`, `get`, `delete`

Changing these also removes the need to construct `String` values just for `HashMap` lookups — `HashMap<String, _>` supports `&str` lookup via `Borrow<str>`.

**Explicit `return` at end of functions** *(Medium)*
Rust uses the last expression as the implicit return value. Explicit `return` at the end of a function is not idiomatic and is flagged by Clippy. Affects nearly every function in `lexer/mod.rs`, `parser/`, `interpreter/builtin/`, `interpreter/coerce.rs`. Early-exit `return`s mid-function are correct and should stay.

**Unit enum variants written as tuple variants** *(Medium)*
`DataType::Undefined()` and `StatementResult::Void()` are zero-field tuple variants. They should be unit variants without parentheses:
```rust
// Current
enum DataType { Undefined(), ... }
// Idiomatic
enum DataType { Undefined, ... }
```
The compiler will flag every construction and match site after the change.

**`TokenKind` should derive `Copy`** *(Medium)*
Every `TokenKind` variant is a unit variant — no heap data. Deriving `Copy` eliminates `.clone()` calls when passing `TokenKind` by value (e.g. `delimiter.clone()` in `parse_comma_separated_list`).

**Type aliases for shared reference types** *(Medium)*
Compound reference types written out in full at every signature make code hard to scan and refactor. Define aliases centrally:
```rust
pub type SharedScope = Rc<Scope>;
```
If the underlying type ever changes (e.g. `Rc` → `Arc`), one line updates instead of every signature.

**Unnecessary `.clone()` on owned data** *(Medium)*
Distinct from `Rc::clone` (which is cheap and correct). Actual smell locations:
- `interpreter/mod.rs:74-75` — `statement.identifier.clone()` and `statement.value.clone()` to create locals immediately passed by reference. Pass references directly.
- `interpreter/mod.rs:57-58` — entire statement cloned on every interpreter loop iteration. Iterate by reference instead.
- `parser/mod.rs:57` — `next_token.clone()` inside `expect()` where `next_token` is already owned. `.clone()` is redundant.
- `interpreter/scope.rs:138` — `identifier.clone()` where `identifier: &String`. Change to `&str` and call `.to_owned()` only at the insert point.

**Audit remaining `RefCell` usage** *(Medium)*
After any scope refactoring, check whether interior mutability is genuinely needed or whether `&mut T` suffices. `RefCell` defers borrow errors to runtime; `&mut T` catches them at compile time.

**Verbose `match` on `Option<char>` in lexer** *(Low)*
Lookahead blocks for `>`, `<`, `=` use nested matches that can be simplified with `matches!`:
```rust
// Verbose
match chars.peek() {
    Some(next_char) => match next_char { '=' => { ... } _ => {} },
    None => {}
}
// Idiomatic
if matches!(chars.peek(), Some('=')) { ... }
```

**`iter().nth(i)` vs direct indexing** *(Low)*
`interpreter/list.rs` — `iter().nth(i)` is O(n). Since bounds are already checked manually above it, use direct indexing: `Rc::clone(&self.values.borrow()[i])`.

**Struct field shorthand not used** *(Low)*
- `interpreter/scope.rs` — `parent: parent` → `parent`
- `parser/expressions.rs` — `parameters: parameters` → `parameters`

**Redundant `.into_iter()` on `chars()`** *(Low)*
`lexer/lexer.rs` — `str::chars()` already returns an iterator. `.into_iter()` is a no-op. Remove it: `corrected_text.chars().peekable()`.

**Helper constructor for `Rc<RefCell<T>>`** *(Low)*
Where still used, `Rc::new(RefCell::new(val))` is noisy. A small generic helper cleans it up:
```rust
fn shared<T>(val: T) -> Rc<RefCell<T>> { Rc::new(RefCell::new(val)) }
```

---

## Duplication & Patterns

**`Rc::new(DataType::Undefined)` in void builtins**
Every void builtin ends with this. A small helper reduces noise:
```rust
fn undefined() -> SharedDataType { DataType::Undefined.to_shared() }
```
Once `DataType::Undefined` is a unit variant (see above), a `OnceLock` shared constant could avoid re-allocating per call.

**Dict key resolution reimplemented inline**
`interpreter/mod.rs` — `interpret_dictionary_expression` resolves a dict key by re-implementing the same match that `expect_string` already does. Call the existing helper instead.

**Intermediate `Vec` when building a dictionary**
`interpreter/mod.rs` — a `Vec` is collected first, then iterated to insert into `HashMap`. The intermediate collection is unnecessary. `HashMap` implements `FromIterator<(K, V)>` so `.collect()` works directly:
```rust
let map: HashMap<_, _> = pairs.iter().map(|(k, v)| (resolve_key(k), interp(v))).collect();
```

**String concatenation duplicated in `interpret_binary_expression`**
`String + String`, `String + Number`, `Number + String` each have their own branch but all produce `format!("{}{}", l, r)`. Since `DataType` implements `Display`, one branch replaces three:
```rust
DataType::String(l) | (_, DataType::String(_)) => format!("{}{}", l, r)
```

**Duplicate `Display` implementations**
`DataType` in `datatype.rs` and `LiteralType` in the parser both implement `Display` with near-identical `match` structures. Adding a new literal type requires updating both. Long-term fix is to unify or derive one from the other; for now, keep the coupling in mind.
