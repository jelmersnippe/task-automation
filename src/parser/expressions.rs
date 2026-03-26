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

#[derive(PartialEq, Debug)]
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
    // TODO: Support in lexer
    GreaterOrEqual,
    LessOrEqual,
    And,
    Or,
}

impl From<TokenKind> for BinaryOperator {
    fn from(kind: TokenKind) -> Self {
        return match kind {
            TokenKind::GreaterThan => BinaryOperator::GreaterThan,
            TokenKind::LessThan => BinaryOperator::LessThan,
            TokenKind::Equal => BinaryOperator::Equal,
            TokenKind::NotEqual => BinaryOperator::NotEqual,
            TokenKind::Plus => BinaryOperator::Add,
            TokenKind::Minus => BinaryOperator::Subtract,
            TokenKind::Divide => BinaryOperator::Divide,
            TokenKind::Times => BinaryOperator::Multiply,
            _ => panic!("Can't convert {:?} to BinaryOperator", kind),
        };
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

#[derive(PartialEq, Debug)]
pub struct UnaryOperationExpression {
    pub operator: UnaryOperator,
    pub expression: Box<ExpressionType>,
}

impl Parser {
    pub(crate) fn parse_expression(&mut self) -> ExpressionType {
        let expression = self.parse_simple_expression();

        return self.parse_binary_expression(expression);
    }

    fn parse_binary_expression(&mut self, expression: ExpressionType) -> ExpressionType {
        if let Some(binary_operator) = self.match_any(&[
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Times,
            TokenKind::Divide,
        ]) {
            return self
                .parse_binary_operation(expression, BinaryOperator::from(binary_operator.kind));
        }

        return expression;
    }

    fn parse_simple_expression(&mut self) -> ExpressionType {
        if let Some(token) = self.next() {
            return match token.kind {
                TokenKind::LeftParenthesis => {
                    let expression = self.parse_expression();

                    self.expect(TokenKind::RightParenthesis);

                    return expression;
                }
                TokenKind::Minus => ExpressionType::UnaryOperation(UnaryOperationExpression {
                    operator: UnaryOperator::Minus,
                    expression: Box::new(self.parse_simple_expression()),
                }),
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
                _ => panic!("Unsupported expression type {:?}", token.kind),
            };
        }

        panic!("No next token in parse_expression")
    }

    fn parse_binary_operation(
        &mut self,
        left: ExpressionType,
        operator: BinaryOperator,
    ) -> ExpressionType {
        let right = self.parse_expression();

        let is_low_prio_operator =
            matches!(operator, BinaryOperator::Add | BinaryOperator::Subtract);
        if is_low_prio_operator {
            return ExpressionType::BinaryOperation(BinaryOperationExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }

        if let ExpressionType::BinaryOperation(x) = right {
            return ExpressionType::BinaryOperation(BinaryOperationExpression {
                left: Box::new(ExpressionType::BinaryOperation(BinaryOperationExpression {
                    left: Box::new(left),
                    operator,
                    right: x.left,
                })),
                operator: x.operator,
                right: x.right,
            });
        } else {
            return ExpressionType::BinaryOperation(BinaryOperationExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            });
        }
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
