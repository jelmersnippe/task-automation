use std::rc::Rc;

use crate::{
    interpreter::{Interpreter, list::ListDeclaration, scope::DataType},
    lexer,
    parser::Parser,
};

#[test]
fn interprets_while_with_condition() {
    let dsl = "
    var x = 0

    while (x < 3) {
        x = x + 1
    }
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Number(3.0))
    );
}

#[test]
fn interprets_while_with_false() {
    let dsl = "
    var x = 0

    while (false) {
        x = x + 1

        if (x >= 5) {
            break
        }
    }
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Number(0.0))
    );
}

#[test]
fn interprets_while_with_continue() {
    let dsl = "
    var x = 0
    var y = 0

    while (x < 1) {
        x = x + 1

        continue

        y = y + 1
    }
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

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
fn interprets_while_with_break() {
    let dsl = "
    var x = 0
    var y = 0

    while (true) {
        x = x + 1

        break

        y = y + 1
    }
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

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
fn interprets_while_with_nested_continue() {
    let dsl = "
    var x = 0
    var y = []

    while (x < 5) {
        x = x + 1

        if (x == 1) {
            continue
        }
        
        y.push(x)
    }
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Number(5.0))
    );
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("y")),
        Rc::new(DataType::List(ListDeclaration::new(vec![
            Rc::new(DataType::Number(2.0)),
            Rc::new(DataType::Number(3.0)),
            Rc::new(DataType::Number(4.0)),
            Rc::new(DataType::Number(5.0)),
        ])))
    );
}

#[test]
fn interprets_while_with_nested_break() {
    let dsl = "
    var x = 0

    while (true) {
        x = x + 1

        if (x >= 5) {
            break
        }
    }
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Number(5.0))
    );
}
