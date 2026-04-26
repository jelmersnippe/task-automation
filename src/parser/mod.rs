pub(crate) mod expressions;
pub(crate) mod statements;

#[cfg(test)]
mod tests;

use crate::lexer::{Token, TokenKind};

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

        while self.pos < self.tokens.len() {
            let statement = self.parse_statement();

            ast.push(statement)
        }

        return ast;
    }

    fn peek(&self) -> Option<Token> {
        return self.peek_ahead(1);
    }

    fn peek_ahead(&self, look_ahead: usize) -> Option<Token> {
        let corrected_look_ahead = self.pos + look_ahead - 1;
        if corrected_look_ahead >= self.tokens.len() {
            return None;
        }

        return Some(self.tokens[corrected_look_ahead].clone());
    }

    fn next(&mut self) -> Option<Token> {
        if self.pos >= self.tokens.len() {
            return None;
        }

        let token = Some(self.tokens[self.pos].clone());
        self.pos += 1;
        return token;
    }

    fn expect(&mut self, kind: TokenKind) -> Token {
        if let Some(next_token) = self.next() {
            if next_token.kind == kind {
                return next_token.clone();
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

    fn match_any(&mut self, kinds: &[TokenKind]) -> Option<Token> {
        if let Some(next_token) = self.peek()
            && kinds.contains(&next_token.kind)
        {
            self.pos += 1;
            return Some(next_token.clone());
        }

        return None;
    }
}
