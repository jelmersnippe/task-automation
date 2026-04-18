var x = 1
var y=2;
var z = 3 ; var a = 4

var simple_list = [1, "Hello", true, [1, 2, 3], y]

print(x)
print(y)
print(z)
print(a)
print(simple_list)

if(true){
    var local = "local value"

    print(local)
}

var intVal = 10
var floatVal = 10.5
var textVal = "hello world"
var boolTrue = true
var boolFalse = false

print(intVal)
print(floatVal)
print(textVal)
print(boolTrue)
print(boolFalse)

var math1 = 1 + 2 * 3
var math2 = (1 + 2) * 3
var math3 = 10 / 2 + 3
var math4 = 10 / (2 + 3)

print(math1)
print(math2)
print(math3)
print(math4)

var greeting = "Hello " + "World"
print(greeting)

fn add(x, y) {
    return x + y
}

var result = add(5, 7)
print(result)

fn sayHi() {
    print("hi")
}

sayHi()

fn runTask(name, task) {
    print("Running: " + name)
    task()
}

runTask("example", fn() {
    print("inside anonymous function")
})

fn outer(x) {
    fn inner(y) {
        return y * 2
    }

    return inner(x) + 1
}

print(outer(5))

var complex_list = [outer(5), outer]
print(complex_list)

if (true) {
    print("if true branch")
}

if (false) {
    print("should not run")
}

var scopeTest = "outer"

if(true){
    var scopeTest = "inner"
    print(scopeTest)

    if(true){
        var scopeTest = "inner-inner"
        print(scopeTest)
    }

    print(scopeTest)
}

print(scopeTest)

var s1=1;var s2=2;var s3=3
print(s1);print(s2);print(s3)

var    weird   =    5
print(   weird   )

fn   spaced ( a , b )   {
    return   a+b
}

print(spaced(1,2))

fn earlyReturn(x) {
    if (x) {
        return "early"
    }

    return "late"
}

print(earlyReturn(true))
print(earlyReturn(false))

