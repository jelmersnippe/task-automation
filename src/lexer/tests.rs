use crate::lexer::lexer::{self, LiteralType, Token, TokenKind};

#[test]
fn tokenizes_built_in_print() {
    let result = lexer::lexer(String::from("print(\"Hello World\")"));

    assert_eq!(
        result,
        vec![
            Token::new("print", TokenKind::Print),
            Token::new("(", TokenKind::LeftParenthesis),
            Token::new(
                "Hello World",
                TokenKind::Literal(LiteralType::String(String::from("\"Hello World\"")))
            ),
            Token::new(")", TokenKind::RightParenthesis),
        ]
    );
}

#[test]
fn tokenizes_if_else_statement() {
    let result = lexer::lexer(String::from("if(x == 5) {} else {}"));

    assert_eq!(
        result,
        vec![
            Token::new("if", TokenKind::If),
            Token::new("(", TokenKind::LeftParenthesis),
            Token::new("x", TokenKind::Identifier),
            Token::new("==", TokenKind::Equal),
            Token::new("5", TokenKind::Literal(LiteralType::Number(5.0))),
            Token::new(")", TokenKind::RightParenthesis),
            Token::new("{", TokenKind::LeftCurly),
            Token::new("}", TokenKind::RightCurly),
            Token::new("else", TokenKind::Else),
            Token::new("{", TokenKind::LeftCurly),
            Token::new("}", TokenKind::RightCurly),
        ]
    );
}

#[test]
fn tokenizes_function_call_without_arguments() {
    let result = lexer::lexer(String::from("greet()"));

    assert_eq!(
        result,
        vec![
            Token::new("greet", TokenKind::Identifier),
            Token::new("(", TokenKind::LeftParenthesis),
            Token::new(")", TokenKind::RightParenthesis),
        ]
    );
}

#[test]
fn tokenizes_function_call_with_arguments() {
    let result = lexer::lexer(String::from("greet(arg1, arg2)"));

    assert_eq!(
        result,
        vec![
            Token::new("greet", TokenKind::Identifier),
            Token::new("(", TokenKind::LeftParenthesis),
            Token::new("arg1", TokenKind::Identifier),
            Token::new(",", TokenKind::Comma),
            Token::new("arg2", TokenKind::Identifier),
            Token::new(")", TokenKind::RightParenthesis),
        ]
    );
}

#[test]
fn tokenizes_function_declaration_without_arguments() {
    let result = lexer::lexer(String::from("fn greet() {}"));

    assert_eq!(
        result,
        vec![
            Token::new("fn", TokenKind::Function),
            Token::new("greet", TokenKind::Identifier),
            Token::new("(", TokenKind::LeftParenthesis),
            Token::new(")", TokenKind::RightParenthesis),
            Token::new("{", TokenKind::LeftCurly),
            Token::new("}", TokenKind::RightCurly)
        ]
    );
}

#[test]
fn tokenizes_function_declaration_with_return() {
    let result = lexer::lexer(String::from("fn greet() {return 5}"));

    assert_eq!(
        result,
        vec![
            Token::new("fn", TokenKind::Function),
            Token::new("greet", TokenKind::Identifier),
            Token::new("(", TokenKind::LeftParenthesis),
            Token::new(")", TokenKind::RightParenthesis),
            Token::new("{", TokenKind::LeftCurly),
            Token::new("return", TokenKind::Return),
            Token::new("5", TokenKind::Literal(LiteralType::Number(5.0))),
            Token::new("}", TokenKind::RightCurly)
        ]
    );
}

#[test]
fn tokenizes_function_declaration_with_single_argument() {
    let result = lexer::lexer(String::from("fn greet(arg1) {}"));

    assert_eq!(
        result,
        vec![
            Token::new("fn", TokenKind::Function),
            Token::new("greet", TokenKind::Identifier),
            Token::new("(", TokenKind::LeftParenthesis),
            Token::new("arg1", TokenKind::Identifier),
            Token::new(")", TokenKind::RightParenthesis),
            Token::new("{", TokenKind::LeftCurly),
            Token::new("}", TokenKind::RightCurly)
        ]
    );
}

#[test]
fn tokenizes_function_declaration_with_multiple_arguments() {
    let result = lexer::lexer(String::from("fn greet(arg1, arg2) {}"));

    assert_eq!(
        result,
        vec![
            Token::new("fn", TokenKind::Function),
            Token::new("greet", TokenKind::Identifier),
            Token::new("(", TokenKind::LeftParenthesis),
            Token::new("arg1", TokenKind::Identifier),
            Token::new(",", TokenKind::Comma),
            Token::new("arg2", TokenKind::Identifier),
            Token::new(")", TokenKind::RightParenthesis),
            Token::new("{", TokenKind::LeftCurly),
            Token::new("}", TokenKind::RightCurly)
        ]
    );
}

#[test]
fn tokenizes_arithmetic_operators() {
    let result = lexer::lexer(String::from("+-/*"));

    assert_eq!(
        result,
        vec![
            Token::new("+", TokenKind::Add),
            Token::new("-", TokenKind::Subtract),
            Token::new("/", TokenKind::Divide),
            Token::new("*", TokenKind::Multiply)
        ]
    );
}

#[test]
fn does_not_parse_inverted_not_equal() {
    let result = lexer::lexer(String::from("=!"));

    assert_eq!(
        result,
        vec![
            Token::new("=", TokenKind::Assign),
            Token::new("!", TokenKind::Bang)
        ]
    );
}

#[test]
fn tokenizes_comparison_operators() {
    let result = lexer::lexer(String::from("><==!=!"));

    assert_eq!(
        result,
        vec![
            Token::new(">", TokenKind::GreaterThan),
            Token::new("<", TokenKind::LessThan),
            Token::new("==", TokenKind::Equal),
            Token::new("!=", TokenKind::NotEqual),
            Token::new("!", TokenKind::Bang)
        ]
    );
}

#[test]
fn tokenizes_equal_and_assign() {
    let result = lexer::lexer(String::from("= == ="));

    assert_eq!(
        result,
        vec![
            Token::new("=", TokenKind::Assign),
            Token::new("==", TokenKind::Equal),
            Token::new("=", TokenKind::Assign),
        ]
    );
}

#[test]
fn tokenizes_number_variable_assignment() {
    let result = lexer::lexer(String::from("var x = 5"));

    assert_eq!(
        result,
        vec![
            Token::new("var", TokenKind::Variable),
            Token::new("x", TokenKind::Identifier),
            Token::new("=", TokenKind::Assign),
            Token::new("5", TokenKind::Literal(LiteralType::Number(5.0))),
        ]
    );
}

#[test]
fn tokenizes_boolean_variable_assignment() {
    let result = lexer::lexer(String::from("var x = true; var y = false"));

    assert_eq!(
        result,
        vec![
            Token::new("var", TokenKind::Variable),
            Token::new("x", TokenKind::Identifier),
            Token::new("=", TokenKind::Assign),
            Token::new("true", TokenKind::Literal(LiteralType::Boolean(true))),
            Token::new("var", TokenKind::Variable),
            Token::new("y", TokenKind::Identifier),
            Token::new("=", TokenKind::Assign),
            Token::new("false", TokenKind::Literal(LiteralType::Boolean(false))),
        ]
    );
}

#[test]
fn tokenizes_string_variable_assignment() {
    let result = lexer::lexer(String::from("var x = \"Hello World\""));

    assert_eq!(
        result,
        vec![
            Token::new("var", TokenKind::Variable),
            Token::new("x", TokenKind::Identifier),
            Token::new("=", TokenKind::Assign),
            Token::new(
                "Hello World",
                TokenKind::Literal(LiteralType::String(String::from("\"Hello World\"")))
            ),
        ]
    );
}

#[test]
fn tokenizes_numbers() {
    let result = lexer::lexer(String::from("1 1.1 .1"));

    assert_eq!(
        result,
        vec![
            Token::new("1", TokenKind::Literal(LiteralType::Number(1.0))),
            Token::new("1.1", TokenKind::Literal(LiteralType::Number(1.1))),
            Token::new(".1", TokenKind::Literal(LiteralType::Number(0.1)))
        ]
    );
}

#[test]
fn does_not_parse_number_with_multiple_decimal_separators() {
    let result = lexer::lexer(String::from("1..1"));

    assert_eq!(result, vec![Token::new("1..1", TokenKind::Illegal)]);
}

#[test]
fn does_not_parse_identifier_starting_with_number() {
    let result = lexer::lexer(String::from("1foo"));

    assert_eq!(result, vec![Token::new("1foo", TokenKind::Illegal)]);
}

#[test]
fn tokenizes_identifiers() {
    let result = lexer::lexer(String::from("foo _foo fo_o foo_"));

    assert_eq!(
        result,
        vec![
            Token::new("foo", TokenKind::Identifier),
            Token::new("_foo", TokenKind::Identifier),
            Token::new("fo_o", TokenKind::Identifier),
            Token::new("foo_", TokenKind::Identifier)
        ]
    );
}

#[test]
fn tokenizes_keywords() {
    let result = lexer::lexer(String::from("var fn return true false if else"));

    assert_eq!(
        result,
        vec![
            Token::new("var", TokenKind::Variable),
            Token::new("fn", TokenKind::Function),
            Token::new("return", TokenKind::Return),
            Token::new("true", TokenKind::Literal(LiteralType::Boolean(true))),
            Token::new("false", TokenKind::Literal(LiteralType::Boolean(false))),
            Token::new("if", TokenKind::If),
            Token::new("else", TokenKind::Else),
        ]
    );
}

#[test]
fn tokenizes_separators_and_punctuators() {
    let result = lexer::lexer(String::from("()[]{},"));

    assert_eq!(
        result,
        vec![
            Token::new("(", TokenKind::LeftParenthesis),
            Token::new(")", TokenKind::RightParenthesis),
            Token::new("[", TokenKind::LeftBracket),
            Token::new("]", TokenKind::RightBracket),
            Token::new("{", TokenKind::LeftCurly),
            Token::new("}", TokenKind::RightCurly),
            Token::new(",", TokenKind::Comma)
        ]
    );
}

#[test]
fn ignores_break_chars() {
    let result = lexer::lexer(String::from(" ;\n\t\r"));

    assert_eq!(result, vec![]);
}
