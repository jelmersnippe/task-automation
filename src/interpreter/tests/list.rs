use std::rc::Rc;

use crate::{
    interpreter::{
        Interpreter,
        function::FunctionDeclaration,
        list::ListDeclaration,
        scope::{Callable, DataType},
    },
    lexer::lexer,
    parser::{
        Parser,
        expressions::{ExpressionType, LiteralType},
        statements::StatementType,
    },
};

#[test]
fn interprets_array_reference_overwrite() {
    let dsl = "
    var x = [1,2,3]
    fn foo() {
        return x;
    }
    var y = foo()
    y = [9, 9, 9]
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::List(ListDeclaration::new(vec![
            Rc::new(DataType::Number(1.0)),
            Rc::new(DataType::Number(2.0)),
            Rc::new(DataType::Number(3.0)),
        ])))
    );
    assert_eq!(
        interpreter.scope.get_variable(&String::from("y")),
        Rc::new(DataType::List(ListDeclaration::new(vec![
            Rc::new(DataType::Number(9.0)),
            Rc::new(DataType::Number(9.0)),
            Rc::new(DataType::Number(9.0)),
        ])))
    );
}

#[test]
fn interprets_array_reference_assignment() {
    let dsl = "
    var x = [1,2,3]
    fn foo() {
        return x;
    }
    var y = foo()
    y[0] = 5
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::List(ListDeclaration::new(vec![
            Rc::new(DataType::Number(5.0)),
            Rc::new(DataType::Number(2.0)),
            Rc::new(DataType::Number(3.0)),
        ])))
    );
    assert_eq!(
        interpreter.scope.get_variable(&String::from("y")),
        Rc::new(DataType::List(ListDeclaration::new(vec![
            Rc::new(DataType::Number(5.0)),
            Rc::new(DataType::Number(2.0)),
            Rc::new(DataType::Number(3.0)),
        ])))
    );
}

#[test]
fn interpret_accessor_function_call() {
    let dsl = "
    fn foo() {
        return 1
    }

    var x = [foo];
    var y = x[0]()
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);

    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("y")),
        Rc::new(DataType::Number(1.0))
    );
}

#[test]
fn interpret_function_call_accessor_assignment() {
    let dsl = "
    var x = [1];
    
    fn foo() {
        return x
    }

    foo()[0] = 2
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);

    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::List(ListDeclaration::new(vec![Rc::new(
            DataType::Number(2.0)
        )])))
    );
}

#[test]
fn interpret_list_assignment_nested() {
    let dsl = "
    var x = [[1]]
    x[0][0] = 2
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);

    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::List(ListDeclaration::new(vec![Rc::new(
            DataType::List(ListDeclaration::new(vec![Rc::new(DataType::Number(2.0))]))
        )])))
    );
}

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
        Rc::new(DataType::List(ListDeclaration::new(vec![Rc::new(
            DataType::Number(2.0)
        )])))
    );
}

#[test]
fn interpret_list_accessor_nested() {
    let dsl = "
    var x = [[1]]
    var y = x[0][0]
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);

    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::List(ListDeclaration::new(vec![Rc::new(
            DataType::List(ListDeclaration::new(vec![Rc::new(DataType::Number(1.0))]))
        )])))
    );
    assert_eq!(
        interpreter.scope.get_variable(&String::from("y")),
        Rc::new(DataType::Number(1.0))
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
        Rc::new(DataType::List(ListDeclaration::new(vec![Rc::new(
            DataType::Number(1.0)
        )])))
    );
    assert_eq!(
        interpreter.scope.get_variable(&String::from("y")),
        Rc::new(DataType::Number(1.0))
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
        Rc::new(DataType::List(ListDeclaration::new(vec![
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
            Rc::new(DataType::Function(Callable::User(
                FunctionDeclaration::new(
                    Some(String::from("foo")),
                    vec![],
                    vec![StatementType::Return(ExpressionType::Literal(
                        LiteralType::Number(3.0)
                    ))]
                )
            ))),
        ])))
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
        Rc::new(DataType::List(ListDeclaration::new(vec![])))
    );
}
