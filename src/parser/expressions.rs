use super::Parser;
use crate::lexer::lexer::{Token, TokenKind};

#[derive(PartialEq, Debug)]
pub enum ExpressionType {
    Literal(LiteralType),
    Identifier(IdentifierExpression),
    FunctionCall(FunctionCallExpression),
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
                TokenKind::String => ExpressionType::Literal(LiteralType::String(token.value)),
                TokenKind::True => ExpressionType::Literal(LiteralType::Boolean(true)),
                TokenKind::False => ExpressionType::Literal(LiteralType::Boolean(false)),
                TokenKind::Identifier => self.parse_identifier_expression(token),
                _ => panic!("Unsupported expression type {:?}", token.kind),
            };
        }

        panic!("No next token in parse_expression");
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

    pub(crate) fn parse_function_call_expression(&mut self) -> FunctionCallExpression {
        let identifier = self.expect(TokenKind::Identifier);

        self.expect(TokenKind::LeftParenthesis);

        return FunctionCallExpression {
            name: identifier.value,
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
