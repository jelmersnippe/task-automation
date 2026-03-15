use crate::{
    lexer::lexer::{LiteralType, Token, TokenKind},
    parser::{
        Parser,
        expressions::{ExpressionType, FunctionCallExpression, IdentifierExpression},
        statements::{
            Block, BuiltInStatement, FunctionDeclarationStatement, PrintStatement, StatementType,
            VariableDeclarationStatement,
        },
    },
};

#[test]
fn parses_binary_operation_expressions() {
    let result = Parser::new(vec![
        Token::new("print", TokenKind::Print),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("x", TokenKind::Identifier),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::BuiltIn(BuiltInStatement::Print(
            PrintStatement {
                argument: ExpressionType::Identifier(IdentifierExpression {
                    name: String::from("x")
                })
            }
        ))]
    )
}

#[test]
fn parses_built_in_print() {
    let result = Parser::new(vec![
        Token::new("print", TokenKind::Print),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("x", TokenKind::Identifier),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::BuiltIn(BuiltInStatement::Print(
            PrintStatement {
                argument: ExpressionType::Identifier(IdentifierExpression {
                    name: String::from("x")
                })
            }
        ))]
    )
}

#[test]
fn parses_function_call_expression_without_arguments() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("greet", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::FunctionCall(FunctionCallExpression {
                    name: String::from("greet"),
                    arguments: vec![]
                })
            }
        )]
    )
}

#[test]
fn parses_function_call_expression_with_literal_arguments() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("greet", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("5", TokenKind::Literal(LiteralType::Number(5.0))),
        Token::new(",", TokenKind::Comma),
        Token::new(
            "Hello World",
            TokenKind::Literal(LiteralType::String(String::from("Hello World"))),
        ),
        Token::new(",", TokenKind::Comma),
        Token::new("true", TokenKind::Literal(LiteralType::Boolean(true))),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::FunctionCall(FunctionCallExpression {
                    name: String::from("greet"),
                    arguments: vec![
                        ExpressionType::Literal(LiteralType::Number(5 as f32)),
                        ExpressionType::Literal(LiteralType::String(String::from("Hello World"))),
                        ExpressionType::Literal(LiteralType::Boolean(true))
                    ]
                })
            }
        )]
    )
}

#[test]
fn parses_function_call_expression_with_identifier_argument() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("greet", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("x", TokenKind::Identifier),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::FunctionCall(FunctionCallExpression {
                    name: String::from("greet"),
                    arguments: vec![ExpressionType::Identifier(IdentifierExpression {
                        name: String::from("x")
                    })]
                })
            }
        )]
    )
}

#[test]
fn parses_function_call_statement_without_arguments() {
    let result = Parser::new(vec![
        Token::new("greet", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::FunctionCall(FunctionCallExpression {
            name: String::from("greet"),
            arguments: vec![]
        })]
    )
}

#[test]
fn parses_function_call_statement_with_literal_arguments() {
    let result = Parser::new(vec![
        Token::new("greet", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("5", TokenKind::Literal(LiteralType::Number(5.0))),
        Token::new(",", TokenKind::Comma),
        Token::new(
            "Hello World",
            TokenKind::Literal(LiteralType::String(String::from("Hello World"))),
        ),
        Token::new(",", TokenKind::Comma),
        Token::new("true", TokenKind::Literal(LiteralType::Boolean(true))),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::FunctionCall(FunctionCallExpression {
            name: String::from("greet"),
            arguments: vec![
                ExpressionType::Literal(LiteralType::Number(5 as f32)),
                ExpressionType::Literal(LiteralType::String(String::from("Hello World"))),
                ExpressionType::Literal(LiteralType::Boolean(true))
            ]
        })]
    )
}

#[test]
fn parses_function_call_statement_with_identifier_argument() {
    let result = Parser::new(vec![
        Token::new("greet", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("x", TokenKind::Identifier),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::FunctionCall(FunctionCallExpression {
            name: String::from("greet"),
            arguments: vec![ExpressionType::Identifier(IdentifierExpression {
                name: String::from("x")
            })]
        })]
    )
}

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
        Token::new("5", TokenKind::Literal(LiteralType::Number(5.0))),
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
        Token::new("5", TokenKind::Literal(LiteralType::Number(5.0))),
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
        Token::new(
            "Hello World",
            TokenKind::Literal(LiteralType::String(String::from("Hello World"))),
        ),
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
        Token::new("true", TokenKind::Literal(LiteralType::Boolean(true))),
        Token::new("var", TokenKind::Variable),
        Token::new("y", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("false", TokenKind::Literal(LiteralType::Boolean(false))),
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
