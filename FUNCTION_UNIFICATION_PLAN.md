# Function Call System Unification Plan

## Current State Analysis

Looking at the codebase, there are several distinct function call systems:

1. **Global Built-ins** (`print`, `len`, etc.)
   - Signature: `fn(Option<Rc<DataType>>, Vec<Rc<DataType>>, &RuntimeContext) -> Rc<DataType>` (`BuiltinFn`)
   - Storage: `BUILTINS` array, added to scope during interpreter initialization
   - Execution: Direct function call with optional receiver

2. **Module Functions** (git module)
   - Signature: `fn(Vec<Rc<DataType>>, &RuntimeContext) -> DataType` (`ModuleFn`)
   - Storage: `Module.functions: HashMap<String, ModuleFn>`
   - Execution: Called via `Callable::Module` variant, requires adaptation

3. **Data Type Methods** (dict.has, list.push, etc.)
   - Signature: Same as global built-ins (`BuiltinFn`)
   - Storage: Hardcoded in `DataType::get_method()`
   - Execution: Retrieved via `get_method()` and called as callable

4. **User-Defined Functions**
   - Representation: `FunctionDeclaration` AST node
   - Storage: Scope as `Callable::User(function_declaration)`
   - Execution: Via `function_declaration.execute()`

## Opportunity for Further Unification

Yes, all these systems can be unified further. The `BuiltinFn` signature is already capable of handling:
- Global functions (receiver = None)
- Module functions (receiver = None, with adaptation)
- Data type methods (receiver = Some(instance), already using this)
- User functions (receiver = None, with adaptation)

## Proposed Unified Architecture

### Phase 1: Standardize on BuiltinFn as Universal Execution Signature

Keep `BuiltinFn` as the single execution pathway:
```rust
pub type UnifiedFn = fn(Option<Rc<DataType>>, Vec<Rc<DataType>>, &RuntimeContext) -> Rc<DataType>;
```

### Phase 2: Create Adaptation Layers

Each function type gets adapted to `UnifiedFn` when stored:

1. **Built-in Functions**: Already match - no adaptation needed
2. **Module Functions**: 
   ```rust
   fn adapt_module<F: ModuleFn>(f: F) -> UnifiedFn {
       move |_receiver: Option<Rc<DataType>>, args: Vec<Rc<DataType>>, context| 
           -> Rc<DataType> { 
               Rc::new(f(args, context)) 
           }
   }
   ```
3. **Data Type Methods**: Already match - no adaptation needed
4. **User Functions**:
   ```rust
   fn adapt_user(func_decl: FunctionDeclaration) -> UnifiedFn {
       move |_receiver: Option<Rc<DataType>>, args: Vec<Rc<DataType>>, context| 
           -> Rc<DataType> { 
               func_decl.execute(args, context) 
           }
   }
   ```

### Phase 3: Unify Storage Mechanism

Replace fragmented storage with a single callable representation:

```rust
pub struct StoredCallable {
    name: Option<String>,    // For debugging/error messages
    function: UnifiedFn,     // The actual executable function
    // Optional: source location, arity, etc. for better errors
}

pub struct Scope {
    variables: HashMap<String, Rc<DataType>>,
    callables: HashMap<String, StoredCallable>,  // Unified callable storage
}

pub struct Module {
    name: String,
    callables: HashMap<String, StoredCallable>,  // Same structure as scope
}
```

### Phase 4: Simplify Execution Pathway

With unified storage, execution becomes straightforward:

```rust
impl StoredCallable {
    pub fn execute(&self, parameters: &Parameters, scope: Rc<RefCell<Scope>>, context: &RuntimeContext) -> Rc<DataType> {
        // All callers pass None as receiver (first parameter)
        // Receiver is only meaningful for method calls (handled elsewhere)
        (self.function)(None, parameters.resolve(scope.clone(), context), context)
    }
}
```

### Phase 5: Handle Method Calls Separately

Method calls (`obj.method()`) remain a special case but use the same infrastructure:

1. When encountering `.method`:
   - Look up `method` in the object's type-specific method table
   - If found, bind the object as receiver and execute
   - This could reuse the same `StoredCallable` structure but with bound receiver

### Phase 6: Migration Strategy

1. **Short-term**: 
   - Keep existing `Callable` enum but add adaptation layer
   - Update module system to store adapted functions
   - Update user function storage to use adaptation

2. **Medium-term**:
   - Deprecate `ModuleFn` type alias
   - Replace `Callable` enum with direct `StoredCallable` storage
   - Remove redundant execution pathways

3. **Long-term**:
   - Consider if `FunctionDeclaration` AST nodes are still needed or if we can store user functions directly as `UnifiedFn` with closure-like behavior (though this would lose AST benefits for debugging/introspection)

## Benefits of This Approach

1. **Single Execution Pathway**: All functions invoke via `UnifiedFn`
2. **Uniform Storage**: Scopes and modules store callables identically
3. **Consistent Method Lookup**: `obj.method()` works the same whether obj is a built-in type, module, or user-defined type
4. **Reduced Complexity**: Eliminates the growing `Callable` enum
5. **Future-Proof**: Adding new function types only requires writing an adapter to `UnifiedFn`
6. **Performance**: Removes enum matching in execution hot path

## Addressing Specific Concerns

### Regarding Data Type Methods
These are already perfectly positioned for unification since they use `BuiltinFn`. They would simply be stored as `StoredCallable` in type-specific method tables rather than hardcoded in `get_method()`.

### Regarding User Functions
They would continue to be represented as `FunctionDeclaration` AST nodes (preserving parse tree benefits) but when stored in scopes/modules, they'd be adapted to `UnifiedFn` via the adapter function.

### Regarding Module-Level Functions
The adaptation layer adds negligible overhead (one extra ignored parameter) while giving you the unified interface you want.

## Implementation Recommendation

Start with Phase 1 and 2: create the adaptation layers and update the module system to use them. This gives you immediate benefits with minimal disruption. Then gradually migrate storage mechanisms.
