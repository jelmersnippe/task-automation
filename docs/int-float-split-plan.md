# Int/Float Split Plan

## Agreed Semantics

- `Int` backed by `i64`, `Float` backed by `f64`
- Mixed arithmetic (Int op Float) promotes to Float
- Division always returns Float (`5 / 2 = 2.5`)
- Explicit cast builtins: `to_int()` and `to_float()`

---

## Phase 1 — `DataType` and `DataKind`

**Goal:** Replace `DataType::Number(f32)` with two explicit variants.

- Replace `DataType::Number(f32)` with `DataType::Int(i64)` and `DataType::Float(f64)` in `datatype.rs`
- Update `PartialEq` for `DataType` — add arms for `(Int, Int)`, `(Float, Float)`, and cross-type `(Int, Float)` / `(Float, Int)` for numeric equality comparisons
- Update `Display` for `DataType`:
  - `Int(x)` → `"{}"` (no decimal point)
  - `Float(x)` → `"{}"` (Rust's default `f64` display already shows decimals)
- Fix `DataKind::from(&DataType)` in `coerce.rs` — currently always returns `DataKind::Int` for any number. Update to return `DataKind::Int` for `DataType::Int` and `DataKind::Float` for `DataType::Float`
- Update `len()` in `global.rs` which returns `DataType::Number(string.len() as f32)` → `DataType::Int(string.len() as i64)`. Audit all other `DataType::Number` construction sites and replace with the appropriate variant

---

## Phase 2 — Lexer: Distinguish Integer and Float Literals

**Goal:** The parser has enough information to distinguish integer from float literals.

- Keep `TokenKind::Number` unchanged — no new token kinds needed
- The distinction is made in the parser by inspecting `token.value.contains('.')` at parse time
- No lexer changes required

---

## Phase 3 — AST: `LiteralType`

**Goal:** The AST explicitly represents the distinction between integer and float literals.

- Replace `LiteralType::Number(f32)` in `expressions.rs` with:
  - `LiteralType::Int(i64)`
  - `LiteralType::Float(f64)`
- Update `Display` for `LiteralType` accordingly
- Update `parse_simple_expression` in `expressions.rs` — currently `token.value.parse::<f32>().unwrap()`. Change to:
  - If `token.value.contains('.')` → parse as `f64`, emit `LiteralType::Float`
  - Otherwise → parse as `i64`, emit `LiteralType::Int`
- Update the interpreter's `interpret_expression` literal arm in `mod.rs` to map `LiteralType::Int` → `DataType::Int` and `LiteralType::Float` → `DataType::Float`

---

## Phase 4 — Interpreter: Binary Expression Arithmetic

**Goal:** `interpret_binary_expression` handles all four numeric combinations with correct promotion and division rules.

Replace the single `DataType::Number` arm with explicit handling for all combinations:

- **Int op Int** — returns `Int` for `+`, `-`, `*`; returns `Float` for `/` (always)
- **Float op Float** — returns `Float` for all arithmetic
- **Int op Float** — promote Int to `f64`, return `Float`
- **Float op Int** — promote Int to `f64`, return `Float`
- **Comparison operators** (`==`, `!=`, `<`, `>`, `<=`, `>=`) — all numeric combinations return `Boolean`. For cross-type comparisons, promote Int to `f64` before comparing
- **String concatenation** — both `Int` and `Float` stringify and concatenate, same as current `Number` behaviour. `Display` on `DataType` drives the string representation so this is automatic after Phase 1

---

## Phase 5 — `coerce.rs`: Update Type Coercion Helpers

- Replace `expect_int` — remove the whole-number validation logic, replace with a simple `DataType::Int(x) => Ok(*x)` pattern match, `_ => Err(DataKind::from(data))`
- Add `expect_float` mirroring `expect_int` for `DataType::Float`
- `Args::int()` already delegates to `expect_int` — will work correctly after the above change
- Add `Args::float()` mirroring `Args::int()` using `expect_float`
- Update `expect_string` — currently converts `DataType::Number(x)` to string. Replace with separate arms for `DataType::Int(x)` and `DataType::Float(x)`

---

## Phase 6 — Cast Builtins: `to_int()` and `to_float()`

Add two new builtins in `global.rs` and register them in `BUILTINS`:

**`to_int(value)`**:
- `Float(x)` → `Int(x as i64)` — truncates toward zero
- `Int(x)` → `Int(x)` — identity
- `String(x)` → parse as `i64`, return `ExecutionError` if parsing fails
- Other types → `ExecutionError`

**`to_float(value)`**:
- `Int(x)` → `Float(x as f64)`
- `Float(x)` → `Float(x)` — identity
- `String(x)` → parse as `f64`, return `ExecutionError` if parsing fails
- Other types → `ExecutionError`

---

## Phase 7 — Update Tests

- Update all test cases that construct `DataType::Number(...)` or `LiteralType::Number(...)` to use `Int` or `Float` as appropriate
- Add tests for:
  - Mixed arithmetic promotion (`1 + 1.5 == 2.5`)
  - Integer division producing float (`5 / 2 == 2.5`)
  - Integer division exact case (`4 / 2 == 2.0`)
  - `to_int(1.9) == 1`, `to_int("42") == 42`
  - `to_float(5) == 5.0`, `to_float("3.14") == 3.14`
  - `to_int` and `to_float` error on invalid string input
  - Type errors on invalid arithmetic (e.g. `Bool + Int`)

---

## Execution Order

Phases 1 → 3 → 4 → 5 → 6 → 7 are sequential. Phase 2 requires no changes. Phase 3 depends on Phase 1 since AST literals map to `DataType` variants. Phase 4 depends on both 1 and 3. Phases 5 and 6 depend on Phase 1. Phase 7 is last.
