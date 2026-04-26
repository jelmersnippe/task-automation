use crate::{
    lexer::{Token, TokenKind},
    parser::{
        Parser,
        expressions::{
            BinaryOperationExpression, BinaryOperator, ExpressionType, IdentifierExpression,
            LiteralType, UnaryOperationExpression, UnaryOperator,
        },
        statements::{StatementType, VariableDeclarationStatement},
    },
};

// Truly evil
#[test]
fn parses_truly_evil_3() {
    // var x = ((1 + 2) * 3 > (4 + 5)) && ((6 < 7) || (8 > 9))
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
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
        Token::new(">", TokenKind::GreaterThan),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("4", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("5", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("&&", TokenKind::And),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("6", TokenKind::Number),
        Token::new("<", TokenKind::LessThan),
        Token::new("7", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("||", TokenKind::Or),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("8", TokenKind::Number),
        Token::new(">", TokenKind::GreaterThan),
        Token::new("9", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new(")", TokenKind::RightParenthesis),
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
                            ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                                ExpressionType::Literal(LiteralType::Number(1.0)),
                                BinaryOperator::Add,
                                ExpressionType::Literal(LiteralType::Number(2.0)),
                            )),
                            BinaryOperator::Multiply,
                            ExpressionType::Literal(LiteralType::Number(3.0)),
                        )),
                        BinaryOperator::GreaterThan,
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(4.0)),
                            BinaryOperator::Add,
                            ExpressionType::Literal(LiteralType::Number(5.0)),
                        )),
                    )),
                    BinaryOperator::And,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(6.0)),
                            BinaryOperator::LessThan,
                            ExpressionType::Literal(LiteralType::Number(7.0)),
                        )),
                        BinaryOperator::Or,
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(8.0)),
                            BinaryOperator::GreaterThan,
                            ExpressionType::Literal(LiteralType::Number(9.0)),
                        )),
                    )),
                )),
            }
        ),]
    )
}
#[test]
fn parses_truly_evil_2() {
    // var x = (1 + 2 * 3 > 5 && 4 < 10) || (6 == 6 && 7 > 8)
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("3", TokenKind::Number),
        Token::new(">", TokenKind::GreaterThan),
        Token::new("5", TokenKind::Number),
        Token::new("&&", TokenKind::And),
        Token::new("4", TokenKind::Number),
        Token::new("<", TokenKind::LessThan),
        Token::new("10", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("||", TokenKind::Or),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("6", TokenKind::Number),
        Token::new("==", TokenKind::Equal),
        Token::new("6", TokenKind::Number),
        Token::new("&&", TokenKind::And),
        Token::new("7", TokenKind::Number),
        Token::new(">", TokenKind::GreaterThan),
        Token::new("8", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
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
                            ExpressionType::Literal(LiteralType::Number(5.0)),
                        )),
                        BinaryOperator::And,
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(4.0)),
                            BinaryOperator::LessThan,
                            ExpressionType::Literal(LiteralType::Number(10.0)),
                        )),
                    )),
                    BinaryOperator::Or,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(6.0)),
                            BinaryOperator::Equal,
                            ExpressionType::Literal(LiteralType::Number(6.0)),
                        )),
                        BinaryOperator::And,
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(7.0)),
                            BinaryOperator::GreaterThan,
                            ExpressionType::Literal(LiteralType::Number(8.0)),
                        )),
                    )),
                )),
            }
        ),]
    )
}
#[test]
fn parses_truly_evil_1() {
    // var x = -((1 + 2) * (3 - 4) > 5) && !(6 < 7)
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("-", TokenKind::Minus),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("*", TokenKind::Times),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("3", TokenKind::Number),
        Token::new("-", TokenKind::Minus),
        Token::new("4", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new(">", TokenKind::GreaterThan),
        Token::new("5", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("&&", TokenKind::And),
        Token::new("!", TokenKind::Bang),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("6", TokenKind::Number),
        Token::new("<", TokenKind::LessThan),
        Token::new("7", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
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
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                                ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                                    ExpressionType::Literal(LiteralType::Number(1.0)),
                                    BinaryOperator::Add,
                                    ExpressionType::Literal(LiteralType::Number(2.0)),
                                )),
                                BinaryOperator::Multiply,
                                ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                                    ExpressionType::Literal(LiteralType::Number(3.0)),
                                    BinaryOperator::Subtract,
                                    ExpressionType::Literal(LiteralType::Number(4.0)),
                                )),
                            )),
                            BinaryOperator::GreaterThan,
                            ExpressionType::Literal(LiteralType::Number(5.0)),
                        )),
                    )),
                    BinaryOperator::And,
                    ExpressionType::UnaryOperation(UnaryOperationExpression::new(
                        UnaryOperator::Bang,
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(6.0)),
                            BinaryOperator::LessThan,
                            ExpressionType::Literal(LiteralType::Number(7.0)),
                        )),
                    )),
                )),
            }
        ),]
    )
}
// Deep nesting
#[test]
fn parses_deep_nesting_3() {
    // var x = (1 + 2 * 3 > 5 && 4 < 10) || (6 == 6)
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("3", TokenKind::Number),
        Token::new(">", TokenKind::GreaterThan),
        Token::new("5", TokenKind::Number),
        Token::new("&&", TokenKind::And),
        Token::new("4", TokenKind::Number),
        Token::new("<", TokenKind::LessThan),
        Token::new("10", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("||", TokenKind::Or),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("6", TokenKind::Number),
        Token::new("==", TokenKind::Equal),
        Token::new("6", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
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
                            ExpressionType::Literal(LiteralType::Number(5.0)),
                        )),
                        BinaryOperator::And,
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(4.0)),
                            BinaryOperator::LessThan,
                            ExpressionType::Literal(LiteralType::Number(10.0)),
                        )),
                    )),
                    BinaryOperator::Or,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(6.0)),
                        BinaryOperator::Equal,
                        ExpressionType::Literal(LiteralType::Number(6.0)),
                    )),
                )),
            }
        ),]
    )
}
#[test]
fn parses_deep_nesting_2() {
    // var x = ((1 + 2) * (3 + 4)) * (5 + 6)
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("(", TokenKind::LeftParenthesis),
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
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("*", TokenKind::Times),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("5", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("6", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
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
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(3.0)),
                            BinaryOperator::Add,
                            ExpressionType::Literal(LiteralType::Number(4.0)),
                        )),
                    )),
                    BinaryOperator::Multiply,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(5.0)),
                        BinaryOperator::Add,
                        ExpressionType::Literal(LiteralType::Number(6.0)),
                    )),
                )),
            }
        ),]
    )
}
#[test]
fn parses_deep_nesting_1() {
    // var x = (1 + (2 * (3 + (4 * 5))))
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("2", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("3", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("4", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("5", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    ExpressionType::Literal(LiteralType::Number(1.0)),
                    BinaryOperator::Add,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(2.0)),
                        BinaryOperator::Multiply,
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(3.0)),
                            BinaryOperator::Add,
                            ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                                ExpressionType::Literal(LiteralType::Number(4.0)),
                                BinaryOperator::Multiply,
                                ExpressionType::Literal(LiteralType::Number(5.0)),
                            )),
                        )),
                    )),
                )),
            }
        ),]
    )
}
// Associativity Traps
#[test]
fn parses_associativity_traps_6() {
    // var x = a || b && c || d && e
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("a", TokenKind::Identifier),
        Token::new("||", TokenKind::Or),
        Token::new("b", TokenKind::Identifier),
        Token::new("&&", TokenKind::And),
        Token::new("c", TokenKind::Identifier),
        Token::new("||", TokenKind::Or),
        Token::new("d", TokenKind::Identifier),
        Token::new("&&", TokenKind::And),
        Token::new("e", TokenKind::Identifier),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Identifier(IdentifierExpression {
                            name: String::from("a")
                        }),
                        BinaryOperator::Or,
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Identifier(IdentifierExpression {
                                name: String::from("b")
                            }),
                            BinaryOperator::And,
                            ExpressionType::Identifier(IdentifierExpression {
                                name: String::from("c")
                            }),
                        )),
                    )),
                    BinaryOperator::Or,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Identifier(IdentifierExpression {
                            name: String::from("d")
                        }),
                        BinaryOperator::And,
                        ExpressionType::Identifier(IdentifierExpression {
                            name: String::from("e")
                        }),
                    )),
                )),
            }
        ),]
    )
}
#[test]
fn parses_associativity_traps_5() {
    // var x = 1 + 2 * 3 + 4 * 5
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("3", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("4", TokenKind::Number),
        Token::new("*", TokenKind::Times),
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
                        ExpressionType::Literal(LiteralType::Number(1.0)),
                        BinaryOperator::Add,
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(2.0)),
                            BinaryOperator::Multiply,
                            ExpressionType::Literal(LiteralType::Number(3.0)),
                        )),
                    )),
                    BinaryOperator::Add,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(4.0)),
                        BinaryOperator::Multiply,
                        ExpressionType::Literal(LiteralType::Number(5.0)),
                    )),
                )),
            }
        ),]
    )
}
#[test]
fn parses_associativity_traps_4() {
    // var x = true || false || true
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("true", TokenKind::True),
        Token::new("||", TokenKind::Or),
        Token::new("false", TokenKind::False),
        Token::new("||", TokenKind::Or),
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
                    BinaryOperator::Or,
                    ExpressionType::Literal(LiteralType::Boolean(true)),
                )),
            }
        ),]
    )
}
#[test]
fn parses_associativity_traps_3() {
    // var x = true && false && true
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("true", TokenKind::True),
        Token::new("&&", TokenKind::And),
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
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Boolean(true)),
                        BinaryOperator::And,
                        ExpressionType::Literal(LiteralType::Boolean(false)),
                    )),
                    BinaryOperator::And,
                    ExpressionType::Literal(LiteralType::Boolean(true)),
                )),
            }
        ),]
    )
}
#[test]
fn parses_associativity_traps_2() {
    // var x = 1 < 2 < 3
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("1", TokenKind::Number),
        Token::new("<", TokenKind::LessThan),
        Token::new("2", TokenKind::Number),
        Token::new("<", TokenKind::LessThan),
        Token::new("3", TokenKind::Number),
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
                    BinaryOperator::LessThan,
                    ExpressionType::Literal(LiteralType::Number(3.0)),
                )),
            }
        ),]
    )
}
#[test]
fn parses_associativity_traps_1() {
    // var x = 1 - 2 - 3
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("1", TokenKind::Number),
        Token::new("-", TokenKind::Minus),
        Token::new("2", TokenKind::Number),
        Token::new("-", TokenKind::Minus),
        Token::new("3", TokenKind::Number),
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
                        BinaryOperator::Subtract,
                        ExpressionType::Literal(LiteralType::Number(2.0)),
                    )),
                    BinaryOperator::Subtract,
                    ExpressionType::Literal(LiteralType::Number(3.0)),
                )),
            }
        ),]
    )
}

// Mixed Everything
#[test]
fn parses_combinations_4() {
    // var x = (1 + 2 * 3 > 5) && (4 < 10)
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("3", TokenKind::Number),
        Token::new(">", TokenKind::GreaterThan),
        Token::new("5", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
        Token::new("&&", TokenKind::And),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("4", TokenKind::Number),
        Token::new("<", TokenKind::LessThan),
        Token::new("10", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
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
                            ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                                ExpressionType::Literal(LiteralType::Number(2.0)),
                                BinaryOperator::Multiply,
                                ExpressionType::Literal(LiteralType::Number(3.0)),
                            ))
                        )),
                        BinaryOperator::GreaterThan,
                        ExpressionType::Literal(LiteralType::Number(5.0)),
                    )),
                    BinaryOperator::And,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(4.0)),
                        BinaryOperator::LessThan,
                        ExpressionType::Literal(LiteralType::Number(10.0)),
                    ))
                ))
            }
        ),]
    )
}

#[test]
fn parses_combinations_3() {
    // var x = 1 + (2 * 3 > 5 && 4 < 10)
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("(", TokenKind::LeftParenthesis),
        Token::new("2", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("3", TokenKind::Number),
        Token::new(">", TokenKind::GreaterThan),
        Token::new("5", TokenKind::Number),
        Token::new("&&", TokenKind::And),
        Token::new("4", TokenKind::Number),
        Token::new("<", TokenKind::LessThan),
        Token::new("10", TokenKind::Number),
        Token::new(")", TokenKind::RightParenthesis),
    ])
    .parse();

    assert_eq!(
        result,
        vec![StatementType::VariableDeclaration(
            VariableDeclarationStatement {
                identifier: String::from("x"),
                value: ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    ExpressionType::Literal(LiteralType::Number(1.0)),
                    BinaryOperator::Add,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                                ExpressionType::Literal(LiteralType::Number(2.0)),
                                BinaryOperator::Multiply,
                                ExpressionType::Literal(LiteralType::Number(3.0)),
                            )),
                            BinaryOperator::GreaterThan,
                            ExpressionType::Literal(LiteralType::Number(5.0)),
                        )),
                        BinaryOperator::And,
                        ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                            ExpressionType::Literal(LiteralType::Number(4.0)),
                            BinaryOperator::LessThan,
                            ExpressionType::Literal(LiteralType::Number(10.0)),
                        )),
                    ))
                ))
            }
        ),]
    )
}

#[test]
fn parses_combinations_2() {
    // var x = (1 + 2) * 3 > 5 && 4 < 10
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
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
        Token::new("5", TokenKind::Number),
        Token::new("&&", TokenKind::And),
        Token::new("4", TokenKind::Number),
        Token::new("<", TokenKind::LessThan),
        Token::new("10", TokenKind::Number),
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
                            ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                                ExpressionType::Literal(LiteralType::Number(1.0)),
                                BinaryOperator::Add,
                                ExpressionType::Literal(LiteralType::Number(2.0)),
                            )),
                            BinaryOperator::Multiply,
                            ExpressionType::Literal(LiteralType::Number(3.0)),
                        )),
                        BinaryOperator::GreaterThan,
                        ExpressionType::Literal(LiteralType::Number(5.0)),
                    )),
                    BinaryOperator::And,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(4.0)),
                        BinaryOperator::LessThan,
                        ExpressionType::Literal(LiteralType::Number(10.0)),
                    ))
                ))
            }
        ),]
    )
}

#[test]
fn parses_combinations_1() {
    // var x = 1 + 2 * 3 > 5 && 4 < 10
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
        Token::new("x", TokenKind::Identifier),
        Token::new("=", TokenKind::Assign),
        Token::new("1", TokenKind::Number),
        Token::new("+", TokenKind::Plus),
        Token::new("2", TokenKind::Number),
        Token::new("*", TokenKind::Times),
        Token::new("3", TokenKind::Number),
        Token::new(">", TokenKind::GreaterThan),
        Token::new("5", TokenKind::Number),
        Token::new("&&", TokenKind::And),
        Token::new("4", TokenKind::Number),
        Token::new("<", TokenKind::LessThan),
        Token::new("10", TokenKind::Number),
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
                            ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                                ExpressionType::Literal(LiteralType::Number(2.0)),
                                BinaryOperator::Multiply,
                                ExpressionType::Literal(LiteralType::Number(3.0)),
                            ))
                        )),
                        BinaryOperator::GreaterThan,
                        ExpressionType::Literal(LiteralType::Number(5.0)),
                    )),
                    BinaryOperator::And,
                    ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        ExpressionType::Literal(LiteralType::Number(4.0)),
                        BinaryOperator::LessThan,
                        ExpressionType::Literal(LiteralType::Number(10.0)),
                    ))
                ))
            }
        ),]
    )
}

// Logical Operators
#[test]
fn parses_logical_operator_1() {
    // var x = true && false
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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

#[test]
fn parses_logical_operator_5() {
    // var x = 1 < 2 || 3 > 4 && 5 < 6
    let result = Parser::new(vec![
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
        Token::new("var", TokenKind::Var),
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
