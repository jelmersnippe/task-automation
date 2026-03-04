use crate::{
    lexer::lexer::{Token, TokenKind},
    parser::statements::{StatementType, parse_variable_statement, print_statement},
};

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

pub fn print_ast(ast: &Vec<StatementType>) {
    ast.iter().for_each(|x| print_statement(x));
}
