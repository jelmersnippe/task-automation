use crate::automation_engine::lexer::{self, Token, TokenKind};

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
    let result = lexer::lexer(String::from("var"));

    assert_eq!(result, vec![Token::new("var", TokenKind::Variable)]);
}

#[test]
fn parses_separators_and_punctuators() {
    let result = lexer::lexer(String::from("()[]{}"));

    let expected_token_count = 6;
    assert_eq!(
        result.len(),
        expected_token_count,
        "Should produce {} tokens",
        expected_token_count
    );

    assert_eq!(
        result,
        vec![
            Token::new("(", TokenKind::LeftParenthesis),
            Token::new(")", TokenKind::RightParenthesis),
            Token::new("[", TokenKind::LeftBracket),
            Token::new("]", TokenKind::RightBracket),
            Token::new("{", TokenKind::LeftCurly),
            Token::new("}", TokenKind::RightCurly)
        ]
    );
}

#[test]
fn ignores_break_chars() {
    let result = lexer::lexer(String::from(" ;\n\t\r"));

    assert_eq!(result, vec![]);
}
