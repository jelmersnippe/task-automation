use super::Parser;
use super::expressions::{ExpressionType, IdentifierExpression};
use crate::lexer::TokenKind;

#[derive(PartialEq, Debug, Clone)]
pub enum StatementType {
    VariableDeclaration(VariableDeclarationStatement),
    FunctionDeclaration(FunctionDeclarationStatement),
    Return(ExpressionType),
    IfStatement(IfStatement),
    While(While),
    Expression(ExpressionStatement),
}

#[derive(PartialEq, Debug, Clone)]
pub enum ExpressionStatement {
    Assignment(AssignmentStatement),
    Inline(ExpressionType),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Block {
    pub statements: Vec<StatementType>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct VariableDeclarationStatement {
    pub identifier: String,
    pub value: ExpressionType,
}

#[derive(PartialEq, Debug, Clone)]
pub struct AssignmentStatement {
    pub identifier: ExpressionType,
    pub value: ExpressionType,
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionDeclarationStatement {
    pub identifier: String,
    pub arguments: Vec<IdentifierExpression>,
    pub body: Block,
}

#[derive(PartialEq, Debug, Clone)]
pub struct IfStatement {
    pub condition: ExpressionType,
    pub body: Block,
}

#[derive(PartialEq, Debug, Clone)]
pub struct While {
    pub condition: ExpressionType,
    pub body: Block,
}

impl Parser {
    pub(crate) fn parse_statement(&mut self) -> StatementType {
        if let Some(token) = self.peek() {
            return match token.kind {
                TokenKind::Var => self.parse_variable_declaration(),
                TokenKind::Fn
                    if matches!(
                    self.peek_ahead(2),
                    Some(x) if x.kind == TokenKind::Identifier) =>
                {
                    self.parse_function_declaration()
                }
                TokenKind::Return => self.parse_return_statement(),
                TokenKind::If => self.parse_if_statement(),
                TokenKind::While => self.parse_while_statement(),
                _ => self.parse_expression_statement(),
            };
        }

        panic!("No more tokens to parse");
    }

    fn parse_expression_statement(&mut self) -> StatementType {
        let expression = self.parse_expression();

        if self.r#match(TokenKind::Assign) {
            let value = self.parse_expression();

            return StatementType::Expression(ExpressionStatement::Assignment(
                AssignmentStatement {
                    identifier: expression,
                    value,
                },
            ));
        }

        StatementType::Expression(ExpressionStatement::Inline(expression))
    }

    fn parse_return_statement(&mut self) -> StatementType {
        self.expect(TokenKind::Return);

        StatementType::Return(self.parse_expression())
    }

    fn parse_variable_declaration(&mut self) -> StatementType {
        self.expect(TokenKind::Var);

        let identifier = self.expect(TokenKind::Identifier).value;

        self.expect(TokenKind::Assign);
        let value = self.parse_expression();

        return StatementType::VariableDeclaration(VariableDeclarationStatement {
            identifier,
            value,
        });
    }

    fn parse_arguments(&mut self) -> Vec<IdentifierExpression> {
        let mut arguments: Vec<IdentifierExpression> = vec![];

        self.expect(TokenKind::LeftParenthesis);

        if !self.r#match(TokenKind::RightParenthesis) {
            arguments.push(IdentifierExpression {
                name: self.expect(TokenKind::Identifier).value,
            });

            while self.r#match(TokenKind::Comma) {
                arguments.push(IdentifierExpression {
                    name: self.expect(TokenKind::Identifier).value,
                });
            }

            self.expect(TokenKind::RightParenthesis);
        }

        arguments
    }

    fn parse_function_declaration(&mut self) -> StatementType {
        self.expect(TokenKind::Fn);

        let identifier = self.expect(TokenKind::Identifier).value;
        let arguments = self.parse_arguments();

        self.expect(TokenKind::LeftCurly);
        let body = self.parse_block();

        return StatementType::FunctionDeclaration(FunctionDeclarationStatement {
            identifier,
            arguments,
            body,
        });
    }

    pub fn parse_block(&mut self) -> Block {
        let mut statements: Vec<StatementType> = vec![];
        while !self.r#match(TokenKind::RightCurly) {
            statements.push(self.parse_statement())
        }

        return Block { statements };
    }

    fn parse_if_statement(&mut self) -> StatementType {
        self.expect(TokenKind::If);

        self.expect(TokenKind::LeftParenthesis);
        let condition = self.parse_expression();
        self.expect(TokenKind::RightParenthesis);

        self.expect(TokenKind::LeftCurly);
        let body = self.parse_block();

        return StatementType::IfStatement(IfStatement { condition, body });
    }

    fn parse_while_statement(&mut self) -> StatementType {
        self.expect(TokenKind::While);

        self.expect(TokenKind::LeftParenthesis);
        let condition = self.parse_expression();
        self.expect(TokenKind::RightParenthesis);

        self.expect(TokenKind::LeftCurly);
        let body = self.parse_block();

        return StatementType::While(While { condition, body });
    }
}
