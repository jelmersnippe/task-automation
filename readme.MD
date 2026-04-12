Optimizations:
- Clean up lexer
    - Simplify main function loop
    - Simplify TokenKind enum and add values
- Simplify operation parsing
    - Recursive-descent parsing (every precedence calls next, then performs while, start from highest)

TODO:
- Scope stack. To make variable/function declaration clashing less problematic, instead of assigning everything to a flat HashMap
    in the interpreter, things should be wrapped in a scope. This should probably be some sort of stack so a scope can 
    easily be popped off when it's done (For example when a function finishes). If parallel scopes are allowed at the root level
    this could also be used to implement multiple files. A core file which holds task registry/dependency logic for example should
    not hog those variable names for other files. Cross-file identifier hogging would be horrible to work with or identify bugs
    around. This is already annoying with function scopes, let alone file scopes.
    The scope does need to know it's parent, so if an identifier is not found it can look up to the next scope recursively untill
    it's found or there are no more scopes to look in. Maybe a stack is not best here, but something like linked list?
