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
        Some(Rc::new(DataType::Function(FunctionDeclaration::new(
            vec![String::from("bar")],
            vec![StatementType::VariableDeclaration(
                VariableDeclarationStatement {
                    identifier: String::from("x"),
                    value: ExpressionType::Identifier(IdentifierExpression {
                        name: String::from("bar")
                    })
                }
            )]
        ))))
    );

    assert_eq!(interpreter.scope.get_variable(&String::from("x")), None);
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
        Some(Rc::new(DataType::Function(FunctionDeclaration::new(
            vec![],
            vec![StatementType::VariableDeclaration(
                VariableDeclarationStatement {
                    identifier: String::from("x"),
                    value: ExpressionType::Literal(LiteralType::Number(3.0))
                }
            )]
        ))))
    );

    assert_eq!(interpreter.scope.get_variable(&String::from("x")), None);
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
        Some(Rc::new(DataType::Function(FunctionDeclaration::new(
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
        ))))
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
        Some(Rc::new(DataType::Function(FunctionDeclaration::new(
            vec![String::from("bar"), String::from("baz")],
            vec![]
        ))))
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
        Some(Rc::new(DataType::Function(FunctionDeclaration::new(
            vec![],
            vec![]
        ))))
    );
}

#[test]
fn interprets_variable_assignment_number() {
    let dsl = "var x = 3";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Some(Rc::new(DataType::Number(3.0)))
    );
}

#[test]
fn interprets_variable_assignment_string() {
    let dsl = "var x = \"Hello\"";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Some(Rc::new(DataType::String(String::from("Hello"))))
    );
}

#[test]
fn interprets_variable_assignment_bool() {
    let dsl = "var x = true";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Some(Rc::new(DataType::Boolean(true)))
    );
}

#[test]
fn interprets_variable_assignment_scoped_2() {
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
        Some(Rc::new(DataType::Boolean(true)))
    );
}

#[test]
fn interprets_variable_assignment_scoped() {
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
        Some(Rc::new(DataType::Boolean(true)))
    );
}

#[test]
#[should_panic]
fn panics_on_variable_assignment_existing() {
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
