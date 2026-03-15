use super::Parser;
use crate::lexer::lexer::TokenKind;

#[derive(PartialEq, Debug)]
pub enum ExpressionType {
    Literal(LiteralType),
    Identifier(IdentifierExpression),
}

#[derive(PartialEq, Debug)]
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

impl Parser {
    pub(crate) fn parse_expression(&mut self) -> ExpressionType {
        if let Some(token) = self.next() {
            return match token.kind {
                TokenKind::Number => ExpressionType::Literal(LiteralType::Number(
                    token.value.parse::<f32>().unwrap(),
                )),
                TokenKind::String => {
                    ExpressionType::Literal(LiteralType::String(token.value.clone()))
                }
                TokenKind::True => return ExpressionType::Literal(LiteralType::Boolean(true)),
                TokenKind::False => ExpressionType::Literal(LiteralType::Boolean(false)),
                TokenKind::Identifier => ExpressionType::Identifier(IdentifierExpression {
                    name: token.value.clone(),
                }),
                _ => panic!("Unsupported expression type {:?}", token.kind),
            };
        }

        panic!("No next token in parse_expression");
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
    };
}
