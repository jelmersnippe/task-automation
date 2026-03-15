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
}

#[derive(PartialEq, Debug)]
pub struct IdentifierExpression {
    pub name: String,
}

impl Parser {
    pub(crate) fn parse_expression(&mut self) -> ExpressionType {
        if let Some(token) = self.next() {
            match token.kind {
                TokenKind::Number => {
                    return ExpressionType::Literal(LiteralType::Number(
                        token.value.parse::<f32>().unwrap(),
                    ));
                }
                TokenKind::String => {
                    return ExpressionType::Literal(LiteralType::String(token.value.clone()));
                }
                TokenKind::Identifier => {
                    return ExpressionType::Identifier(IdentifierExpression {
                        name: token.value.clone(),
                    });
                }
                _ => panic!("Unsupported expression type {:?}", token.kind),
            }
        }

        panic!("No next token in parse_expression");
    }
}

pub fn expression_to_string(expression: &ExpressionType) -> String {
    match expression {
        ExpressionType::Literal(literal_type) => match literal_type {
            LiteralType::String(value) => return format!("String literal with value '{}'", value),
            LiteralType::Number(value) => return format!("Number literal with value {}", value),
        },
        ExpressionType::Identifier(identifier_expression) => {
            return format!("Identifier '{}'", identifier_expression.name);
        }
    }
}
