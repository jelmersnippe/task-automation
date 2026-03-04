use crate::{
    lexer::lexer::{Token, TokenKind},
    parser::parser::{
        self, ExpressionType, LiteralType, StatementType, VariableDeclarationStatement,
    },
};

#[test]
fn parses_number_variable_assignment() {
    let result = parser::parse(&vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("5", TokenKind::Number),
    ]);

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::Literal(LiteralType::Number(5 as f32))
            }
        )]
    )
}

#[test]
fn parses_string_variable_assignment() {
    let result = parser::parse(&vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("Hello World", TokenKind::String),
    ]);

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::Literal(LiteralType::String(String::from("Hello World")))
            }
        )]
    )
}
