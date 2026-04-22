use std::{collections::HashMap, rc::Rc};

use crate::{
    interpreter::{
        Interpreter,
        function::FunctionDeclaration,
        list::{DictionaryDeclaration, ListDeclaration},
        scope::DataType,
    },
    lexer::lexer,
    parser::{
        Parser,
        expressions::{ExpressionType, LiteralType},
        statements::StatementType,
    },
};

#[test]
fn interprets_dictionary_assignment() {
    let dsl = "
    var x = {
        a: 1,
    }
    x[\"b\"] = 3
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::Dictionary(DictionaryDeclaration::new(
            HashMap::from([
                (String::from("a"), Rc::new(DataType::Number(1.0))),
                (String::from("b"), Rc::new(DataType::Number(3.0))),
            ])
        )))
    );
}

#[test]
fn interprets_dictionary_accessor() {
    let dsl = "
    var x = {
        a: 1,
        b: 3,
    }
    var y = x[\"a\"]

    var foo = \"b\"
    var bar = x[foo]
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("y")),
        Rc::new(DataType::Number(1.0)),
    );
    assert_eq!(
        interpreter.scope.get_variable(&String::from("bar")),
        Rc::new(DataType::Number(3.0)),
    );
}

#[test]
fn interprets_dictionary_declaration() {
    let dsl = "
    fn foo() {return 3}
    var x = {
        a: 1,
        b: \"Hello\",
        c: true,
        d: [1, 2, 3],
        e: {},
        f: foo(),
        g: foo,
    }
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.get_variable(&String::from("x")),
        Rc::new(DataType::Dictionary(DictionaryDeclaration::new(
            HashMap::from([
                (String::from("a"), Rc::new(DataType::Number(1.0))),
                (
                    String::from("b"),
                    Rc::new(DataType::String(String::from("Hello")))
                ),
                (String::from("c"), Rc::new(DataType::Boolean(true))),
                (
                    String::from("d"),
                    Rc::new(DataType::List(ListDeclaration::new(vec![
                        Rc::new(DataType::Number(1.0)),
                        Rc::new(DataType::Number(2.0)),
                        Rc::new(DataType::Number(3.0)),
                    ])))
                ),
                (
                    String::from("e"),
                    Rc::new(DataType::Dictionary(DictionaryDeclaration::new(
                        HashMap::new()
                    )))
                ),
                (String::from("f"), Rc::new(DataType::Number(3.0))),
                (
                    String::from("g"),
                    Rc::new(DataType::Function(FunctionDeclaration::new(
                        Some(String::from("foo")),
                        vec![],
                        vec![StatementType::Return(ExpressionType::Literal(
                            LiteralType::Number(3.0)
                        ))],
                    )))
                ),
            ])
        )))
    );
}
