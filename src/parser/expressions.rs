use super::Parser;
use crate::lexer::lexer::{Token, TokenKind};

#[derive(PartialEq, Debug)]
pub enum ExpressionType {
    Literal(LiteralType),
    Identifier(IdentifierExpression),
    FunctionCall(FunctionCallExpression),
    BinaryOperation(BinaryOperationExpression),
    UnaryOperation(UnaryOperationExpression),
}

#[derive(PartialEq, Debug, Clone)]
pub enum LiteralType {
    String(String),
    Number(f32),
    Boolean(bool),
}

#[derive(PartialEq, Debug)]
pub struct IdentifierExpression {
    pub name: String,
}

#[derive(PartialEq, Debug)]
pub struct FunctionCallExpression {
    pub name: String,
    pub arguments: Vec<ExpressionType>,
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum BinaryOperator {
    // Arithmitic
    Add,
    Subtract,
    Divide,
    Multiply,

    // Comparison
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterOrEqual,
    LessOrEqual,

    // Logical
    And,
    Or,
}

impl From<TokenKind> for BinaryOperator {
    fn from(kind: TokenKind) -> Self {
        return match kind {
            TokenKind::GreaterThan => BinaryOperator::GreaterThan,
            TokenKind::GreaterOrEqual => BinaryOperator::GreaterOrEqual,
            TokenKind::LessThan => BinaryOperator::LessThan,
            TokenKind::LessOrEqual => BinaryOperator::LessOrEqual,
            TokenKind::Equal => BinaryOperator::Equal,
            TokenKind::NotEqual => BinaryOperator::NotEqual,
            TokenKind::And => BinaryOperator::And,
            TokenKind::Or => BinaryOperator::Or,
            TokenKind::Plus => BinaryOperator::Add,
            TokenKind::Minus => BinaryOperator::Subtract,
            TokenKind::Divide => BinaryOperator::Divide,
            TokenKind::Times => BinaryOperator::Multiply,
            _ => panic!("Can't convert {:?} to BinaryOperator", kind),
        };
    }
}

impl BinaryOperator {
    fn get_precedence(&self) -> i32 {
        match self {
            BinaryOperator::Or => 0,
            BinaryOperator::And => 1,
            BinaryOperator::Equal => 2,
            BinaryOperator::NotEqual => 2,
            BinaryOperator::GreaterThan => 2,
            BinaryOperator::LessThan => 2,
            BinaryOperator::GreaterOrEqual => 2,
            BinaryOperator::LessOrEqual => 2,
            BinaryOperator::Add => 3,
            BinaryOperator::Subtract => 3,
            BinaryOperator::Divide => 4,
            BinaryOperator::Multiply => 4,
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum UnaryOperator {
    Minus,
    Bang,
}

#[derive(PartialEq, Debug)]
pub struct BinaryOperationExpression {
    pub left: Box<ExpressionType>,
    pub operator: BinaryOperator,
    pub right: Box<ExpressionType>,
}

impl BinaryOperationExpression {
    pub fn new(left: ExpressionType, operator: BinaryOperator, right: ExpressionType) -> Self {
        return BinaryOperationExpression {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        };
    }

    pub fn insert_new_right(self, operator: BinaryOperator, right: ExpressionType) -> Self {
        if operator.get_precedence() > self.operator.get_precedence() {
            return BinaryOperationExpression::new(
                *self.left,
                self.operator,
                ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                    *self.right,
                    operator,
                    right,
                )),
            );
        } else {
            return match *self.right {
                ExpressionType::BinaryOperation(binary_operation_expression) => {
                    BinaryOperationExpression::new(
                        *self.left,
                        self.operator,
                        ExpressionType::BinaryOperation(
                            binary_operation_expression.insert_new_right(operator, right),
                        ),
                    )
                }
                _ => BinaryOperationExpression::new(
                    ExpressionType::BinaryOperation(self),
                    operator,
                    right,
                ),
            };
        };
    }
}

#[derive(PartialEq, Debug)]
pub struct UnaryOperationExpression {
    pub operator: UnaryOperator,
    pub expression: Box<ExpressionType>,
}

impl UnaryOperationExpression {
    pub fn new(operator: UnaryOperator, expression: ExpressionType) -> Self {
        return UnaryOperationExpression {
            operator,
            expression: Box::new(expression),
        };
    }
}

impl Parser {
    pub(crate) fn parse_expression(&mut self) -> ExpressionType {
        let mut left = self.parse_simple_expression();
        let mut operator: Option<BinaryOperator> = None;
        let mut right: Option<ExpressionType> = None;

        while let Some(binary_operator) = self.match_any(&[
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Times,
            TokenKind::Divide,
            TokenKind::Equal,
            TokenKind::NotEqual,
            TokenKind::LessThan,
            TokenKind::LessOrEqual,
            TokenKind::GreaterThan,
            TokenKind::GreaterOrEqual,
            TokenKind::And,
            TokenKind::Or,
        ]) {
            let new_operator = BinaryOperator::from(binary_operator.kind);
            let new_right = self.parse_simple_expression();

            // Create binary operation
            if let Some(r) = right
                && let Some(op) = operator
            {
                if new_operator.get_precedence() > op.get_precedence() {
                    // Create right associative binary operation
                    operator = Some(op);
                    right = match r {
                        ExpressionType::BinaryOperation(binary_operation_expression) => {
                            Some(ExpressionType::BinaryOperation(
                                binary_operation_expression
                                    .insert_new_right(new_operator, new_right),
                            ))
                        }
                        _ => Some(ExpressionType::BinaryOperation(
                            BinaryOperationExpression::new(r, new_operator, new_right),
                        )),
                    };
                } else {
                    // Create left associative binary operation
                    left = ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        left, op, r,
                    ));
                    operator = Some(new_operator);
                    right = Some(new_right);
                }
            // Set initial values for operator and right
            } else {
                operator = Some(new_operator);
                right = Some(new_right);
            }
        }

        if let Some(r) = right
            && let Some(op) = operator
        {
            return ExpressionType::BinaryOperation(BinaryOperationExpression::new(left, op, r));
        }

        return left;
    }

    fn parse_simple_expression(&mut self) -> ExpressionType {
        if let Some(token) = self.next() {
            return match token.kind {
                TokenKind::LeftParenthesis => {
                    let expression = self.parse_expression();

                    self.expect(TokenKind::RightParenthesis);

                    return expression;
                }
                TokenKind::Minus => ExpressionType::UnaryOperation(UnaryOperationExpression::new(
                    UnaryOperator::Minus,
                    self.parse_simple_expression(),
                )),
                TokenKind::Bang => ExpressionType::UnaryOperation(UnaryOperationExpression {
                    operator: UnaryOperator::Bang,
                    expression: Box::new(self.parse_simple_expression()),
                }),
                TokenKind::Number => ExpressionType::Literal(LiteralType::Number(
                    token.value.parse::<f32>().unwrap(),
                )),
                TokenKind::String => ExpressionType::Literal(LiteralType::String(token.value)),
                TokenKind::True => ExpressionType::Literal(LiteralType::Boolean(true)),
                TokenKind::False => ExpressionType::Literal(LiteralType::Boolean(false)),
                TokenKind::Identifier => self.parse_identifier_expression(token),
                _ => panic!(
                    "Unsupported token type for simple expression {:?}",
                    token.kind
                ),
            };
        }

        panic!("No next token in parse_expression")
    }

    fn parse_identifier_expression(&mut self, token: Token) -> ExpressionType {
        if token.kind != TokenKind::Identifier {
            panic!("Expected identifier token, found {:?}", token)
        }

        if self.r#match(TokenKind::LeftParenthesis) {
            return ExpressionType::FunctionCall(FunctionCallExpression {
                name: token.value,
                arguments: self.parse_arguments(),
            });
        }

        return ExpressionType::Identifier(IdentifierExpression { name: token.value });
    }

    pub(crate) fn parse_function_call_expression(
        &mut self,
        identifier_token: Token,
    ) -> FunctionCallExpression {
        self.expect(TokenKind::LeftParenthesis);

        return FunctionCallExpression {
            name: identifier_token.value,
            arguments: self.parse_arguments(),
        };
    }

    fn parse_arguments(&mut self) -> Vec<ExpressionType> {
        let mut arguments: Vec<ExpressionType> = vec![];

        if !self.r#match(TokenKind::RightParenthesis) {
            arguments.push(self.parse_expression());

            while self.r#match(TokenKind::Comma) {
                arguments.push(self.parse_expression());
            }

            self.expect(TokenKind::RightParenthesis);
        }

        return arguments;
    }
}

pub fn expression_to_string(expression: &ExpressionType) -> String {
    return match expression {
        ExpressionType::Literal(literal_type) => match literal_type {
            LiteralType::String(value) => format!("String literal with value '{}'", value),
            LiteralType::Number(value) => format!("Number literal with value {}", value),
            LiteralType::Boolean(value) => format!("Boolean literal with value {}", value),
        },
        ExpressionType::Identifier(identifier_expression) => {
            format!("Identifier '{}'", identifier_expression.name)
        }
        ExpressionType::FunctionCall(function_call_expression) => {
            function_call_expression_to_string(function_call_expression)
        }
        ExpressionType::BinaryOperation(binary_operation_expression) => todo!(),
        ExpressionType::UnaryOperation(unary_operation_expression) => todo!(),
    };
}

pub fn function_call_expression_to_string(expression: &FunctionCallExpression) -> String {
    return format!(
        "Function call\n\tName: '{}'\n\tArguments: {}",
        expression.name,
        expression
            .arguments
            .iter()
            .fold(String::new(), |acc, cur| acc + &expression_to_string(cur))
    );
}
