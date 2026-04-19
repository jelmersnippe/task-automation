#[cfg(test)]
mod dictionary;
#[cfg(test)]
mod list;

use std::rc::Rc;

use crate::{
    interpreter::{Interpreter, function::FunctionDeclaration, scope::DataType},
    lexer::lexer,
    parser::{
        Parser,
        expressions::{
            BinaryOperationExpression, BinaryOperator, ExpressionType, IdentifierExpression,
            LiteralType,
        },
        statements::{StatementType, VariableDeclarationStatement},
    },
};

#[test]
fn interprets_len_builtin() {
    let dsl = "
    var x = len(\"Hello\")
    var y = len([1,2,3])
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::Number(5.0))
    );
    assert_eq!(
        interpreter.scope.get_variable(&String::from("y")),
        Rc::new(DataType::Number(3.0))
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
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::Number(3.0))
    );
    assert_eq!(
        interpreter.scope.get_variable(&String::from("y")),
        Rc::new(DataType::Number(5.0))
    );
}

#[test]
fn interpret_builtin_print() {
    let dsl = "
    print(\"foo\")
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);

    interpreter.interpret();
}

#[test]
#[should_panic]
fn panics_on_no_arguments_to_print() {
    let dsl = "
    print()
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);

    interpreter.interpret();
}

#[test]
#[should_panic]
fn panics_on_too_many_arguments_to_print() {
    let dsl = "
    print(\"foo\", 3)
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);

    interpreter.interpret();
}

#[test]
#[should_panic]
fn panics_on_overriding_builtin() {
    let dsl = "
    var print = 3
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);

    interpreter.interpret();
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
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);

    interpreter.interpret();
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
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::String(String::from("outer")))
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
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::Number(1.0))
    );
    assert_eq!(
        interpreter.scope.get_variable(&String::from("y")),
        Rc::new(DataType::Number(0.0))
    );
}

#[test]
fn interprets_function_call_with_arguments() {
    let dsl = "
    fn foo(bar) {
        var x = bar
    }
    foo(1)
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("foo")),
        Rc::new(DataType::Function(FunctionDeclaration::new(
            Some(String::from("foo")),
            vec![String::from("bar")],
            vec![StatementType::VariableDeclaration(
                VariableDeclarationStatement {
                    identifier: String::from("x"),
                    value: ExpressionType::Identifier(IdentifierExpression {
                        name: String::from("bar")
                    })
                }
            )]
        )))
    );

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::Undefined())
    );
}

#[test]
fn interprets_function_call() {
    let dsl = "
    fn foo() {
        var x = 3
    }
    foo()
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("foo")),
        Rc::new(DataType::Function(FunctionDeclaration::new(
            Some(String::from("foo")),
            vec![],
            vec![StatementType::VariableDeclaration(
                VariableDeclarationStatement {
                    identifier: String::from("x"),
                    value: ExpressionType::Literal(LiteralType::Number(3.0))
                }
            )]
        )))
    );

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::Undefined())
    );
}

#[test]
fn interprets_function_declaration_with_return() {
    let dsl = "fn foo(bar, baz) {
        return bar + baz
    }";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("foo")),
        Rc::new(DataType::Function(FunctionDeclaration::new(
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
        )))
    );
}

#[test]
fn interprets_function_declaration_with_arguments() {
    let dsl = "fn foo(bar, baz) {}";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("foo")),
        Rc::new(DataType::Function(FunctionDeclaration::new(
            Some(String::from("foo")),
            vec![String::from("bar"), String::from("baz")],
            vec![]
        )))
    );
}

#[test]
fn interprets_function_declaration_as_variable() {
    let dsl = "var foo = fn() {}";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("foo")),
        Rc::new(DataType::Function(FunctionDeclaration::new(
            None,
            vec![],
            vec![]
        )))
    );
}

#[test]
fn interprets_function_declaration() {
    let dsl = "fn foo() {}";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("foo")),
        Rc::new(DataType::Function(FunctionDeclaration::new(
            Some(String::from("foo")),
            vec![],
            vec![]
        )))
    );
}

#[test]
fn interprets_variable_assignment_function() {
    let dsl = "
    var x = 3
    x = fn() {}
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::Function(FunctionDeclaration::new(
            None,
            vec![],
            vec![]
        )))
    );
}

#[test]
fn interprets_variable_assignment() {
    let dsl = "
    var x = 3
    x = 5
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::Number(5.0))
    );
}

#[test]
fn interprets_variable_declaration_number() {
    let dsl = "var x = 3";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::Number(3.0))
    );
}

#[test]
fn interprets_variable_declaration_string() {
    let dsl = "var x = \"Hello\"";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::String(String::from("Hello")))
    );
}

#[test]
fn interprets_variable_declaration_bool() {
    let dsl = "var x = true";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
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
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
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
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
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
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();
}

#[test]
#[should_panic]
fn panics_on_function_declaration_existing() {
    let dsl = "
    fn foo() {}
    fn foo() {}
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();
}
