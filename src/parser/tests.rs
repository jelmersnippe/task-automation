use crate::{
    lexer::lexer::{Token, TokenKind},
    parser::{
        Parser,
        expressions::{ExpressionType, IdentifierExpression, LiteralType},
        statements::{
            Block, FunctionDeclarationStatement, StatementType, VariableDeclarationStatement,
        },
    },
};

#[test]
fn parses_function_declaration_without_arguments() {
    let result = Parser::new(vec![
        Token::new("fn", TokenKind::Function),
        Token::new("greet", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("{", TokenKind::LeftCurly),
        Token::new("}", TokenKind::RightCurly),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::FunctionDeclaration(
            FunctionDeclarationStatement {
                identifier: String::from("greet"),
                arguments: vec![],
                body: Block { statements: vec![] }
            }
        )]
    )
}

#[test]
fn parses_function_declaration_with_return() {
    let result = Parser::new(vec![
        Token::new("fn", TokenKind::Function),
        Token::new("greet", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("{", TokenKind::LeftCurly),
        Token::new("return", TokenKind::Return),
        Token::new("5", TokenKind::Number),
        Token::new("}", TokenKind::RightCurly),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::FunctionDeclaration(
            FunctionDeclarationStatement {
                identifier: String::from("greet"),
                arguments: vec![],
                body: Block {
                    statements: vec![StatementType::Return(ExpressionType::Literal(
                        LiteralType::Number(5 as f32)
                    ))]
                }
            }
        )]
    )
}

#[test]
fn parses_function_declaration_with_single_argument() {
    let result = Parser::new(vec![
        Token::new("fn", TokenKind::Function),
        Token::new("greet", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("arg1", TokenKind::Identifier),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("{", TokenKind::LeftCurly),
        Token::new("}", TokenKind::RightCurly),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::FunctionDeclaration(
            FunctionDeclarationStatement {
                identifier: String::from("greet"),
                arguments: vec![IdentifierExpression {
                    name: String::from("arg1")
                }],
                body: Block { statements: vec![] }
            }
        )]
    )
}

#[test]
fn parses_function_declaration_with_multiple_arguments() {
    let result = Parser::new(vec![
        Token::new("fn", TokenKind::Function),
        Token::new("greet", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("arg1", TokenKind::Identifier),
        Token::new(",", TokenKind::Comma),
        Token::new("arg2", TokenKind::Identifier),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("{", TokenKind::LeftCurly),
        Token::new("}", TokenKind::RightCurly),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::FunctionDeclaration(
            FunctionDeclarationStatement {
                identifier: String::from("greet"),
                arguments: vec![
                    IdentifierExpression {
                        name: String::from("arg1")
                    },
                    IdentifierExpression {
                        name: String::from("arg2")
                    }
                ],
                body: Block { statements: vec![] }
            }
        )]
    )
}

#[test]
fn parses_number_variable_assignment() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("5", TokenKind::Number),
    ])
    .parse();

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
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("Hello World", TokenKind::String),
    ])
    .parse();

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

#[test]
fn parses_identifier_variable_assignment() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("identifier", TokenKind::Identifier),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::Identifier(IdentifierExpression {
                    name: String::from("identifier")
                })
            }
        )]
    )
}

#[test]
fn parses_boolean_variable_assignment() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("true", TokenKind::True),
        Token::new("var", TokenKind::Variable),
        Token::new("y", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("false", TokenKind::False),
    ])
    .parse();

    assert_eq!(
        result,
        vec![
            StatementType::VariableDeclaration(VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::Literal(LiteralType::Boolean(true))
            }),
            StatementType::VariableDeclaration(VariableDeclarationStatement {
                identifier: String::from("y"),
                value: ExpressionType::Literal(LiteralType::Boolean(false))
            }),
        ]
    )
}
