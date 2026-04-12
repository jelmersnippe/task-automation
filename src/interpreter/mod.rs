use std::{collections::HashMap, rc::Rc};

use crate::parser::{
    expressions::ExpressionType,
    statements::{BuiltInStatement, FunctionDeclarationStatement, StatementType},
};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug, Clone)]
enum Primitive {
    Number(f32),
    String(String),
    Boolean(bool),
}

struct StatementExecution {
    callback: Box<dyn FnOnce(&mut Interpreter)>,
    cleanup: Box<dyn FnOnce(&mut Interpreter, StatementType)>,
}

#[derive(Debug, PartialEq)]
struct FunctionDeclaration {
    arguments: Vec<String>,
    body: Vec<StatementType>,
}

pub struct Interpreter {
    variables: HashMap<String, Rc<Primitive>>,
    functions: HashMap<String, Rc<FunctionDeclaration>>,
    statements: Vec<StatementType>,
    pos: usize,
}

impl Interpreter {
    pub fn new(statements: Vec<StatementType>) -> Self {
        Self {
            variables: Default::default(),
            functions: Default::default(),
            statements,
            pos: 0,
        }
    }

    pub fn interpret(&mut self) {
        while self.pos < self.statements.len() {
            let statement = self.statements[self.pos].clone();
            self.interpret_statement(&statement);

            self.pos += 1;
        }
    }

    fn interpret_statement(&mut self, statement: &StatementType) {
        match statement {
            StatementType::VariableDeclaration(statement) => {
                let identifier = statement.identifier.clone();
                let value = statement.value.clone();
                let expression = self.interpret_expression(&value);
                self.variables.insert(identifier, expression);
            }
            StatementType::FunctionDeclaration(statement) => {
                let identifier = statement.identifier.clone();
                let arguments = statement.arguments.iter().map(|x| x.name.clone()).collect();
                let statements = statement.body.statements.clone();

                self.functions.insert(
                    identifier,
                    Rc::new(FunctionDeclaration {
                        arguments,
                        body: statements,
                    }),
                );
            }
            StatementType::Return(statement) => todo!(),
            StatementType::FunctionCall(statement) => {
                let function_declaration = {
                    let x = self.functions.get(&statement.name).unwrap();
                    x.clone()
                };

                let expected_arguments = function_declaration.arguments.len();
                let received_arguments = statement.arguments.len();

                if expected_arguments != received_arguments {
                    panic!(
                        "Argument count for function '{}' is invalid. Expect: {}, received {}",
                        &statement.name, expected_arguments, received_arguments
                    );
                }

                let resolved_arguments: Vec<Rc<Primitive>> = statement
                    .arguments
                    .iter()
                    .map(|x| self.interpret_expression(x))
                    .collect();

                // Set arguments as available variables
                for (identifier, value) in function_declaration
                    .arguments
                    .iter()
                    .zip(resolved_arguments)
                {
                    self.variables.insert(identifier.clone(), value);
                }

                for x in function_declaration.body.iter() {
                    self.interpret_statement(&x);
                }
                for x in function_declaration.body.iter().rev() {
                    self.cleanup_statement(&x);
                }

                // Remove argument variables afer scope ended
                for argument in function_declaration.arguments.iter() {
                    self.variables.remove(argument);
                }
            }
            StatementType::BuiltIn(statement) => match statement {
                BuiltInStatement::Print(print_statement) => {
                    println!("{:?}", self.interpret_expression(&print_statement.argument))
                }
            },
            StatementType::IfStatement(statement) => todo!(),
        }
    }

    fn cleanup_statement(&mut self, statement: &StatementType) {
        match statement {
            StatementType::VariableDeclaration(statement) => {
                let identifier = statement.identifier.clone();
                self.variables.remove(&identifier);
            }
            StatementType::FunctionDeclaration(statement) => {
                let identifier = statement.identifier.clone();
                self.functions.remove(&identifier);
            }
            _ => {}
        }
    }

    fn interpret_expression(&mut self, expression: &ExpressionType) -> Rc<Primitive> {
        match expression {
            ExpressionType::Literal(literal_type) => {
                return match literal_type {
                    crate::parser::expressions::LiteralType::String(x) => {
                        Rc::new(Primitive::String(x.clone()))
                    }
                    crate::parser::expressions::LiteralType::Number(x) => {
                        Rc::new(Primitive::Number(x.clone()))
                    }
                    crate::parser::expressions::LiteralType::Boolean(x) => {
                        Rc::new(Primitive::Boolean(x.clone()))
                    }
                };
            }
            ExpressionType::Identifier(identifier_expression) => {
                if let Some(var) = self.variables.get(&identifier_expression.name) {
                    return Rc::clone(var);
                } else {
                    panic!(
                        "Variable for identifier {} not found",
                        identifier_expression.name
                    )
                }
            }
            ExpressionType::FunctionCall(function_call_expression) => todo!(),
            ExpressionType::FunctionDeclaration(function_declaration_expression) => todo!(),
            ExpressionType::BinaryOperation(binary_operation_expression) => todo!(),
            ExpressionType::UnaryOperation(unary_operation_expression) => todo!(),
        }
    }
}
