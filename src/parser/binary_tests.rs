use std::fmt::Binary;

use crate::{
    lexer::lexer::{Token, TokenKind},
    parser::{
        Parser,
        expressions::{
            BinaryOperationExpression, BinaryOperator, ExpressionType, FunctionCallExpression,
            IdentifierExpression, LiteralType, UnaryOperationExpression, UnaryOperator,
        },
        statements::{StatementType, VariableDeclarationStatement},
    },
};

#[test]
fn parses_comparison_with_arithmetic() {
    // var x = 1 + 2 > 2
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new(">", TokenKind::GreaterThan),
        Token::new("2", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(1.0))),
                        operator: BinaryOperator::Add,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                    })),
                    operator: BinaryOperator::GreaterThan,
                    right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                })
            }
        ),]
    )
}

#[test]
fn parses_left_associativity_subtraction() {
    // var x = 10 - 5 - 2
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("10", TokenKind::Number),
        Token::new("-", TokenKind::Minus),
        Token::new("5", TokenKind::Number),
        Token::new("-", TokenKind::Minus),
        Token::new("2", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(10.0))),
                        operator: BinaryOperator::Subtract,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(5.0))),
                    })),
                    operator: BinaryOperator::Subtract,
                    right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                })
            }
        ),]
    )
}

#[test]
fn parses_unary_operation_with_parenthesis() {
    // var x = -(1 + 2)
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("-", TokenKind::Minus),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::UnaryOperation(UnaryOperationExpression {
                    expression: Box::new(ExpressionType::BinaryOperation(
                        BinaryOperationExpression {
                            left: Box::new(ExpressionType::Literal(LiteralType::Number(1.0))),
                            operator: BinaryOperator::Add,
                            right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                        }
                    )),
                    operator: UnaryOperator::Minus
                })
            }
        )]
    )
}

#[test]
fn parses_binary_operation_with_parenthesis() {
    // var x = (1 + 2 + (3 * 4)) * 5
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("3", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("4", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("*", TokenKind::Times),
        Token::new("5", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(1.0))),
                        operator: BinaryOperator::Add,
                        right: Box::new(ExpressionType::BinaryOperation(
                            BinaryOperationExpression {
                                left: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                                operator: BinaryOperator::Add,
                                right: Box::new(ExpressionType::BinaryOperation(
                                    BinaryOperationExpression {
                                        left: Box::new(ExpressionType::Literal(
                                            LiteralType::Number(3.0)
                                        )),
                                        operator: BinaryOperator::Multiply,
                                        right: Box::new(ExpressionType::Literal(
                                            LiteralType::Number(4.0)
                                        )),
                                    }
                                )),
                            }
                        )),
                    })),
                    operator: BinaryOperator::Multiply,
                    right: Box::new(ExpressionType::Literal(LiteralType::Number(5.0))),
                })
            }
        ),]
    )
}

#[test]
fn parses_binary_operation_with_nested_precedence() {
    // var x = 1 * 2 * 3 + 4 / 5 * 6 + 7
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("1", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("2", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("3", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("4", TokenKind::Number),
        Token::new("/", TokenKind::Divide),
        Token::new("5", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("6", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("7", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(1.0))),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(ExpressionType::BinaryOperation(
                            BinaryOperationExpression {
                                left: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                                operator: BinaryOperator::Multiply,
                                right: Box::new(ExpressionType::Literal(LiteralType::Number(3.0))),
                            }
                        ))
                    })),
                    operator: BinaryOperator::Add,
                    right: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::BinaryOperation(
                            BinaryOperationExpression {
                                left: Box::new(ExpressionType::Literal(LiteralType::Number(4.0))),
                                operator: BinaryOperator::Divide,
                                right: Box::new(ExpressionType::BinaryOperation(
                                    BinaryOperationExpression {
                                        left: Box::new(ExpressionType::Literal(
                                            LiteralType::Number(5.0)
                                        )),
                                        operator: BinaryOperator::Multiply,
                                        right: Box::new(ExpressionType::Literal(
                                            LiteralType::Number(6.0)
                                        )),
                                    }
                                ))
                            }
                        )),
                        operator: BinaryOperator::Add,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(7.0))),
                    }))
                })
            }
        ),]
    )
}

#[test]
fn parses_binary_operation_with_precedence() {
    // var a = 2 + 2 * 2 var b = 2 * 2 + 2
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("a", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("2", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("2", TokenKind::Number),
        Token::new("var", TokenKind::Variable),
        Token::new("b", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("2", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("2", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![
            StatementType::VariableDeclaration(VariableDeclarationStatement {
                identifier: String::from("a"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                    operator: BinaryOperator::Add,
                    right: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                    }))
                })
            }),
            StatementType::VariableDeclaration(VariableDeclarationStatement {
                identifier: String::from("b"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                    })),
                    operator: BinaryOperator::Add,
                    right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                })
            }),
        ]
    )
}

#[test]
fn parses_binary_operation_with_negative_numbers() {
    // var a = -2 + 2 var b = 2 - -2
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("a", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("-", TokenKind::Minus),
        Token::new("2", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new("var", TokenKind::Variable),
        Token::new("b", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("2", TokenKind::Number),
        Token::new("-", TokenKind::Minus),
        Token::new("-", TokenKind::Minus),
        Token::new("2", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![
            StatementType::VariableDeclaration(VariableDeclarationStatement {
                identifier: String::from("a"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::UnaryOperation(UnaryOperationExpression {
                        expression: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                        operator: UnaryOperator::Minus
                    })),
                    operator: BinaryOperator::Add,
                    right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                })
            }),
            StatementType::VariableDeclaration(VariableDeclarationStatement {
                identifier: String::from("b"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                    operator: BinaryOperator::Subtract,
                    right: Box::new(ExpressionType::UnaryOperation(UnaryOperationExpression {
                        operator: UnaryOperator::Minus,
                        expression: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                    }))
                })
            }),
        ]
    )
}

#[test]
fn parses_binary_operation_with_function_calls() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("a", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("foo", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("+", TokenKind::Plus),
        Token::new("bar", TokenKind::Identifier),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("a"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::FunctionCall(FunctionCallExpression {
                        name: String::from("foo"),
                        arguments: vec![]
                    })),
                    operator: BinaryOperator::Add,
                    right: Box::new(ExpressionType::FunctionCall(FunctionCallExpression {
                        name: String::from("bar"),
                        arguments: vec![]
                    })),
                })
            }
        ),]
    )
}

#[test]
fn parses_binary_operation_with_identifiers() {
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("a", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("foo", TokenKind::Identifier),
        Token::new("+", TokenKind::Plus),
        Token::new("bar", TokenKind::Identifier),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("a"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::Identifier(IdentifierExpression {
                        name: String::from("foo")
                    })),
                    operator: BinaryOperator::Add,
                    right: Box::new(ExpressionType::Identifier(IdentifierExpression {
                        name: String::from("bar")
                    })),
                })
            }
        ),]
    )
}

// Logical Operators
#[test]
fn parses_logical_operator_1() {
    // var x = true && false
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("true", TokenKind::True),
        Token::new("&&", TokenKind::And),
        Token::new("false", TokenKind::False),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    ExpressionType::Literal(LiteralType::Boolean(true)),
                    BinaryOperator::And,
                    ExpressionType::Literal(LiteralType::Boolean(false)),
                ))
            }
        ),]
    )
}

#[test]
fn parses_logical_operator_2() {
    // var x = true || false && true
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("true", TokenKind::True),
        Token::new("||", TokenKind::Or),
        Token::new("false", TokenKind::False),
        Token::new("&&", TokenKind::And),
        Token::new("true", TokenKind::True),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    ExpressionType::Literal(LiteralType::Boolean(true)),
                    BinaryOperator::Or,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Boolean(false)),
                        BinaryOperator::And,
                        ExpressionType::Literal(LiteralType::Boolean(true)),
                    )),
                ))
            }
        ),]
    )
}

#[test]
fn parses_logical_operator_3() {
    // var x = (true || false) && true
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("true", TokenKind::True),
        Token::new("||", TokenKind::Or),
        Token::new("false", TokenKind::False),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("&&", TokenKind::And),
        Token::new("true", TokenKind::True),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Boolean(true)),
                        BinaryOperator::Or,
                        ExpressionType::Literal(LiteralType::Boolean(false)),
                    )),
                    BinaryOperator::And,
                    ExpressionType::Literal(LiteralType::Boolean(true)),
                ))
            }
        ),]
    )
}

#[test]
fn parses_logical_operator_4() {
    // var x = 1 < 2 && 3 < 4
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("1", TokenKind::Number),
        Token::new("<", TokenKind::LessThan),
        Token::new("2", TokenKind::Number),
        Token::new("&&", TokenKind::And),
        Token::new("3", TokenKind::Number),
        Token::new("<", TokenKind::LessThan),
        Token::new("4", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(1.0)),
                        BinaryOperator::LessThan,
                        ExpressionType::Literal(LiteralType::Number(2.0)),
                    )),
                    BinaryOperator::And,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(3.0)),
                        BinaryOperator::LessThan,
                        ExpressionType::Literal(LiteralType::Number(4.0)),
                    ))
                ))
            }
        ),]
    )
}

// TODO:
// Mixed everything
// let var z = 1 + 2 * 3 > 5 && 4 < 10
// let var aa = (1 + 2) * 3 > 5 && 4 < 10
// let var ab = 1 + (2 * 3 > 5 && 4 < 10)
// let var ac = (1 + 2 * 3 > 5) && (4 < 10)
//
// Associativity Traps
// let var ad = 1 - 2 - 3
// let var ae = 1 / 2 / 3
// let var af = 1 < 2 < 3
// let var ag = true && false && true
// let var ah = true || false || true
//
// Deep Nesting
// let var ai = (1 + (2 * (3 + (4 * 5))))
// let var aj = ((1 + 2) * (3 + 4)) * (5 + 6)
// let var ak = (1 + 2 * 3 > 5 && 4 < 10) || (6 == 6)
//
// Truly evil
// let var al = -((1 + 2) * (3 - 4) > 5) && !(6 < 7)
// let var am = (1 + 2 * 3 > 5 && 4 < 10) || (6 == 6 && 7 > 8)
// let var an = ((1 + 2) * 3 > (4 + 5)) && ((6 < 7) || (8 > 9))

#[test]
fn parses_logical_operator_5() {
    // var x = 1 < 2 || 3 > 4 && 5 < 6
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("1", TokenKind::Number),
        Token::new("<", TokenKind::LessThan),
        Token::new("2", TokenKind::Number),
        Token::new("||", TokenKind::Or),
        Token::new("3", TokenKind::Number),
        Token::new(">", TokenKind::GreaterThan),
        Token::new("4", TokenKind::Number),
        Token::new("&&", TokenKind::And),
        Token::new("5", TokenKind::Number),
        Token::new("<", TokenKind::LessThan),
        Token::new("6", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(1.0)),
                        BinaryOperator::LessThan,
                        ExpressionType::Literal(LiteralType::Number(2.0)),
                    )),
                    BinaryOperator::Or,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(3.0)),
                            BinaryOperator::GreaterThan,
                            ExpressionType::Literal(LiteralType::Number(4.0)),
                        )),
                        BinaryOperator::And,
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(5.0)),
                            BinaryOperator::LessThan,
                            ExpressionType::Literal(LiteralType::Number(6.0)),
                        ))
                    ))
                ))
            }
        ),]
    )
}

// Comparison vs Arithmetic precedence
#[test]
fn parses_comparison_vs_arithmetic_precedence_5() {
    // var x = 10 - (5 == 5)
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("10", TokenKind::Number),
        Token::new("-", TokenKind::Minus),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("5", TokenKind::Number),
        Token::new("==", TokenKind::Equal),
        Token::new("5", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    ExpressionType::Literal(LiteralType::Number(10.0)),
                    BinaryOperator::Subtract,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(5.0)),
                        BinaryOperator::Equal,
                        ExpressionType::Literal(LiteralType::Number(5.0)),
                    ))
                ))
            }
        ),]
    )
}

#[test]
fn parses_comparison_vs_arithmetic_precedence_4() {
    // var x = 10 - 5 == 5
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("10", TokenKind::Number),
        Token::new("-", TokenKind::Minus),
        Token::new("5", TokenKind::Number),
        Token::new("==", TokenKind::Equal),
        Token::new("5", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(10.0)),
                        BinaryOperator::Subtract,
                        ExpressionType::Literal(LiteralType::Number(5.0)),
                    )),
                    BinaryOperator::Equal,
                    ExpressionType::Literal(LiteralType::Number(5.0)),
                ))
            }
        ),]
    )
}

#[test]
fn parses_comparison_vs_arithmetic_precedence_3() {
    // var x = (1 + 2) * 3 > 6
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("*", TokenKind::Times),
        Token::new("3", TokenKind::Number),
        Token::new(">", TokenKind::GreaterThan),
        Token::new("6", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(1.0)),
                            BinaryOperator::Add,
                            ExpressionType::Literal(LiteralType::Number(2.0)),
                        )),
                        BinaryOperator::Multiply,
                        ExpressionType::Literal(LiteralType::Number(3.0)),
                    )),
                    BinaryOperator::GreaterThan,
                    ExpressionType::Literal(LiteralType::Number(6.0)),
                ))
            }
        ),]
    )
}

#[test]
fn parses_comparison_vs_arithmetic_precedence_2() {
    // var x = 1 + 2 * 3 > 6
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("3", TokenKind::Number),
        Token::new(">", TokenKind::GreaterThan),
        Token::new("6", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(1.0)),
                        BinaryOperator::Add,
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(2.0)),
                            BinaryOperator::Multiply,
                            ExpressionType::Literal(LiteralType::Number(3.0)),
                        )),
                    )),
                    BinaryOperator::GreaterThan,
                    ExpressionType::Literal(LiteralType::Number(6.0)),
                ))
            }
        ),]
    )
}

#[test]
fn parses_comparison_vs_arithmetic_precedence_1() {
    // var x = 1 + 2 > 2
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new(">", TokenKind::GreaterThan),
        Token::new("2", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(1.0)),
                        BinaryOperator::Add,
                        ExpressionType::Literal(LiteralType::Number(2.0)),
                    )),
                    BinaryOperator::GreaterThan,
                    ExpressionType::Literal(LiteralType::Number(2.0)),
                ))
            }
        ),]
    )
}

// Unary + binary interaction
#[test]
fn parses_unary_5() {
    // var x = -(-1 + 2)
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("-", TokenKind::Minus),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("-", TokenKind::Minus),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::UnaryOperation(UnaryOperationExpression::new(
                    UnaryOperator::Minus,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::UnaryOperation(UnaryOperationExpression::new(
                            UnaryOperator::Minus,
                            ExpressionType::Literal(LiteralType::Number(1.0)),
                        )),
                        BinaryOperator::Add,
                        ExpressionType::Literal(LiteralType::Number(2.0))
                    ))
                ))
            }
        ),]
    )
}

#[test]
fn parses_unary_4() {
    // var x = -(1 * 2)
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("-", TokenKind::Minus),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("1", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("2", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::UnaryOperation(UnaryOperationExpression::new(
                    UnaryOperator::Minus,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(1.0)),
                        BinaryOperator::Multiply,
                        ExpressionType::Literal(LiteralType::Number(2.0)),
                    ))
                ))
            }
        ),]
    )
}

#[test]
fn parses_unary_3() {
    // var x = -1 * 2
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("-", TokenKind::Minus),
        Token::new("1", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("2", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    ExpressionType::UnaryOperation(UnaryOperationExpression::new(
                        UnaryOperator::Minus,
                        ExpressionType::Literal(LiteralType::Number(1.0)),
                    )),
                    BinaryOperator::Multiply,
                    ExpressionType::Literal(LiteralType::Number(2.0))
                ))
            }
        ),]
    )
}

#[test]
fn parses_unary_2() {
    // var x = -(1 + 2)
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("-", TokenKind::Minus),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::UnaryOperation(UnaryOperationExpression::new(
                    UnaryOperator::Minus,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(1.0)),
                        BinaryOperator::Add,
                        ExpressionType::Literal(LiteralType::Number(2.0))
                    )),
                ))
            }
        ),]
    )
}

#[test]
fn parses_unary_1() {
    // var x = -1 + 2
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("-", TokenKind::Minus),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    ExpressionType::UnaryOperation(UnaryOperationExpression::new(
                        UnaryOperator::Minus,
                        ExpressionType::Literal(LiteralType::Number(1.0)),
                    )),
                    BinaryOperator::Add,
                    ExpressionType::Literal(LiteralType::Number(2.0)),
                ))
            }
        ),]
    )
}

// Parenthesis Overriding Precedence
#[test]
fn parses_parenthesis_overriding_precedence_5() {
    // var x = ((1 + 2) * 3) + 4
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("*", TokenKind::Times),
        Token::new("3", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("+", TokenKind::Plus),
        Token::new("4", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::BinaryOperation(
                            BinaryOperationExpression {
                                left: Box::new(ExpressionType::Literal(LiteralType::Number(1.0))),
                                operator: BinaryOperator::Add,
                                right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0)))
                            }
                        )),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(3.0)))
                    })),
                    operator: BinaryOperator::Add,
                    right: Box::new(ExpressionType::Literal(LiteralType::Number(4.0))),
                })
            }
        ),]
    )
}

#[test]
fn parses_parenthesis_overriding_precedence_4() {
    // var x = (1 + 2) * (3 + 4)
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("*", TokenKind::Times),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("3", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("4", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(1.0))),
                        operator: BinaryOperator::Add,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0)))
                    })),
                    operator: BinaryOperator::Multiply,
                    right: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(3.0))),
                        operator: BinaryOperator::Add,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(4.0)))
                    }))
                })
            }
        ),]
    )
}

#[test]
fn parses_parenthesis_overriding_precedence_3() {
    // var x = 10 - (5 - 2)
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("10", TokenKind::Number),
        Token::new("-", TokenKind::Minus),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("5", TokenKind::Number),
        Token::new("-", TokenKind::Minus),
        Token::new("2", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::Literal(LiteralType::Number(10.0))),
                    operator: BinaryOperator::Subtract,
                    right: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(5.0))),
                        operator: BinaryOperator::Subtract,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0)))
                    }))
                })
            }
        ),]
    )
}

#[test]
fn parses_parenthesis_overriding_precedence_2() {
    // var x = (10 - 5) - 2
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("10", TokenKind::Number),
        Token::new("-", TokenKind::Minus),
        Token::new("5", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("-", TokenKind::Minus),
        Token::new("2", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(10.0))),
                        operator: BinaryOperator::Subtract,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(5.0)))
                    })),
                    operator: BinaryOperator::Subtract,
                    right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                })
            }
        ),]
    )
}

#[test]
fn parses_parenthesis_overriding_precedence_1() {
    // var x = (1 + 2) * 3
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("*", TokenKind::Times),
        Token::new("3", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(1.0))),
                        operator: BinaryOperator::Add,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0)))
                    })),
                    operator: BinaryOperator::Multiply,
                    right: Box::new(ExpressionType::Literal(LiteralType::Number(3.0))),
                })
            }
        ),]
    )
}

// Basic Artithmetic Precedence
#[test]
fn parses_basic_arithmetic_precedence_5() {
    // var x = 10 / (2 * 3)
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("10", TokenKind::Number),
        Token::new("/", TokenKind::Divide),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("2", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("3", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::Literal(LiteralType::Number(10.0))),
                    operator: BinaryOperator::Divide,
                    right: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(3.0)))
                    }))
                })
            }
        ),]
    )
}

#[test]
fn parses_basic_arithmetic_precedence_4() {
    // var x = 10 / 2 * 3
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("10", TokenKind::Number),
        Token::new("/", TokenKind::Divide),
        Token::new("2", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("3", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(10.0))),
                        operator: BinaryOperator::Divide,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0)))
                    })),
                    operator: BinaryOperator::Multiply,
                    right: Box::new(ExpressionType::Literal(LiteralType::Number(3.0))),
                })
            }
        ),]
    )
}

#[test]
fn parses_basic_arithmetic_precedence_3() {
    // var x = 10 - 5 - 2
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("10", TokenKind::Number),
        Token::new("-", TokenKind::Minus),
        Token::new("5", TokenKind::Number),
        Token::new("-", TokenKind::Minus),
        Token::new("2", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(10.0))),
                        operator: BinaryOperator::Subtract,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(5.0)))
                    })),
                    operator: BinaryOperator::Subtract,
                    right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                })
            }
        ),]
    )
}

#[test]
fn parses_basic_arithmetic_precedence_2() {
    // var x = 1 * 2 + 3
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("1", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("2", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("3", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(1.0))),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(2.0)))
                    })),
                    operator: BinaryOperator::Add,
                    right: Box::new(ExpressionType::Literal(LiteralType::Number(3.0))),
                })
            }
        ),]
    )
}

#[test]
fn parses_basic_arithmetic_precedence_1() {
    // var x = 1 + 2 * 3
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Variable),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("3", TokenKind::Number),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(ExpressionType::Literal(LiteralType::Number(1.0))),
                    operator: BinaryOperator::Add,
                    right: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                        left: Box::new(ExpressionType::Literal(LiteralType::Number(2.0))),
                        operator: BinaryOperator::Multiply,
                        right: Box::new(ExpressionType::Literal(LiteralType::Number(3.0)))
                    }))
                })
            }
        ),]
    )
}
