use super::Parser;
use super::expressions;
use crate::lexer::lexer::TokenKind;

#[derive(PartialEq, Debug)]
pub enum StatementType {
    VariableDeclaration(VariableDeclarationStatement),
}

#[derive(PartialEq, Debug)]
pub struct Block {
    pub statements: Vec<StatementType>,
}

#[derive(PartialEq, Debug)]
pub struct VariableDeclarationStatement {
    pub identifier: String,
    pub value: expressions::ExpressionType,
}

impl Parser {
    pub(crate) fn parse_variable_statement(&mut self) -> StatementType {
        if let Some(identifier_token) = self.next()
            && identifier_token.kind == TokenKind::Identifier
        {
            let identifier = identifier_token.value.clone();

            if let Some(assign_token) = self.next()
                && assign_token.kind == TokenKind::Assign
            {
                let value = self.parse_expression();

                return StatementType::VariableDeclaration(VariableDeclarationStatement {
                    identifier,
                    value,
                });
            }
        }

        panic!("No identifier found for variable statement")
    }
}

pub fn print_statement(statement: &StatementType) {
    match statement {
        StatementType::VariableDeclaration(variable_declaration_statement) => {
            println!(
                "Variable declaration with identifier {}",
                variable_declaration_statement.identifier
            );
            expressions::print_expression(&variable_declaration_statement.value);
        }
    }
}
