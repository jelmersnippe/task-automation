# Spec
- Variables are set only
- There are no types. It's up to the user to determine how they use variables
- Whitespaces don't matter. (except around keywords)
- There is no need to end lines with ;. Instead, ; can be used to end a statement without using a space
- Scopes are contained inside { }
- Comments are denoted with //
- Errors stop the process. Result-based error handling is being introduced; in the future errors will surface cleanly rather than crashing.

## Variables
Variables can be created by using the `var` keyword
```
var x = 1
print(x) // Success. Prints "1"
```

### Global
Variables can be made global by using the `global` keyword

```
if (true) {
    var global y = 1
    var x = 2

    print(y) // Success. Prints "1"
    print(x) // Success. Print "2"
}

print(y) // Success. Prints "1"
print(x) // Failure. Throws a runtime error
```

## Primitives
### Number
Both integers and decimal values are supported. Decimals are "." separated.
```
var x = 1
var y = 1.5
```

### Text
Text is written between double quotes
```
var x = "Hello"
```

### Boolean
Booleans are written as `true` and `false`.
```
var trueValue = true
var falseValue = false
```

## Data structures
### Array
`[]` with `,` separated values. Supports indexed access (`x[0]`), and methods: `push(value)`, `pop()`, `clear()`.
```
var x = [1, 2, 3]
x.push(4)
var first = x[0]
```

### Map
`{}` with text keys and any value. Supports key access (`x["key"]`), and methods: `has(key)`, `delete(key)`, `clear()`.
```
var x = { name: "Alice", age: 30, }
var name = x["name"]
var exists = x.has("name")
```

## Functions
Functions can be created with or without parameters. The body of the function is defined within a `{}` scope.
Functions can be created with or without a name. Anonymous functions can be used as parameters for other functions.
Functions can return values by using the `return` keyword.
```
fn greet(name) {
    print("Hello " + name + "!")
}

greet("User") // Prints "Hello User!"

fn runTask(task_name, task) {
    print("running task: " + task_name)

    task()
}

runTask("test", fn() {
    print("Testing stuff")
}) // Prints "running task: test" and "Testing stuff"

fn add(x, y) {
    return x + y
}

var result = add(1, 2)
```
print(result) // Prints 3

## Control flow
```
if (condition) {
    execution
}
```




