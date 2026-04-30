mod dictionary;
mod list;
mod r#while;

use std::rc::Rc;

use crate::{
    RuntimeContext,
    interpreter::{Interpreter, datatype::DataType, function::FunctionDeclaration},
    parser::{
        expressions::{
            BinaryOperationExpression, BinaryOperator, ExpressionType, IdentifierExpression,
        },
        statements::StatementType,
    },
    runner::interpret,
};

pub fn run(dsl: &'static str) -> Interpreter {
    let mut runtime_context = RuntimeContext::new();
    return interpret(dsl.to_string(), &mut runtime_context);
}

#[test]
#[should_panic]
fn panics_on_if_with_continue() {
    let dsl = "
    if (true) {
        continue
    }
    ";
    run(dsl);
}

#[test]
#[should_panic]
fn panics_on_if_with_break() {
    let dsl = "
    if (true) {
        break
    }
    ";
    run(dsl);
}

#[test]
fn interprets_scoped_variable_rebinding() {
    let dsl = "
    var x = 1

    fn foo() {
        x = 2
    }

    foo()
    ";
    let interpreter = run(dsl);

    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Number(2.0))
    );
}

#[test]
fn interprets_variable_rebinding() {
    let dsl = "
    var x = 3
    fn foo() {
        return x;
    }
    var y = foo()
    y = 5
    ";
    let interpreter = run(dsl);

    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Number(3.0))
    );
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("y")),
        Rc::new(DataType::Number(5.0))
    );
}

#[test]
fn interpret_builtin_print() {
    let dsl = "
    print(\"foo\")
    ";
    run(dsl);
}

#[test]
#[should_panic]
fn panics_on_no_arguments_to_print() {
    let dsl = "
    print()
    ";
    run(dsl);
}

#[test]
#[should_panic]
fn panics_on_too_many_arguments_to_print() {
    let dsl = "
    print(\"foo\", 3)
    ";
    run(dsl);
}

#[test]
#[should_panic]
fn panics_on_overriding_builtin() {
    let dsl = "
    var print = 3
    ";
    run(dsl);
}

#[test]
#[should_panic = "Continue is not supported outside of loops"]
fn panics_on_function_call_with_nested_continue() {
    let dsl = "
    fn foo() {
        if (true) {
            continue
        }
    }
    foo()
    ";
    run(dsl);
}

#[test]
#[should_panic = "Break is not supported outside of loops"]
fn panics_on_function_call_with_nested_break() {
    let dsl = "
    fn foo() {
        if (true) {
            break
        }
    }
    foo()
    ";
    run(dsl);
}

#[test]
#[should_panic = "Break is not supported outside of loops"]
fn panics_on_function_call_with_break() {
    let dsl = "
    fn foo() {
        break
    }
    foo()
    ";
    run(dsl);
}

#[test]
#[should_panic = "Continue is not supported outside of loops"]
fn panics_on_function_call_with_continue() {
    let dsl = "
    fn foo() {
        continue
    }
    foo()
    ";
    run(dsl);
}

#[test]
#[should_panic]
fn panics_on_function_call_with_invalid_arguments() {
    let dsl = "
    fn foo(bar) {
        var x = bar
    }
    foo()
    ";
    run(dsl);
}

#[test]
fn interprets_if_scoped_variables() {
    let dsl = "
    var x = \"outer\"

    if (true) {
        var x = \"inner\"

        if (true) {
            var x = \"inner-inner\"
        }
    }
    ";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::String(String::from("outer")))
    );
}

#[test]
fn interprets_function_call_inline() {
    let dsl = "
    var x = fn() {return 3}()
    ";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Number(3.0))
    );
}

#[test]
fn interprets_function_call_with_return_inside_if() {
    let dsl = "
    fn foo(bar) {
        if (bar) {
            return 1
        }

        return 0
    }

    var x = foo(true)
    var y = foo(false)
    ";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Number(1.0))
    );
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("y")),
        Rc::new(DataType::Number(0.0))
    );
}

#[test]
fn interprets_function_call_with_arguments() {
    let dsl = "
    fn foo(bar) {
        return bar
    }
    var x = foo(1)
    ";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Number(1.0))
    );
}

#[test]
fn interprets_function_call() {
    let dsl = "
    fn foo() {
        return 3
    }
    var x = foo()
    ";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Number(3.0))
    );
}

#[test]
fn interprets_function_declaration_with_return() {
    let dsl = "fn foo(bar, baz) {
        return bar + baz
    }";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("foo")),
        Rc::new(DataType::Function(
            FunctionDeclaration::new(
                Some(String::from("foo")),
                vec![String::from("bar"), String::from("baz")],
                vec![StatementType::Return(ExpressionType::BinaryOperation(
                    BinaryOperationExpression::new(
                        ExpressionType::Identifier(IdentifierExpression {
                            name: String::from("bar")
                        }),
                        BinaryOperator::Add,
                        ExpressionType::Identifier(IdentifierExpression {
                            name: String::from("baz")
                        })
                    )
                ))],
                interpreter.scope.clone()
            )
            .into_callable()
        ))
    );
}

#[test]
fn interprets_function_declaration_with_arguments() {
    let dsl = "fn foo(bar, baz) {}";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("foo")),
        Rc::new(DataType::Function(
            FunctionDeclaration::new(
                Some(String::from("foo")),
                vec![String::from("bar"), String::from("baz")],
                vec![],
                interpreter.scope.clone()
            )
            .into_callable()
        ))
    );
}

#[test]
fn interprets_function_declaration_as_variable() {
    let dsl = "var foo = fn() {}";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("foo")),
        Rc::new(DataType::Function(
            FunctionDeclaration::new(None, vec![], vec![], interpreter.scope.clone())
                .into_callable()
        ))
    );
}

#[test]
fn interprets_function_declaration() {
    let dsl = "fn foo() {}";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("foo")),
        Rc::new(DataType::Function(
            FunctionDeclaration::new(
                Some(String::from("foo")),
                vec![],
                vec![],
                interpreter.scope.clone()
            )
            .into_callable()
        ))
    );
}

#[test]
fn interprets_variable_assignment_function() {
    let dsl = "
    var x = 3
    x = fn() {}
    ";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Function(
            FunctionDeclaration::new(None, vec![], vec![], interpreter.scope.clone())
                .into_callable()
        ))
    );
}

#[test]
fn interprets_variable_assignment() {
    let dsl = "
    var x = 3
    x = 5
    ";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Number(5.0))
    );
}

#[test]
fn interprets_variable_declaration_number() {
    let dsl = "var x = 3";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Number(3.0))
    );
}

#[test]
fn interprets_variable_declaration_string() {
    let dsl = "var x = \"Hello\"";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::String(String::from("Hello")))
    );
}

#[test]
fn interprets_variable_declaration_bool() {
    let dsl = "var x = true";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Boolean(true))
    );
}

#[test]
fn interprets_variable_declaration_scoped_2() {
    let dsl = "
    fn foo() {
        var x = false
    }

    var x = true
    foo()
    ";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Boolean(true))
    );
}

#[test]
fn interprets_variable_declaration_scoped() {
    let dsl = "
    fn foo() {
        var x = false
    }

    foo()
    var x = true
    ";
    let interpreter = run(dsl);
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Boolean(true))
    );
}

#[test]
#[should_panic]
fn panics_on_variable_declaration_existing() {
    let dsl = "
    var x = true
    var x = false
    ";
    run(dsl);
}

#[test]
#[should_panic]
fn panics_on_function_declaration_existing() {
    let dsl = "
    fn foo() {}
    fn foo() {}
    ";
    run(dsl);
}
