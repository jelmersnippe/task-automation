#[cfg(test)]
mod binary;
#[cfg(test)]
mod list;

use crate::{
    lexer::lexer::{Token, TokenKind},
    parser::{
        Parser,
        expressions::{
            CallExpression, ExpressionType, FunctionDeclarationExpression, IdentifierExpression,
            LiteralType, Parameters, UnaryOperationExpression, UnaryOperator,
        },
        statements::{
            AssignmentStatement, Block, ExpressionStatement, FunctionDeclarationStatement,
            StatementType, VariableDeclarationStatement,
        },
    },
};

#[test]
fn parses_function_declaration_as_argument() {
    let result = Parser::new(vec![
        Token::new("inlineFunctionCall", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("fn", TokenKind::Fn),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("{", TokenKind::LeftCurly),
        Token::new("}", TokenKind::RightCurly),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::Expression(ExpressionStatement::Inline(
            ExpressionType::FunctionCall(CallExpression {
                value: Box::new(ExpressionType::Identifier(IdentifierExpression {
                    name: String::from("inlineFunctionCall")
                })),
                parameters: Parameters::new(vec![ExpressionType::FunctionDeclaration(
                    FunctionDeclarationExpression {
                        parameters: vec![],
                        body: Block { statements: vec![] }
                    }
                )])
            })
        ))]
    )
}

#[test]
fn parses_function_call_expression_without_arguments() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
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
                value: ExpressionType::FunctionCall(CallExpression {
                    value: Box::new(ExpressionType::Identifier(IdentifierExpression {
                        name: String::from("greet")
                    })),
                    parameters: Parameters::new(vec![])
                })
            }
        )]
    )
}

#[test]
fn parses_function_call_expression_with_literal_arguments() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("greet", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("5", TokenKind::Number),
        Token::new(",", TokenKind::Comma),
        Token::new("Hello World", TokenKind::String),
        Token::new(",", TokenKind::Comma),
        Token::new("true", TokenKind::True),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::FunctionCall(CallExpression {
                    value: Box::new(ExpressionType::Identifier(IdentifierExpression {
                        name: String::from("greet")
                    })),
                    parameters: Parameters::new(vec![
                        ExpressionType::Literal(LiteralType::Number(5 as f32)),
                        ExpressionType::Literal(LiteralType::String(String::from("Hello World"))),
                        ExpressionType::Literal(LiteralType::Boolean(true))
                    ])
                })
            }
        )]
    )
}

#[test]
fn parses_function_call_expression_with_identifier_argument() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
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
                value: ExpressionType::FunctionCall(CallExpression {
                    value: Box::new(ExpressionType::Identifier(IdentifierExpression {
                        name: String::from("greet")
                    })),
                    parameters: Parameters::new(vec![ExpressionType::Identifier(
                        IdentifierExpression {
                            name: String::from("x")
                        }
                    )])
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
        vec![StatementType::Expression(ExpressionStatement::Inline(
            ExpressionType::FunctionCall(CallExpression {
                value: Box::new(ExpressionType::Identifier(IdentifierExpression {
                    name: String::from("greet")
                })),
                parameters: Parameters::new(vec![])
            })
        ))]
    )
}

#[test]
fn parses_function_call_statement_with_literal_arguments() {
    let result = Parser::new(vec![
        Token::new("greet", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("5", TokenKind::Number),
        Token::new(",", TokenKind::Comma),
        Token::new("Hello World", TokenKind::String),
        Token::new(",", TokenKind::Comma),
        Token::new("true", TokenKind::True),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::Expression(ExpressionStatement::Inline(
            ExpressionType::FunctionCall(CallExpression {
                value: Box::new(ExpressionType::Identifier(IdentifierExpression {
                    name: String::from("greet")
                })),
                parameters: Parameters::new(vec![
                    ExpressionType::Literal(LiteralType::Number(5 as f32)),
                    ExpressionType::Literal(LiteralType::String(String::from("Hello World"))),
                    ExpressionType::Literal(LiteralType::Boolean(true))
                ])
            })
        ))]
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
        vec![StatementType::Expression(ExpressionStatement::Inline(
            ExpressionType::FunctionCall(CallExpression {
                value: Box::new(ExpressionType::Identifier(IdentifierExpression {
                    name: String::from("greet")
                })),
                parameters: Parameters::new(vec![ExpressionType::Identifier(
                    IdentifierExpression {
                        name: String::from("x")
                    }
                )])
            })
        ))]
    )
}

#[test]
fn parses_function_declaration_without_arguments() {
    let result = Parser::new(vec![
        Token::new("fn", TokenKind::Fn),
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
        Token::new("fn", TokenKind::Fn),
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
        Token::new("fn", TokenKind::Fn),
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
        Token::new("fn", TokenKind::Fn),
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
fn parses_variable_declaration_number() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("5", TokenKind::Number),
        Token::new("var", TokenKind::Var),
        Token::new("y", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("-", TokenKind::Minus),
        Token::new("5", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![
            StatementType::VariableDeclaration(VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::Literal(LiteralType::Number(5 as f32))
            }),
            StatementType::VariableDeclaration(VariableDeclarationStatement {
                identifier: String::from("y"),
                value: ExpressionType::UnaryOperation(UnaryOperationExpression {
                    expression: Box::new(ExpressionType::Literal(LiteralType::Number(5 as f32))),
                    operator: UnaryOperator::Minus
                })
            })
        ]
    )
}

#[test]
fn parses_variable_declaration_string() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
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
fn parses_variable_declaration_identifier() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
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
fn parses_variable_declaration_boolean() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("true", TokenKind::True),
        Token::new("var", TokenKind::Var),
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

#[test]
fn parses_variable_assignment_number() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("5", TokenKind::Number),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("-", TokenKind::Minus),
        Token::new("5", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![
            StatementType::VariableDeclaration(VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::Literal(LiteralType::Number(5 as f32))
            }),
            StatementType::Expression(ExpressionStatement::Assignment(AssignmentStatement {
                identifier: ExpressionType::Identifier(IdentifierExpression {
                    name: String::from("x")
                }),
                value: ExpressionType::UnaryOperation(UnaryOperationExpression {
                    expression: Box::new(ExpressionType::Literal(LiteralType::Number(5 as f32))),
                    operator: UnaryOperator::Minus
                })
            }))
        ]
    )
}

#[test]
fn parses_variable_assignment_string() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("Hello World", TokenKind::String),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("Update", TokenKind::String),
    ])
    .parse();

    assert_eq!(
        result,
        vec![
            StatementType::VariableDeclaration(VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::Literal(LiteralType::String(String::from("Hello World")))
            }),
            StatementType::Expression(ExpressionStatement::Assignment(AssignmentStatement {
                identifier: ExpressionType::Identifier(IdentifierExpression {
                    name: String::from("x")
                }),
                value: ExpressionType::Literal(LiteralType::String(String::from("Update")))
            }))
        ]
    )
}

#[test]
fn parses_variable_assignment_identifier() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("identifier", TokenKind::Identifier),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("identifier2", TokenKind::Identifier),
    ])
    .parse();

    assert_eq!(
        result,
        vec![
            StatementType::VariableDeclaration(VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::Identifier(IdentifierExpression {
                    name: String::from("identifier")
                })
            }),
            StatementType::Expression(ExpressionStatement::Assignment(AssignmentStatement {
                identifier: ExpressionType::Identifier(IdentifierExpression {
                    name: String::from("x")
                }),
                value: ExpressionType::Identifier(IdentifierExpression {
                    name: String::from("identifier2")
                })
            }))
        ]
    )
}

#[test]
fn parses_variable_assignment_boolean() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("true", TokenKind::True),
        Token::new("x", TokenKind::Identifier),
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
            StatementType::Expression(ExpressionStatement::Assignment(AssignmentStatement {
                identifier: ExpressionType::Identifier(IdentifierExpression {
                    name: String::from("x")
                }),
                value: ExpressionType::Literal(LiteralType::Boolean(false))
            })),
        ]
    )
}
