use std::iter::Peekable;

use crate::{
    lexer::lexer::{Token, TokenKind},
    parser::expressions::{ExpressionType, parse_expression, print_expression},
};

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
    pub value: ExpressionType,
}

pub fn parse_variable_statement(tokens: &mut Peekable<std::slice::Iter<Token>>) -> StatementType {
    if let Some(identifier_token) = tokens.next()
        && identifier_token.kind == TokenKind::Identifier
    {
        let identifier = identifier_token.value.clone();

        if let Some(assign_token) = tokens.next()
            && assign_token.kind == TokenKind::Assign
        {
            let value = parse_expression(tokens);

            return StatementType::VariableDeclaration(VariableDeclarationStatement {
                identifier,
                value,
            });
        }
    }

    panic!("No identifier found for variable statement")
}

pub fn print_statement(statement: &StatementType) {
    match statement {
        StatementType::VariableDeclaration(variable_declaration_statement) => {
            println!(
                "Variable declaration with identifier {}",
                variable_declaration_statement.identifier
            );
            print_expression(&variable_declaration_statement.value);
        }
    }
}
