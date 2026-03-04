use std::iter::Peekable;

use crate::lexer::lexer::{Token, TokenKind};

#[derive(PartialEq, Debug)]
pub enum StatementType {
    VariableDeclaration(VariableDeclarationStatement),
}

#[derive(PartialEq, Debug)]
pub enum ExpressionType {
    Literal(LiteralType),
}

#[derive(PartialEq, Debug)]
pub enum LiteralType {
    String(String),
    Number(f32),
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

pub fn parse(input: &Vec<Token>) -> Vec<StatementType> {
    let mut ast = Vec::<StatementType>::new();
    let mut tokens = input.iter().peekable();

    while let Some(token) = tokens.next() {
        match token.kind {
            TokenKind::Variable => {
                ast.push(parse_variable_statement(&mut tokens));
            }
            _ => panic!("Unknown token type in root parse"),
        }
    }

    return ast;
}

fn parse_variable_statement(tokens: &mut Peekable<std::slice::Iter<Token>>) -> StatementType {
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

fn parse_expression(tokens: &mut Peekable<std::slice::Iter<Token>>) -> ExpressionType {
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

pub fn print_ast(ast: &Vec<StatementType>) {
    ast.iter().for_each(|x| print_statement(x));
}

fn print_statement(statement: &StatementType) {
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

fn print_expression(expression: &ExpressionType) {
    match expression {
        ExpressionType::Literal(literal_type) => match literal_type {
            LiteralType::String(value) => println!("String literal with value '{}'", value),
            LiteralType::Number(value) => println!("Number literal with value {}", value),
        },
    }
}
