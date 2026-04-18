use std::rc::Rc;

use crate::{
    interpreter::{
        Interpreter, function::FunctionDeclaration, list::ListDeclaration, scope::DataType,
    },
    lexer::lexer,
    parser::{
        Parser,
        expressions::{ExpressionType, LiteralType},
        statements::StatementType,
    },
};

#[test]
fn interpret_list_assignment() {
    let dsl = "
    var x = [1]
    x[0] = 2
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);

    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Some(Rc::new(DataType::List(ListDeclaration::new(vec![
            Rc::new(DataType::Number(2.0)),
        ]))))
    );
}

#[test]
fn interpret_list_accessor() {
    let dsl = "
    var x = [1]
    var y = x[0]
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);

    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Some(Rc::new(DataType::List(ListDeclaration::new(vec![
            Rc::new(DataType::Number(1.0)),
        ]))))
    );
    assert_eq!(
        interpreter.scope.get_variable(&String::from("y")),
        Some(Rc::new(DataType::Number(1.0)),)
    );
}

#[test]
fn interpret_list_declaration() {
    let dsl = "
    fn foo() {
        return 3
    }

    var y = 2
    var x = [1, \"Hello\", true, [1, 2, 3], y, foo(), foo]
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);

    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Some(Rc::new(DataType::List(ListDeclaration::new(vec![
            Rc::new(DataType::Number(1.0)),
            Rc::new(DataType::String(String::from("Hello"))),
            Rc::new(DataType::Boolean(true)),
            Rc::new(DataType::List(ListDeclaration::new(vec![
                Rc::new(DataType::Number(1.0)),
                Rc::new(DataType::Number(2.0)),
                Rc::new(DataType::Number(3.0)),
            ]))),
            Rc::new(DataType::Number(2.0)),
            Rc::new(DataType::Number(3.0)),
            Rc::new(DataType::Function(FunctionDeclaration::new(
                vec![],
                vec![StatementType::Return(ExpressionType::Literal(
                    LiteralType::Number(3.0)
                ))]
            ))),
        ]))))
    );
}

#[test]
fn interpret_list_declaration_empty() {
    let dsl = "
    var x = []
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);

    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Some(Rc::new(DataType::List(ListDeclaration::new(vec![]))))
    );
}
