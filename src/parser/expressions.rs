use std::fmt;

use super::Parser;
use crate::{
    lexer::{Token, TokenKind},
    parser::statements::Block,
};

#[derive(PartialEq, Debug, Clone)]
pub enum ExpressionType {
    Literal(LiteralType),
    Identifier(IdentifierExpression),
    FunctionCall(CallExpression),
    Accessor(AccessorExpression),
    Property(PropertyExpression),
    FunctionDeclaration(FunctionDeclarationExpression),
    BinaryOperation(BinaryOperationExpression),
    UnaryOperation(UnaryOperationExpression),
    List(ListExpression),
    Dictionary(DictionaryExpression),
}

#[derive(PartialEq, Debug, Clone)]
pub struct DictionaryExpression {
    pub keys: Vec<ExpressionType>,
    pub values: Vec<ExpressionType>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct ListExpression {
    pub values: Vec<ExpressionType>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LiteralType {
    String(String),
    Number(f32),
    Boolean(bool),
    Undefined,
}

impl fmt::Display for LiteralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LiteralType::String(x) => write!(f, "{}", x),
            LiteralType::Number(x) => write!(f, "{}", x),
            LiteralType::Boolean(x) => write!(f, "{}", x),
            LiteralType::Undefined => write!(f, "undefined",),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct IdentifierExpression {
    pub name: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct CallExpression {
    pub value: Box<ExpressionType>,
    pub parameters: Vec<ExpressionType>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct AccessorExpression {
    pub value: Box<ExpressionType>,
    pub key: Box<ExpressionType>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct PropertyExpression {
    pub value: Box<ExpressionType>,
    pub key: Box<IdentifierExpression>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionDeclarationExpression {
    pub parameters: Vec<IdentifierExpression>,
    pub body: Block,
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

#[derive(PartialEq, Debug, Clone)]
pub enum UnaryOperator {
    Minus,
    Bang,
}

#[derive(PartialEq, Debug, Clone)]
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

#[derive(PartialEq, Debug, Clone)]
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
        let simple = self.parse_simple_expression();
        let mut expression = self.parse_postfix_chain(simple);
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
            let simple = self.parse_simple_expression();
            let new_right = self.parse_postfix_chain(simple);

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
                    expression = ExpressionType::BinaryOperation(BinaryOperationExpression::new(
                        expression, op, r,
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
            expression =
                ExpressionType::BinaryOperation(BinaryOperationExpression::new(expression, op, r));
        }

        self.parse_postfix_chain(expression)
    }

    fn parse_postfix_chain(&mut self, mut expression: ExpressionType) -> ExpressionType {
        while let Some(x) = self.match_any(&[
            TokenKind::LeftBracket,
            TokenKind::LeftParenthesis,
            TokenKind::Period,
        ]) {
            match x.kind {
                TokenKind::LeftBracket => {
                    let key = self.parse_expression();

                    self.expect(TokenKind::RightBracket);

                    expression = ExpressionType::Accessor(AccessorExpression {
                        value: Box::new(expression),
                        key: Box::new(key),
                    });
                }
                TokenKind::LeftParenthesis => {
                    let parameters = self.parse_comma_separated_list(TokenKind::RightParenthesis, false);

                    expression = ExpressionType::FunctionCall(CallExpression {
                        value: Box::new(expression),
                        parameters: parameters,
                    });
                }
                TokenKind::Period => {
                    let key = self.expect(TokenKind::Identifier);

                    expression = ExpressionType::Property(PropertyExpression {
                        value: Box::new(expression),
                        key: Box::new(IdentifierExpression { name: key.value }),
                    });
                }
                _ => panic!("Reached invalid expression statement parsing"),
            }
        }

        expression
    }

    fn parse_simple_expression(&mut self) -> ExpressionType {
        if let Some(token) = self.next() {
            return match token.kind {
                TokenKind::LeftParenthesis => {
                    let expression = self.parse_expression();

                    self.expect(TokenKind::RightParenthesis);

                    expression
                }
                TokenKind::Minus => {
                    let expression = self.parse_simple_expression();
                    ExpressionType::UnaryOperation(UnaryOperationExpression::new(
                        UnaryOperator::Minus,
                        expression,
                    ))
                }
                TokenKind::Bang => {
                    let expression = self.parse_simple_expression();

                    ExpressionType::UnaryOperation(UnaryOperationExpression::new(
                        UnaryOperator::Bang,
                        expression,
                    ))
                }
                TokenKind::Number => ExpressionType::Literal(LiteralType::Number(
                    token.value.parse::<f32>().unwrap(),
                )),
                TokenKind::String => ExpressionType::Literal(LiteralType::String(token.value)),
                TokenKind::True => ExpressionType::Literal(LiteralType::Boolean(true)),
                TokenKind::False => ExpressionType::Literal(LiteralType::Boolean(false)),
                TokenKind::Undefined => ExpressionType::Literal(LiteralType::Undefined),
                TokenKind::Identifier => self.parse_identifier_expression(token),
                TokenKind::Fn => self.parse_function_expression(token),
                TokenKind::LeftBracket => ExpressionType::List(ListExpression {
                    values: self.parse_comma_separated_list(TokenKind::RightBracket, true),
                }),
                TokenKind::LeftCurly => self.parse_dictionary_expression(),
                x => panic!("Invalid token for simple expression parsing: {:?}", x),
            };
        }

        panic!("No next token in parse_expression")
    }

    fn parse_identifier_expression(&mut self, token: Token) -> ExpressionType {
        if token.kind != TokenKind::Identifier {
            panic!("Expected identifier token, found {:?}", token)
        }

        if let Some(x) = self.peek()
            && x.kind == TokenKind::LeftParenthesis
        {
            return self.parse_function_expression(token);
        }

        return ExpressionType::Identifier(IdentifierExpression { name: token.value });
    }

    pub(crate) fn parse_function_expression(&mut self, identifier_token: Token) -> ExpressionType {
        self.expect(TokenKind::LeftParenthesis);

        let is_function_declaration = identifier_token.kind == TokenKind::Fn;

        if is_function_declaration {
            let parameters = self.parse_parameters();
            self.expect(TokenKind::LeftCurly);

            // Fn does not support break/continue. Pretend we are not in a loop while parsing body
            let prev_loop_depth = self.loop_depth;
            self.loop_depth = 0;
            let body = self.parse_block();
            self.loop_depth = prev_loop_depth;

            return ExpressionType::FunctionDeclaration(FunctionDeclarationExpression {
                parameters,
                body,
            });
        }

        ExpressionType::FunctionCall(CallExpression {
            value: Box::new(self.parse_identifier_expression(identifier_token)),
            parameters: self.parse_comma_separated_list(TokenKind::RightParenthesis, false),
        })
    }

    fn parse_parameters(&mut self) -> Vec<IdentifierExpression> {
        let mut parameters: Vec<IdentifierExpression> = vec![];

        if !self.r#match(TokenKind::RightParenthesis) {
            let expression = self.parse_expression();
            match expression {
                ExpressionType::Identifier(identifier_expression) => {
                    parameters.push(identifier_expression)
                }
                _ => panic!("Expected identifier while parsing parameters"),
            }

            while self.r#match(TokenKind::Comma) {
                let expression = self.parse_expression();
                match expression {
                    ExpressionType::Identifier(identifier_expression) => {
                        parameters.push(identifier_expression)
                    }
                    _ => panic!("Expected identifier while parsing parameters"),
                }
            }

            self.expect(TokenKind::RightParenthesis);
        }

        return parameters;
    }

    pub(crate) fn parse_comma_separated_list(
        &mut self,
        delimiter: TokenKind,
        trailing_comma: bool,
    ) -> Vec<ExpressionType> {
        let mut arguments: Vec<ExpressionType> = vec![];

        if !self.r#match(delimiter.clone()) {
            arguments.push(self.parse_expression());

            while self.r#match(TokenKind::Comma) {
                if trailing_comma
                    && self.peek().map(|t| t.kind == delimiter).unwrap_or(false)
                {
                    break;
                }
                arguments.push(self.parse_expression());
            }

            self.expect(delimiter);
        }

        return arguments;
    }

    fn parse_dictionary_expression(&mut self) -> ExpressionType {
        let mut keys: Vec<ExpressionType> = vec![];
        let mut values: Vec<ExpressionType> = vec![];
        while !self.r#match(TokenKind::RightCurly) {
            let key = self.parse_expression();
            match &key {
                ExpressionType::Identifier(identifier) => keys.push(ExpressionType::Literal(
                    LiteralType::String(identifier.name.clone()),
                )),
                ExpressionType::Literal(x) => {
                    keys.push(ExpressionType::Literal(LiteralType::String(x.to_string())))
                }
                ExpressionType::List(list) if list.values.len() == 1 => keys.push(key),
                _ => panic!(
                    "Invalid key for dictionary. Expected identifier, literal or [] with single expression, received: '{:?}'",
                    key
                ),
            }

            self.expect(TokenKind::Colon);

            let value = self.parse_expression();
            values.push(value);

            if self.peek().map(|t| t.kind == TokenKind::RightCurly).unwrap_or(false) {
                self.r#match(TokenKind::RightCurly);
                break;
            }
            self.expect(TokenKind::Comma);
        }

        return ExpressionType::Dictionary(DictionaryExpression { keys, values });
    }
}
