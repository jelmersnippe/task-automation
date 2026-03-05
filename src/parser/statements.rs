use super::Parser;
use super::expressions;
use crate::lexer::lexer::TokenKind;

#[derive(PartialEq, Debug)]
pub enum StatementType {
    VariableDeclaration(VariableDeclarationStatement),
    FunctionDeclaration(FunctionDeclarationStatement),
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

#[derive(PartialEq, Debug)]
pub struct FunctionDeclarationStatement {
    pub identifier: String,
    pub arguments: Vec<String>,
    pub body: Block,
}

impl Parser {
    pub(crate) fn parse_statement(&mut self) -> StatementType {
        if let Some(token) = self.next() {
            return match token.kind {
                TokenKind::Variable => self.parse_variable_statement(),
                TokenKind::Function => self.parse_function_declaration(),
                _ => panic!("Unknown token type in root parse"),
            };
        }

        panic!("No more tokens to parse");
    }

    fn parse_variable_statement(&mut self) -> StatementType {
        let identifier = self.expect(TokenKind::Identifier).value.clone();
        self.expect(TokenKind::Assign);
        let value = self.parse_expression();

        return StatementType::VariableDeclaration(VariableDeclarationStatement {
            identifier,
            value,
        });
    }

    fn parse_function_declaration(&mut self) -> StatementType {
        let identifier = self.expect(TokenKind::Identifier).value.clone();
        self.expect(TokenKind::LeftParenthesis);
        let mut arguments: Vec<String> = vec![];

        if !self.r#match(TokenKind::RightParenthesis) {
            loop {
                let identifier = self.expect(TokenKind::Identifier);
                arguments.push(identifier.value.clone());

                if self.r#match(TokenKind::RightParenthesis) {
                    break;
                }

                self.expect(TokenKind::Comma);
            }
        }

        let body = self.parse_block();

        return StatementType::FunctionDeclaration(FunctionDeclarationStatement {
            identifier,
            arguments,
            body,
        });
    }

    pub fn parse_block(&mut self) -> Block {
        self.expect(TokenKind::LeftCurly);
        // TODO
        self.expect(TokenKind::RightCurly);
        return Block { statements: vec![] };
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
            println!();
        }
        StatementType::FunctionDeclaration(function_declaration_statement) => {
            println!(
                "Function definition with identifier {}",
                function_declaration_statement.identifier
            );
            if function_declaration_statement.arguments.len() > 0 {
                println!("Arguments:",);
                function_declaration_statement
                    .arguments
                    .iter()
                    .for_each(|x| println!("\t{}", x));
            }
            if function_declaration_statement.body.statements.len() > 0 {
                println!("Body:");
                function_declaration_statement
                    .body
                    .statements
                    .iter()
                    .for_each(|x| print!("\t,{:?}", print_statement(x)));
            }
        }
    }
}
