use std::collections::HashMap;

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
            .lock()
            .unwrap()
            .get_variable(&String::from("bar"))
            .unwrap(),
        (DataType::Boolean(true)).to_shared(),
    );
    assert_eq!(
        interpreter
            .scope
            .lock()
            .unwrap()
            .get_variable(&String::from("baz"))
            .unwrap(),
        (DataType::Number(10.0)).to_shared(),
    );
    assert_eq!(
        interpreter
            .scope
            .lock()
            .unwrap()
            .get_variable(&String::from("a"))
            .unwrap(),
        (DataType::Boolean(true)).to_shared(),
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
            .lock()
            .unwrap()
            .get_variable(&String::from("a"))
            .unwrap(),
        (DataType::Boolean(true)).to_shared(),
    );
    assert_eq!(
        interpreter
            .scope
            .lock()
            .unwrap()
            .get_variable(&String::from("b"))
            .unwrap(),
        (DataType::Boolean(true)).to_shared(),
    );
    assert_eq!(
        interpreter
            .scope
            .lock()
            .unwrap()
            .get_variable(&String::from("c"))
            .unwrap(),
        (DataType::Boolean(false)).to_shared(),
    );
    assert_eq!(
        interpreter
            .scope
            .lock()
            .unwrap()
            .get_variable(&String::from("d"))
            .unwrap(),
        (DataType::Boolean(false)).to_shared(),
    );
    assert_eq!(
        interpreter
            .scope
            .lock()
            .unwrap()
            .get_variable(&String::from("e"))
            .unwrap(),
        (DataType::Boolean(false)).to_shared(),
    );
    assert_eq!(
        interpreter
            .scope
            .lock()
            .unwrap()
            .get_variable(&String::from("f"))
            .unwrap(),
        (DataType::Number(0.0)).to_shared(),
    );

    assert_eq!(
        interpreter
            .scope
            .lock()
            .unwrap()
            .get_variable(&String::from("x"))
            .unwrap(),
        (DataType::Dictionary(DictionaryDeclaration::new(HashMap::new()))).to_shared()
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
            .lock()
            .unwrap()
            .get_variable(&String::from("x"))
            .unwrap(),
        (DataType::Dictionary(DictionaryDeclaration::new(HashMap::from([
            (String::from("a"), (DataType::Number(1.0)).to_shared()),
            (String::from("b"), (DataType::Number(3.0)).to_shared()),
        ]))))
        .to_shared()
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
            .lock()
            .unwrap()
            .get_variable(&String::from("y"))
            .unwrap(),
        (DataType::Number(1.0)).to_shared(),
    );
    assert_eq!(
        interpreter
            .scope
            .lock()
            .unwrap()
            .get_variable(&String::from("bar"))
            .unwrap(),
        (DataType::Number(3.0)).to_shared(),
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
            .lock()
            .unwrap()
            .get_variable(&String::from("x"))
            .unwrap(),
        (DataType::Dictionary(DictionaryDeclaration::new(HashMap::from([
            (String::from("a"), (DataType::Number(1.0)).to_shared()),
            (
                String::from("b"),
                (DataType::String(String::from("Hello"))).to_shared()
            ),
            (String::from("c"), (DataType::Boolean(true)).to_shared()),
            (
                String::from("d"),
                (DataType::List(ListDeclaration::new(vec![
                    (DataType::Number(1.0)).to_shared(),
                    (DataType::Number(2.0)).to_shared(),
                    (DataType::Number(3.0)).to_shared(),
                ])))
                .to_shared()
            ),
            (
                String::from("e"),
                (DataType::Dictionary(DictionaryDeclaration::new(HashMap::new()))).to_shared()
            ),
            (String::from("f"), (DataType::Number(3.0)).to_shared()),
            (
                String::from("g"),
                (DataType::Function(
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
                .to_shared()
            ),
        ]))))
        .to_shared()
    );
}
