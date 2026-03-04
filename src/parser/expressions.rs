use std::iter::Peekable;

use crate::lexer::lexer::{Token, TokenKind};

#[derive(PartialEq, Debug)]
pub enum ExpressionType {
    Literal(LiteralType),
}

#[derive(PartialEq, Debug)]
pub enum LiteralType {
    String(String),
    Number(f32),
}

pub fn parse_expression(tokens: &mut Peekable<std::slice::Iter<Token>>) -> ExpressionType {
    if let Some(token) = tokens.next() {
        match token.kind {
            TokenKind::Number => {
                return ExpressionType::Literal(LiteralType::Number(
                    token.value.parse::<f32>().unwrap(),
                ));
            }
            TokenKind::String => {
                return ExpressionType::Literal(LiteralType::String(token.value.clone()));
            }
            _ => panic!("Unsupported expression type {:?}", token.kind),
        }
    }

    panic!("No next token in parse_expression");
}

pub fn print_expression(expression: &ExpressionType) {
    match expression {
        ExpressionType::Literal(literal_type) => match literal_type {
            LiteralType::String(value) => println!("String literal with value '{}'", value),
            LiteralType::Number(value) => println!("Number literal with value {}", value),
        },
    }
}
