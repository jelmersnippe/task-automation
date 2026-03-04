mod expressions;
mod statements;

#[cfg(test)]
mod tests;

use crate::lexer::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn parse(&mut self) -> Vec<statements::StatementType> {
        let mut ast = Vec::<statements::StatementType>::new();

        while let Some(token) = self.next() {
            match token.kind {
                TokenKind::Variable => {
                    ast.push(self.parse_variable_statement());
                }
                _ => panic!("Unknown token type in root parse"),
            }
        }

        return ast;
    }

    fn peek(&self) -> Option<&Token> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        return Some(&self.tokens[self.pos]);
    }

    fn next(&mut self) -> Option<&Token> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        let token = Some(&self.tokens[self.pos]);
        self.pos += 1;
        return token;
    }

    fn expect(&mut self, kind: TokenKind) -> &Token {
        if let Some(next_token) = self.next() {
            if next_token.kind == kind {
                return next_token;
            }

            panic!(
                "Expected {:?} token but found {:?} token instead",
                kind, next_token.kind
            )
        }

        panic!(
            "Expected {:?} token but reached the end of the tokens",
            kind
        )
    }

    fn r#match(&mut self, kind: TokenKind) -> bool {
        if let Some(next_token) = self.peek()
            && next_token.kind == kind
        {
            self.pos += 1;
            return true;
        }

        return false;
    }
}

pub fn print_ast(ast: &Vec<statements::StatementType>) {
    ast.iter().for_each(|x| statements::print_statement(x));
}
