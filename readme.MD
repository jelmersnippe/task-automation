Optimizations:
- Clean up lexer
    - Simplify main function loop
    - Simplify TokenKind enum and add values
- Simplify operation parsing
    - Recursive-descent parsing (every precedence calls next, then performs while, start from highest)
    - Favor left instead of right

