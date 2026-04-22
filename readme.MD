Optimizations:
- Clean up lexer
    - Simplify main function loop
    - Simplify TokenKind enum and add values
- Simplify operation parsing
    - Recursive-descent parsing (every precedence calls next, then performs while, start from highest)

## TODO:
### Core:
- .function calls (clear, has, delete) on dictionary
- while loop
- Task registry -> register_task + run

### Native functionality:
- Window positioning (track pid?)
- MacOS support

### Other
- Symlink / alias for running
- Daemon
