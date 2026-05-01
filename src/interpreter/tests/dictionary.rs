use std::{collections::HashMap, rc::Rc};

use crate::{
    interpreter::{
        datatype::DataType, dictionary::DictionaryDeclaration, function::FunctionDeclaration,
        list::ListDeclaration, tests::run,
    },
    parser::{
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
    run(dsl);
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
    let interpreter = run(dsl);

    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("bar"))
            .unwrap(),
        Rc::new(DataType::Boolean(true)),
    );
    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("baz"))
            .unwrap(),
        Rc::new(DataType::Number(10.0)),
    );
    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("a"))
            .unwrap(),
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
    var f = x.len()
    ";
    let interpreter = run(dsl);

    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("a"))
            .unwrap(),
        Rc::new(DataType::Boolean(true)),
    );
    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("b"))
            .unwrap(),
        Rc::new(DataType::Boolean(true)),
    );
    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("c"))
            .unwrap(),
        Rc::new(DataType::Boolean(false)),
    );
    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("d"))
            .unwrap(),
        Rc::new(DataType::Boolean(false)),
    );
    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("e"))
            .unwrap(),
        Rc::new(DataType::Boolean(false)),
    );
    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("f"))
            .unwrap(),
        Rc::new(DataType::Number(0.0)),
    );

    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("x"))
            .unwrap(),
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
    let interpreter = run(dsl);

    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("x"))
            .unwrap(),
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
    let interpreter = run(dsl);

    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("y"))
            .unwrap(),
        Rc::new(DataType::Number(1.0)),
    );
    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("bar"))
            .unwrap(),
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
    let interpreter = run(dsl);

    assert_eq!(
        interpreter
            .scope
            .borrow()
            .get_variable(&String::from("x"))
            .unwrap(),
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
                    Rc::new(DataType::Function(
                        FunctionDeclaration::new(
                            Some(String::from("foo")),
                            vec![],
                            vec![StatementType::Return(ExpressionType::Literal(
                                LiteralType::Number(3.0)
                            ))],
                            interpreter.scope.clone()
                        )
                        .into_callable()
                    ))
                ),
            ])
        )))
    );
}
