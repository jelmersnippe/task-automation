# Rust-Specific Refactoring Notes

## 1. `&String` vs `&str` (High Priority)

One of the most common beginner patterns in this codebase. When a function only needs to read a string, it should accept `&str` instead of `&String`. `&str` is strictly more flexible — a `&String` can always be coerced to `&str`, but not vice versa. As a rule: only accept `&String` if you need to call methods exclusive to `String` (which is rare).

Affected locations:
- `lexer/mod.rs`: `lookup_keyword`, `is_valid_number`, `is_valid_string`, `is_valid_identifier`
- `interpreter/scope.rs`: `get_variable`, `update_variable`
- `interpreter/list.rs`: `has`, `get`, `delete`

A related benefit: `HashMap<String, _>` supports lookup by `&str` directly (via `Borrow<str>`), so changing these signatures also removes the need to construct or clone `String` values just to perform a lookup.

---

## 2. Explicit `return` at End of Functions (Medium Priority)

Rust uses the last expression in a block as the implicit return value. An explicit `return` statement at the very end of a function is not idiomatic and is flagged by Clippy. This pattern appears in almost every function in the codebase.

```rust
// Not idiomatic
fn parse(&mut self) -> Vec<StatementType> {
    ...
    return ast;
}

// Idiomatic
fn parse(&mut self) -> Vec<StatementType> {
    ...
    ast
}
```

`return` is correct and appropriate for early exits mid-function, which is already used correctly in some places. The fix is simply to drop the `return` keyword (and the semicolon) on the final expression of each function.

Affects nearly every function in: `lexer/mod.rs`, `parser/mod.rs`, `parser/expressions.rs`, `parser/statements.rs`, `interpreter/builtin/`, `interpreter/coerce.rs`.

---

## 3. Unit Enum Variants Written as Tuple Variants (Medium Priority)

`DataType::Undefined()` and `StatementResult::Void()` are zero-field tuple variants. In Rust, a variant with no data should be written without parentheses:

```rust
// What exists
enum DataType    { Undefined(), ... }
enum StatementResult { Void(), ... }

// Idiomatic
enum DataType    { Undefined, ... }
enum StatementResult { Void, ... }
```

The parenthesised form generates a constructor function, which is unnecessary and misleading — it implies a field is being constructed. Every match arm and construction site would need to be updated (removing the `()`), but the compiler will point them all out.

---

## 4. Unnecessary `.clone()` on Owned Data (Medium Priority)

There are two categories of `.clone()` in this codebase. It is important to distinguish them:

**`Rc::clone` is cheap and expected.** Cloning an `Rc<T>` only increments a reference count — it does not copy the underlying data. This is correct and intentional throughout the interpreter. The only style note: prefer `Rc::clone(&x)` over `x.clone()` to make the cheap clone explicit and distinguishable from a deep copy.

**Cloning owned data is the actual smell.** Examples:

- `interpreter/mod.rs:74-75`: `statement.identifier.clone()` and `statement.value.clone()` to create locals that are immediately passed by reference. Pass `&statement.identifier` and `&statement.value` directly instead.
- `interpreter/mod.rs:57-58`: The entire current statement is `.clone()`d on every interpreter loop iteration. If `interpret_statement` accepted `&StatementType` (which it already does in its signature), iterating by reference would avoid this.
- `parser/mod.rs:57`: `return next_token.clone()` inside `expect()` — `next_token` is already owned at that point (returned from `self.next()`), so `.clone()` is completely redundant.
- `interpreter/scope.rs:138`: `identifier.clone()` where `identifier: &String`, cloning a full `String` to insert into a `HashMap`. Changing the parameter to `&str` and calling `.to_owned()` only at the insert point makes the allocation intentional and visible.

---

## 5. Dead Code: Unused Vec in `execute_statements` (High Priority)

`interpreter/mod.rs:469-472`:

```rust
let mut executed_statements: Vec<StatementType> = vec![];
for x in statements {
    let statement_result = interpret_statement(scope.clone(), x);
    executed_statements.push(x.clone()); // never read
    ...
}
```

`executed_statements` is populated on every iteration but is never returned, read, or used in any way. It allocates heap memory and clones every executed statement for no reason. Remove the `let mut executed_statements` declaration and the `executed_statements.push(x.clone())` line entirely.

---

## 6. `TokenKind` Should Derive `Copy` (Medium Priority)

Every variant of `TokenKind` is a unit variant — none contain heap-allocated data. This means `TokenKind` is trivially copyable and should derive `Copy`:

```rust
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TokenKind { ... }
```

This eliminates the need to call `.clone()` when passing a `TokenKind` by value (e.g., `delimiter.clone()` in `parse_comma_separated_list`), and allows comparisons and assignments to move a copy rather than a reference, which simplifies many call sites.

---

## 7. `iter().nth(i)` vs Direct Indexing (Low Priority)

`interpreter/list.rs:86-92`:

```rust
return Rc::clone(
    self.values.borrow().iter().nth(i).expect("Index out of range"),
);
```

`iter().nth(i)` traverses the iterator from the beginning, making it O(n). A `Vec` supports direct O(1) indexing via `&vec[i]`. Since the bounds check is already performed manually just above this line, the `.expect` is also redundant:

```rust
Rc::clone(&self.values.borrow()[i])
```

---

## 8. Struct Field Shorthand Not Used (Low Priority)

When a struct field name and a local variable share the same name, Rust allows shorthand initialisation. Two places miss this:

- `interpreter/scope.rs:106-109`: `parent: parent` → `parent`
- `parser/expressions.rs:404`: `parameters: parameters` → `parameters`

This is a minor style point but is what `rustfmt` and Clippy expect.

---

## 9. Redundant `.into_iter()` on `chars()` (Low Priority)

`lexer/lexer.rs:173`:

```rust
corrected_text.chars().into_iter().peekable()
```

`str::chars()` already returns an iterator (`Chars<'_>`). Calling `.into_iter()` on an iterator is a no-op — the iterator simply returns itself. Remove `.into_iter()`:

```rust
corrected_text.chars().peekable()
```

---

## 10. `is_valid_string` Called Twice in `text_to_token` (Low Priority)

`lexer/lexer.rs:147-161`: `is_valid_string(&text)` is called once to assign `is_string`, but then called a second time inside the `kind` determination chain instead of reusing the already-computed `is_string` variable:

```rust
let is_string = is_valid_string(&text);
let value = if is_string { ... } else { ... };
let kind = if is_valid_number(&text) { ... }
    else if is_valid_identifier(&text) { ... }
    else if is_valid_string(&text) { ... } // should be `is_string`
    else { ... };
```

Replace the second call with `is_string`. A small oversight but avoids a redundant traversal of the string.

---

## 11. Verbose `match` on `Option<char>` in Lexer (Low Priority)

The lookahead blocks for `>`, `<`, and `=` in `lexer/lexer.rs` use nested matches that can be simplified:

```rust
// Verbose
match chars.peek() {
    Some(next_char) => match next_char {
        '=' => { ... chars.next(); }
        _ => {}
    },
    None => {}
};

// Idiomatic
if matches!(chars.peek(), Some('=')) {
    ...
    chars.next();
}
```

The `matches!` macro checks a pattern without binding variables, which is exactly what is needed here.

---

## 12. Inconsistent `return` Inside `match` Arms (Low Priority)

`interpreter/mod.rs` in `interpret_binary_expression`: the `Number + Number` arm returns its value as an implicit last expression (correct), while the `Number + String` and `String + _` arms use `return match { ... }`. All arms should be consistent — the `return` keyword inside a `match` expression arm is unnecessary when the `match` itself is the final expression.

---

## 14. Type Aliases for Shared Reference Types (Medium Priority)

Even after reducing `Rc<RefCell<>>` usage, compound types may remain in other parts of the codebase. Writing these types out in full at every function signature makes the code harder to scan and harder to refactor.

Define aliases in a central location (e.g. a `types.rs` module, or at the top of `scope.rs`):

```rust
pub type SharedScope = Rc<Scope>; // after item 13
```

If the underlying type ever needs to change (e.g. swapping `Rc` for `Arc` for thread safety), you update one line instead of every signature that mentions it.

---

## 15. Helper Constructor for `Rc<RefCell<T>>` (Low Priority)

Where `Rc<RefCell<>>` is still used after the above refactors, construction sites are noisy:

```rust
Rc::new(RefCell::new(some_value))
```

A small generic helper removes the nesting:

```rust
fn shared<T>(val: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(val))
}
```

Usage becomes `shared(some_value)`. Minor, but consistently cleaner across construction sites.

---

## 16. Audit Remaining `RefCell` Usage (Medium Priority)

After item 13, do a pass over any remaining `RefCell` usages and ask: is interior mutability actually needed here, or does something own this data exclusively and just pass it around?

The rule of thumb:
- **`RefCell<T>`** — needed when multiple owners (`Rc`) all need to mutate the value
- **`&mut T`** or owned `T` — sufficient when only one place mutates, even if others read

Reducing `RefCell` usage where it isn't needed makes borrow errors compile-time rather than runtime panics, which is always preferable in Rust.

