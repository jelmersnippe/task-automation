use std::{collections::HashMap, rc::Rc};

use crate::{
    interpreter::{
        Interpreter,
        dictionary::DictionaryDeclaration,
        function::FunctionDeclaration,
        list::ListDeclaration,
        scope::{Callable, DataType},
    },
    lexer,
    parser::{
        Parser,
        expressions::{ExpressionType, LiteralType},
        statements::StatementType,
    },
};

#[should_panic]
#[test]
fn panics_on_accessing_undefined_key() {
    let dsl = "
    var x = {
    }
    x[\"a\"]
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();
}

#[test]
fn interprets_property_scopes() {
    let dsl = "
    var test = 10
    var x = {
        a: 1,
    }

    var y = {
        a: 2,
        b: {
            c: fn() {return test},
        },
    }

    var foo = \"a\"
    var bar = x.has(foo)

    var baz = y[\"b\"][\"c\"]()

    var a = y[\"b\"].has(\"c\")
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("bar")),
        Rc::new(DataType::Boolean(true)),
    );
    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("baz")),
        Rc::new(DataType::Number(10.0)),
    );
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("a")),
        Rc::new(DataType::Boolean(true)),
    );
}

#[test]
fn interprets_dictionary_builtins() {
    let dsl = "
    var x = {
        a: 1,
        b: undefined,
    }

    var a = x.has(\"a\")
    var b = x.has(\"b\")
    var c = x.has(\"x\")

    x.delete(\"a\")
    x.delete(\"x\")
    x.clear()

    var d = x.has(\"a\")
    var e = x.has(\"b\")
    var f = len(x)
    ";
    let tokens = lexer::lexer(String::from(dsl));
    let ast = Parser::new(tokens).parse();
    let mut interpreter = Interpreter::new(ast);
    interpreter.interpret();

    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("a")),
        Rc::new(DataType::Boolean(true)),
    );
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("b")),
        Rc::new(DataType::Boolean(true)),
    );
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("c")),
        Rc::new(DataType::Boolean(false)),
    );
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("d")),
        Rc::new(DataType::Boolean(false)),
    );
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("e")),
        Rc::new(DataType::Boolean(false)),
    );
    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("f")),
        Rc::new(DataType::Number(0.0)),
    );

    assert_eq!(
        interpreter.scope.borrow().get_variable(&String::from("x")),
        Rc::new(DataType::Dictionary(DictionaryDeclaration::new(
            HashMap::new()
        )))
    );
}

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
        interpreter.scope.borrow().get_variable(&String::from("x")),
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
        interpreter.scope.borrow().get_variable(&String::from("y")),
        Rc::new(DataType::Number(1.0)),
    );
    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("bar")),
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
        interpreter.scope.borrow().get_variable(&String::from("x")),
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
                    Rc::new(DataType::Function(Callable::User(
                        FunctionDeclaration::new(
                            Some(String::from("foo")),
                            vec![],
                            vec![StatementType::Return(ExpressionType::Literal(
                                LiteralType::Number(3.0)
                            ))],
                            interpreter.scope.clone()
                        )
                    )))
                ),
            ])
        )))
    );
}
