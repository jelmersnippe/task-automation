use crate::automation_engine::lexer::{self, Token, TokenKind};

#[test]
fn parses_if_else_statement() {
    let result = lexer::lexer(String::from("if(x == 3) {} else {}"));

    assert_eq!(
        result,
        vec![
            Token::new("if", TokenKind::If),
            Token::new("(", TokenKind::LeftParenthesis),
            Token::new("x", TokenKind::Identifier),
            Token::new("==", TokenKind::Equal),
            Token::new("3", TokenKind::Number),
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
fn parses_function_call_without_arguments() {
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
fn parses_function_call_with_arguments() {
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
fn parses_function_definition_without_arguments() {
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
fn parses_function_definition_with_return() {
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
            Token::new("5", TokenKind::Number),
            Token::new("}", TokenKind::RightCurly)
        ]
    );
}

#[test]
fn parses_function_definition_with_single_argument() {
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
fn parses_function_definition_with_multiple_arguments() {
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
fn parses_arithmetic_operators() {
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
fn parses_comparison_operators() {
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
fn parses_equal_and_assign() {
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
fn parses_number_variable_assignment() {
    let result = lexer::lexer(String::from("var x = 5"));

    assert_eq!(
        result,
        vec![
            Token::new("var", TokenKind::Variable),
            Token::new("x", TokenKind::Identifier),
            Token::new("=", TokenKind::Assign),
            Token::new("5", TokenKind::Number)
        ]
    );
}

#[test]
fn parses_boolean_variable_assignment() {
    let result = lexer::lexer(String::from("var x = true; var y = false"));

    assert_eq!(
        result,
        vec![
            Token::new("var", TokenKind::Variable),
            Token::new("x", TokenKind::Identifier),
            Token::new("=", TokenKind::Assign),
            Token::new("true", TokenKind::True),
            Token::new("var", TokenKind::Variable),
            Token::new("y", TokenKind::Identifier),
            Token::new("=", TokenKind::Assign),
            Token::new("false", TokenKind::False)
        ]
    );
}

#[test]
fn parses_string_variable_assignment() {
    let result = lexer::lexer(String::from("var x = \"Hello World\""));

    assert_eq!(
        result,
        vec![
            Token::new("var", TokenKind::Variable),
            Token::new("x", TokenKind::Identifier),
            Token::new("=", TokenKind::Assign),
            Token::new("Hello World", TokenKind::String)
        ]
    );
}

#[test]
fn parses_numbers() {
    let result = lexer::lexer(String::from("1 1.1 .1"));

    assert_eq!(
        result,
        vec![
            Token::new("1", TokenKind::Number),
            Token::new("1.1", TokenKind::Number),
            Token::new(".1", TokenKind::Number)
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
fn parses_identifiers() {
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
fn parses_keywords() {
    let result = lexer::lexer(String::from("var fn return true false if else"));

    assert_eq!(
        result,
        vec![
            Token::new("var", TokenKind::Variable),
            Token::new("fn", TokenKind::Function),
            Token::new("return", TokenKind::Return),
            Token::new("true", TokenKind::True),
            Token::new("false", TokenKind::False),
            Token::new("if", TokenKind::If),
            Token::new("else", TokenKind::Else),
        ]
    );
}

#[test]
fn parses_separators_and_punctuators() {
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
