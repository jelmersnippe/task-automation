# Test Suite Improvements

## Should You Have Separate Test Suites for Lexer, Parser, and Interpreter?

**Yes.** Each layer has a distinct contract:

- The **lexer** contract: given source text, produce a correct token stream.
- The **parser** contract: given a token stream, produce a correct AST structure.
- The **interpreter** contract: given an AST, produce correct runtime values.

If you only test at the interpreter level, a bug in the lexer or parser that happens to cancel out at runtime would be invisible. Separate tests isolate failures precisely — a failing lexer test tells you immediately the problem is in tokenisation, not execution.

The key principle is the **testing pyramid**: many small, fast, focused unit tests at the bottom (lexer, parser), fewer broader integration tests at the top (interpreter). They don't duplicate each other — they serve different purposes.

**How extensive does each layer need to be?**

- **Lexer** — exhaustive for its own surface area. Every token kind, every edge case (multi-char operators, strings with spaces, illegal tokens). The current suite is already strong here.
- **Parser** — every AST node shape, every precedence rule, every structural construct. The binary expression tests are exceptional. Several constructs are missing (see below).
- **Interpreter** — should cover **behaviour**, not structure. Assert that `var x = 2 + 3` results in `x == 5.0`. Asserting a full `FunctionDeclaration` struct in an interpreter test is re-testing the parser at the wrong layer and makes tests fragile.

---

## Lexer Tests

### What's Good
Keyword coverage, operator lookahead, illegal token cases, break character handling, strings with spaces are all well covered.

### Missing Cases

**Empty string literal** — does `""` produce an empty `String` token or behave unexpectedly?
```
var x = ""
```

**String with special characters** — the lexer has no escape sequence handling. A test documenting that `"say \"hi\""` fails would make this limitation explicit and prevent a future regression from silently "fixing" it in an incomplete way.

**Single `&` or `|`** — the lexer panics on a lone `&` or `|`. This panic is completely untested. If the message or behaviour ever changes, the test suite won't catch it.
```
x & y
```

**Number edge cases** — `0.5`, and numbers at end of file without a trailing space. The lexer appends a `\n` to handle end-of-file, but this is an implementation detail not covered by a test.

**Identifier with trailing digits** — `foo1`, `foo_1` are valid identifiers. The current tests cover `_foo`, `fo_o`, `foo_` but none with digits after letters.
```
foo1 _foo1
```

**Adjacent multi-char operators without spaces** — `x>=y`, `x!=y`. Only some combinations are tested (e.g. `==><!=!>=<=`), not all multi-char operators in all adjacency configurations.

---

## Parser Tests

### What's Good
Binary expression precedence tests are thorough and systematic. Unary operator parsing is tested. List parsing is solid.

### Missing Cases

**`if` statement** — there is no parser test for an `if` statement producing an `IfStatement` AST node. This is a significant gap for an implemented construct.
```
if (x) { var y = 1 }
```

**Return statement in isolation** — `return 5` as a standalone input has no dedicated test. It only appears as part of function body tests.

**Dictionary declaration** — there are no parser tests for dictionaries at all, despite a `parse_dictionary_expression` function with several distinct code paths (identifier keys, literal keys, computed `[expr]` keys).
```
var x = { a: 1, }
var y = { ["key"]: 2, }
```

**Chained function calls** — `foo()()` — the post-expression chaining loop handles this but it is untested.
```
var x = foo()()
```

**Mixed accessor/call/property chains** — `foo()[0].bar` — individual chains are tested but not combinations across all three chain types.

**Anonymous function with arguments** — only zero-argument anonymous functions appear in tests.
```
var x = fn(a, b) { return a + b }
```

**Nested function declarations** — a function declared inside another function's body.
```
fn outer() { fn inner() {} }
```

**Property accessor chained with call** — `foo.bar()` — not tested at the parser level.

---

## Interpreter Tests

### What's Good
Scope isolation, reference semantics for lists, early return from `if`, `len` builtin, reference vs overwrite distinction, anonymous functions, inline IIFEs.

### Missing Cases

**Binary operations — none are tested**
The interpreter has a large `interpret_binary_expression` covering arithmetic, comparison, and string concatenation, but there is not a single test asserting computed values. This is the most significant gap in the entire test suite.
```dsl
var x = 2 + 3           // expect 5
var x = 10 - 3          // expect 7
var x = 2 * 4           // expect 8
var x = 10 / 2          // expect 5
var x = 2 > 1           // expect true
var x = 1 == 1          // expect true
var x = 1 != 2          // expect true
var x = "foo" + "bar"   // expect "foobar"
var x = 1 + "px"        // expect "1px"
var x = true && false   // expect false
var x = true || false   // expect true
```

**Unary operations — not tested and currently broken**
The critical bug in `interpret_unary_expression` (see `todo.md`) means these always panic, but there are no tests to surface the failure. Once the bug is fixed, these should pass:
```dsl
var x = -5      // expect -5.0
var x = !true   // expect false
var x = !false  // expect true
```

**`if (false)` — the false branch is not tested**
There is a test for `if (true)` paths, but no direct test that `if (false)` simply skips the block:
```dsl
var x = 1
if (false) { x = 2 }
// expect x == 1
```

**Function return value as a computed result**
Several function tests assert the stored AST structure of the function rather than its return value. The actual execution result is not verified:
```dsl
fn add(a, b) { return a + b }
var x = add(2, 3)
// expect x == 5
```

**Assigning one variable to another**
A basic case that is never directly tested:
```dsl
var x = 5
var y = x
// expect y == 5
```

**String comparisons**
The interpreter handles `==`, `!=`, `>`, `<`, `>=`, `<=` for strings but none are tested:
```dsl
var x = "apple" < "banana"  // expect true
var y = "foo" == "foo"       // expect true
var z = "b" != "a"           // expect true
```

**`len` on a non-empty dictionary**
`len(x)` for a dictionary is only tested after `clear()` produces an empty dict. The non-empty case is absent:
```dsl
var x = { a: 1, b: 2, }
var y = len(x)
// expect y == 2
```

**List out-of-bounds access — should panic**
The bounds check exists in `ListDeclaration::get` but the panic is untested:
```dsl
var x = [1, 2]
var y = x[5]
```

**Type mismatch in binary expression — should panic**
No test for invalid type combinations:
```dsl
var x = true + 1
var y = "foo" - 1
```

**Dictionary duplicate key behaviour**
What happens when the same key appears twice? The current behaviour (last write wins via `HashMap::insert`) is untested and undocumented:
```dsl
var x = { a: 1, a: 2, }
// what is x["a"]?
```

### Structural Note: Interpreter Tests That Re-Test the Parser

Several tests (e.g. `interprets_function_call`, `interprets_function_declaration_with_return`, `interprets_function_declaration_with_arguments`) assert the full AST struct of a stored `FunctionDeclaration`. This is re-testing the parser at the wrong layer. These tests will break on any AST refactor even if the interpreter behaviour is unchanged.

Prefer asserting observable runtime behaviour instead — return values, variable values, and side effects.

**Example of what to avoid:**
```rust
// Asserting stored AST internals in an interpreter test — fragile
assert_eq!(
    scope.get_variable("foo"),
    Rc::new(DataType::Function(Callable::User(
        FunctionDeclaration::new(Some("foo"), vec!["bar", "baz"], vec![...ast nodes...])
    )))
);
```

**Prefer:**
```rust
// Assert the runtime outcome — what the function actually does
// var x = add(2, 3)  →  expect x == 5
assert_eq!(scope.get_variable("x"), Rc::new(DataType::Number(5.0)));
```
