use std::{collections::HashMap, rc::Rc};

use crate::{
    interpreter::{FunctionDeclaration, Interpreter, Primitive},
    lexer::lexer,
    parser::{
        Parser,
        expressions::{
            BinaryOperationExpression, BinaryOperator, ExpressionType, IdentifierExpression,
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

    assert_eq!(interpreter.variables.len(), 0);
    assert_eq!(interpreter.functions.len(), 1);
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

    assert_eq!(interpreter.variables.len(), 0);
    assert_eq!(interpreter.functions.len(), 1);
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

    assert_eq!(interpreter.variables.len(), 0);
    assert_eq!(
        interpreter.functions,
        HashMap::from([(
            String::from("foo"),
            Rc::new(FunctionDeclaration {
                body: vec![StatementType::Return(ExpressionType::BinaryOperation(
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
                arguments: vec![String::from("bar"), String::from("baz")]
            })
        )])
    );
}

#[test]
fn interprets_function_declaration_with_body() {
    let dsl = "fn foo() {
        var x = 3
    }";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(interpreter.variables.len(), 0);
    assert_eq!(
        interpreter.functions,
        HashMap::from([(
            String::from("foo"),
            Rc::new(FunctionDeclaration {
                body: vec![StatementType::VariableDeclaration(
                    VariableDeclarationStatement {
                        identifier: String::from("x"),
                        value: crate::parser::expressions::ExpressionType::Literal(
                            crate::parser::expressions::LiteralType::Number(3.0)
                        ),
                    }
                )],
                arguments: vec![]
            })
        )])
    );
}

#[test]
fn interprets_function_declaration_with_arguments() {
    let dsl = "fn foo(bar, baz) {}";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(interpreter.variables.len(), 0);
    assert_eq!(
        interpreter.functions,
        HashMap::from([(
            String::from("foo"),
            Rc::new(FunctionDeclaration {
                body: vec![],
                arguments: vec![String::from("bar"), String::from("baz")]
            })
        )])
    );
}

#[test]
fn interprets_function_declaration() {
    let dsl = "fn foo() {}";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(interpreter.variables.len(), 0);
    assert_eq!(
        interpreter.functions,
        HashMap::from([(
            String::from("foo"),
            Rc::new(FunctionDeclaration {
                body: vec![],
                arguments: vec![]
            })
        )])
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
        interpreter.variables,
        HashMap::from([(String::from("x"), Rc::new(Primitive::Number(3.0)))])
    );
    assert_eq!(interpreter.functions.len(), 0);
}

#[test]
fn interprets_variable_assignment_string() {
    let dsl = "var x = \"Hello\"";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.variables,
        HashMap::from([(
            String::from("x"),
            Rc::new(Primitive::String(String::from("Hello")))
        )])
    );
    assert_eq!(interpreter.functions.len(), 0);
}

#[test]
fn interprets_variable_assignment_bool() {
    let dsl = "var x = true";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.variables,
        HashMap::from([(String::from("x"), Rc::new(Primitive::Boolean(true)))])
    );
    assert_eq!(interpreter.functions.len(), 0);
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
        interpreter.variables,
        HashMap::from([(String::from("x"), Rc::new(Primitive::Boolean(true)))])
    );
    assert_eq!(interpreter.functions.len(), 1);
}

#[test]
#[should_panic]
fn panics_on_variable_assignment_scoped_which_exists() {
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
