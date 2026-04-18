Optimizations:
- Clean up lexer
    - Simplify main function loop
    - Simplify TokenKind enum and add values
- Simplify operation parsing
    - Recursive-descent parsing (every precedence calls next, then performs while, start from highest)

## TODO:
### Core:
- Arrays + objects
- Support identifiers in scope (DataType::Reference). Pointing to an identifier should store a ref instead of a value
- Support void/null type

### Native functionality:
- Window positioning (track pid?)
- MacOS support
- Run as daemon in the background
