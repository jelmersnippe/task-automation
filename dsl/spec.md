# Spec
- Variables are set only
- There are no types. It's up to the user to determine how they use variables
- Whitespaces don't matter. (except around keywords)
- There is no need to end lines with ;. Instead, ; can be used to end a statement without using a space
- Scopes are contained inside { }
- Comments are denoted with //
- Errors stop the process. In the future errors will only break out of the current scope, logging a failure.

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
TBD. `[]` with `,` separated values

### Map
TBD. `{}` with Text keys and any value

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

## Functions
Functions can be created with or without parameters. The body of the function is defined within a `{}` scope.
Functions can be created with or without a name. Anonymous functions can be used as parameters for other functions.
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
```

## Control flow
```
if (condition) {
    execution
}
```




