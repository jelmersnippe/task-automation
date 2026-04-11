use super::Parser;
use super::expressions;
use crate::lexer::lexer::Token;
use crate::lexer::lexer::TokenKind;

#[derive(PartialEq, Debug, Clone)]
pub enum StatementType {
    VariableDeclaration(VariableDeclarationStatement),
    FunctionDeclaration(FunctionDeclarationStatement),
    Return(expressions::ExpressionType),
    FunctionCall(expressions::FunctionCallExpression),
    BuiltIn(BuiltInStatement),
    IfStatement(IfStatement),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Block {
    pub statements: Vec<StatementType>,
}

#[derive(PartialEq, Debug, Clone)]
pub struct VariableDeclarationStatement {
    pub identifier: String,
    pub value: expressions::ExpressionType,
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionDeclarationStatement {
    pub identifier: String,
    pub arguments: Vec<expressions::IdentifierExpression>,
    pub body: Block,
}

#[derive(PartialEq, Debug, Clone)]
pub struct IfStatement {
    pub condition: expressions::ExpressionType,
    pub body: Block,
}

#[derive(PartialEq, Debug, Clone)]
pub enum BuiltInStatement {
    Print(PrintStatement),
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrintStatement {
    pub argument: expressions::ExpressionType,
}

impl Parser {
    pub(crate) fn parse_statement(&mut self) -> StatementType {
        if let Some(token) = self.next() {
            return match token.kind {
                TokenKind::Variable => self.parse_variable_declaration(),
                TokenKind::Function => self.parse_function_declaration(),
                TokenKind::Return => self.parse_return_statement(),
                TokenKind::Identifier => self.parse_function_call_statement(token),
                TokenKind::Print => self.parse_print_statement(),
                TokenKind::If => self.parse_if_statement(),
                _ => panic!("Unknown token type in root parse: {:?}", token),
            };
        }

        panic!("No more tokens to parse");
    }

    fn parse_print_statement(&mut self) -> StatementType {
        self.expect(TokenKind::LeftParenthesis);
        let argument = self.parse_expression();
        self.expect(TokenKind::RightParenthesis);

        return StatementType::BuiltIn(BuiltInStatement::Print(PrintStatement { argument }));
    }

    fn parse_function_call_statement(&mut self, identifier_token: Token) -> StatementType {
        return match self.parse_function_expression(identifier_token) {
            expressions::ExpressionType::FunctionCall(function_call_expression) => {
                StatementType::FunctionCall(function_call_expression)
            }
            _ => panic!("Tried to parse function call statement but got invalid expression"),
        };
    }

    fn parse_return_statement(&mut self) -> StatementType {
        return StatementType::Return(self.parse_expression());
    }

    fn parse_variable_declaration(&mut self) -> StatementType {
        let identifier = self.expect(TokenKind::Identifier).value;
        self.expect(TokenKind::Assign);
        let value = self.parse_expression();

        return StatementType::VariableDeclaration(VariableDeclarationStatement {
            identifier,
            value,
        });
    }

    fn parse_function_declaration(&mut self) -> StatementType {
        let identifier = self.expect(TokenKind::Identifier).value;
        let mut arguments: Vec<expressions::IdentifierExpression> = vec![];

        self.expect(TokenKind::LeftParenthesis);

        if !self.r#match(TokenKind::RightParenthesis) {
            arguments.push(expressions::IdentifierExpression {
                name: self.expect(TokenKind::Identifier).value,
            });

            while self.r#match(TokenKind::Comma) {
                arguments.push(expressions::IdentifierExpression {
                    name: self.expect(TokenKind::Identifier).value,
                });
            }

            self.expect(TokenKind::RightParenthesis);
        }

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
        self.expect(TokenKind::LeftParenthesis);
        let condition = self.parse_expression();
        self.expect(TokenKind::RightParenthesis);

        self.expect(TokenKind::LeftCurly);
        let body = self.parse_block();

        return StatementType::IfStatement(IfStatement { condition, body });
    }
}

pub fn statement_to_string(statement: &StatementType) -> String {
    match statement {
        StatementType::VariableDeclaration(variable_declaration_statement) => format!(
            "Variable declaration\n\tIdentifier: '{}'\n\tValue: {}",
            variable_declaration_statement.identifier,
            expressions::expression_to_string(&variable_declaration_statement.value)
        ),
        StatementType::FunctionDeclaration(function_declaration_statement) => format!(
            "Function declaration\n\tIdentifier: '{}'\n\tArguments: {}\n\tBody:{}",
            function_declaration_statement.identifier,
            function_declaration_statement
                .arguments
                .iter()
                .fold(String::new(), |acc, cur| acc
                    + "\n\t\t"
                    + &expressions::expression_to_string(
                        &expressions::ExpressionType::Identifier(
                            expressions::IdentifierExpression {
                                name: cur.name.clone(),
                            },
                        ),
                    )),
            function_declaration_statement
                .body
                .statements
                .iter()
                .fold(String::new(), |acc, cur| acc
                    + "\n\t\t"
                    + &statement_to_string(cur))
        ),
        StatementType::Return(expression_type) => format!(
            "Return statement with expression: {}",
            expressions::expression_to_string(expression_type)
        ),
        StatementType::FunctionCall(function_call_expression) => {
            expressions::function_call_expression_to_string(function_call_expression)
        }
        StatementType::BuiltIn(built_in_statement) => {
            format!("Builtin: {:?}", built_in_statement)
        }
        StatementType::IfStatement(if_statement) => format!(
            "If statement\n\tCondition: '{}'\n\tBody: {}",
            expressions::expression_to_string(&if_statement.condition),
            if_statement
                .body
                .statements
                .iter()
                .fold(String::new(), |acc, cur| acc
                    + "\n\t\t"
                    + &statement_to_string(cur))
        ),
    }
}
