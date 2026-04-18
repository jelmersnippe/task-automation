use crate::{
    lexer::lexer::{Token, TokenKind},
    parser::{
        Parser,
        expressions::{
            Arguments, BinaryOperationExpression, BinaryOperator, ExpressionType,
            FunctionCallExpression, FunctionDeclarationExpression, ListExpression, LiteralType,
            UnaryOperationExpression, UnaryOperator,
        },
        statements::{Block, StatementType, VariableDeclarationStatement},
    },
};

#[test]
fn parses_list_declaration_complex() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("[", TokenKind::LeftBracket),
        Token::new("fn", TokenKind::Function),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("{", TokenKind::LeftCurly),
        Token::new("}", TokenKind::RightCurly),
        Token::new(",", TokenKind::Comma),
        Token::new("foo", TokenKind::Identifier),
        Token::new(",", TokenKind::Comma),
        Token::new("bar", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new(",", TokenKind::Comma),
        Token::new("[", TokenKind::LeftBracket),
        Token::new("]", TokenKind::RightBracket),
        Token::new(",", TokenKind::Comma),
        Token::new("-", TokenKind::Minus),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new("]", TokenKind::RightBracket),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::List(ListExpression {
                    values: vec![
                        ExpressionType::FunctionDeclaration(FunctionDeclarationExpression {
                            parameters: vec![],
                            body: Block { statements: vec![] }
                        }),
                        ExpressionType::Identifier(
                            crate::parser::expressions::IdentifierExpression {
                                name: String::from("foo")
                            }
                        ),
                        ExpressionType::FunctionCall(FunctionCallExpression {
                            name: String::from("bar"),
                            arguments: Arguments::new(vec![])
                        }),
                        ExpressionType::List(ListExpression { values: vec![] }),
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::UnaryOperation(UnaryOperationExpression::new(
                                UnaryOperator::Minus,
                                ExpressionType::Literal(LiteralType::Number(1.0)),
                            )),
                            BinaryOperator::Add,
                            ExpressionType::Literal(LiteralType::Number(2.0)),
                        ))
                    ]
                })
            }
        ),]
    )
}

#[test]
fn parses_list_declaration_literals() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("[", TokenKind::LeftBracket),
        Token::new("1", TokenKind::Number),
        Token::new(",", TokenKind::Comma),
        Token::new("Hello world", TokenKind::String),
        Token::new(",", TokenKind::Comma),
        Token::new("true", TokenKind::True),
        Token::new("]", TokenKind::RightBracket),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::List(ListExpression {
                    values: vec![
                        ExpressionType::Literal(LiteralType::Number(1.0)),
                        ExpressionType::Literal(LiteralType::String(String::from("Hello world"))),
                        ExpressionType::Literal(LiteralType::Boolean(true))
                    ]
                })
            }
        ),]
    )
}

#[test]
fn parses_list_declaration_empty() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("[", TokenKind::LeftBracket),
        Token::new("]", TokenKind::RightBracket),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::List(ListExpression { values: vec![] })
            }
        ),]
    )
}
