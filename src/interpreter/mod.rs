use std::{collections::HashMap, rc::Rc};

use crate::parser::{
    expressions::{
        BinaryOperationExpression, BinaryOperator, ExpressionType, FunctionCallExpression,
        UnaryOperationExpression, UnaryOperator,
    },
    statements::{BuiltInStatement, StatementType},
};

#[cfg(test)]
mod tests;

#[derive(PartialEq, Debug, Clone)]
enum DataType {
    Number(f32),
    String(String),
    Boolean(bool),
    Function(FunctionDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
struct FunctionDeclaration {
    arguments: Vec<String>,
    body: Vec<StatementType>,
}

pub struct Interpreter {
    variables: HashMap<String, Rc<DataType>>,
    statements: Vec<StatementType>,
    pos: usize,
}

impl Interpreter {
    pub fn new(statements: Vec<StatementType>) -> Self {
        Self {
            variables: Default::default(),
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

    fn interpret_statement(&mut self, statement: &StatementType) -> Option<Rc<DataType>> {
        match statement {
            StatementType::VariableDeclaration(statement) => {
                let identifier = statement.identifier.clone();
                if let Some(_) = self.variables.get(&identifier) {
                    panic!("Duplicate identifier '{}' already declared", &identifier);
                }

                let value = statement.value.clone();
                let expression = self.interpret_expression(&value);

                self.variables.insert(identifier, expression);
                return None;
            }
            StatementType::FunctionDeclaration(statement) => {
                let identifier = statement.identifier.clone();
                if let Some(_) = self.variables.get(&identifier) {
                    panic!("Duplicate identifier '{}' already declared", &identifier);
                }

                let arguments = statement.arguments.iter().map(|x| x.name.clone()).collect();
                let statements = statement.body.statements.clone();

                self.variables.insert(
                    identifier,
                    Rc::new(DataType::Function(FunctionDeclaration {
                        arguments,
                        body: statements,
                    })),
                );

                return None;
            }
            StatementType::Return(expression) => {
                return Some(self.interpret_expression(expression));
            }
            StatementType::FunctionCall(statement) => return self.execute_function(statement),
            StatementType::BuiltIn(statement) => match statement {
                BuiltInStatement::Print(print_statement) => {
                    println!("{:?}", self.interpret_expression(&print_statement.argument));

                    return None;
                }
            },
            StatementType::IfStatement(statement) => {
                let condition_result = self.interpret_expression(&statement.condition);

                match *condition_result {
                    DataType::Boolean(should_execute) => {
                        if !should_execute {
                            return None;
                        }

                        let return_value =
                            self.execute_statements(statement.body.statements.iter().collect());

                        if let Some(_) = return_value {
                            panic!(
                                "If statement '{:?}' block contains return statement",
                                statement.condition
                            )
                        }

                        return None;
                    }
                    _ => panic!(
                        "Condition '{:?}' of if statement does not result in a boolean",
                        &statement.condition
                    ),
                }
            }
        }
    }

    fn execute_function(&mut self, statement: &FunctionCallExpression) -> Option<Rc<DataType>> {
        let function_declaration = if let Some(x) = self.variables.get(&statement.name) {
            match x.as_ref() {
                DataType::Function(function_declaration) => function_declaration.clone(),
                _ => panic!("Identifier '{}' is not callable", &statement.name),
            }
        } else {
            panic!("Identifier '{}' not found", &statement.name)
        };

        let expected_arguments = function_declaration.arguments.len();
        let received_arguments = statement.arguments.len();

        if expected_arguments != received_arguments {
            panic!(
                "Argument count for function '{}' is invalid. Expect: {}, received {}",
                &statement.name, expected_arguments, received_arguments
            );
        }

        let resolved_arguments: Vec<Rc<DataType>> = statement
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

        let return_value = self.execute_statements(function_declaration.body.iter().collect());

        // Remove argument variables afer scope ended
        for argument in function_declaration.arguments.iter() {
            self.variables.remove(argument);
        }

        return return_value;
    }

    fn execute_statements(&mut self, statements: Vec<&StatementType>) -> Option<Rc<DataType>> {
        let mut return_value: Option<Rc<DataType>> = None;
        let mut executed_statements: Vec<StatementType> = vec![];
        for x in statements {
            let statement_result = self.interpret_statement(x);
            executed_statements.push(x.clone());

            if let Some(value) = statement_result {
                return_value = Some(value);
                break;
            }
        }
        for x in executed_statements.iter().rev() {
            self.cleanup_statement(&x);
        }

        return return_value;
    }

    fn cleanup_statement(&mut self, statement: &StatementType) {
        match statement {
            StatementType::VariableDeclaration(statement) => {
                let identifier = statement.identifier.clone();
                self.variables.remove(&identifier);
            }
            StatementType::FunctionDeclaration(statement) => {
                let identifier = statement.identifier.clone();
                self.variables.remove(&identifier);
            }
            _ => {}
        }
    }

    fn interpret_expression(&mut self, expression: &ExpressionType) -> Rc<DataType> {
        match expression {
            ExpressionType::Literal(literal_type) => {
                return match literal_type {
                    crate::parser::expressions::LiteralType::String(x) => {
                        Rc::new(DataType::String(x.clone()))
                    }
                    crate::parser::expressions::LiteralType::Number(x) => {
                        Rc::new(DataType::Number(x.clone()))
                    }
                    crate::parser::expressions::LiteralType::Boolean(x) => {
                        Rc::new(DataType::Boolean(x.clone()))
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
            ExpressionType::FunctionCall(function_call_expression) => {
                if let Some(return_value) = self.execute_function(function_call_expression) {
                    return return_value;
                } else {
                    panic!(
                        "Function {} does not have a return value",
                        function_call_expression.name
                    )
                }
            }
            ExpressionType::FunctionDeclaration(function_declaration_expression) => {
                let arguments = function_declaration_expression
                    .parameters
                    .iter()
                    .map(|x| x.name.clone())
                    .collect();
                let statements = function_declaration_expression.body.statements.clone();

                return Rc::new(DataType::Function(FunctionDeclaration {
                    arguments,
                    body: statements,
                }));
            }
            ExpressionType::BinaryOperation(binary_operation_expression) => {
                Rc::new(self.interpret_binary_expression(binary_operation_expression))
            }
            ExpressionType::UnaryOperation(unary_operation_expression) => {
                Rc::new(self.interpret_unary_expression(unary_operation_expression))
            }
        }
    }

    fn interpret_unary_expression(&mut self, expression: &UnaryOperationExpression) -> DataType {
        let value = self.interpret_expression(&expression.expression);

        match *value {
            DataType::Number(x) => {
                if expression.operator == UnaryOperator::Minus {
                    DataType::Number(-x);
                }

                panic!(
                    "Unary operator '{:?}' not supported for number",
                    expression.operator
                );
            }
            DataType::Boolean(x) => {
                if expression.operator == UnaryOperator::Bang {
                    DataType::Boolean(!x);
                }

                panic!(
                    "Unary operator '{:?}' not supported for number",
                    expression.operator
                )
            }
            _ => panic!("Unsupported expression type for unary processing"),
        }
    }

    fn interpret_binary_expression(&mut self, expression: &BinaryOperationExpression) -> DataType {
        let left = self.interpret_expression(&expression.left);
        let right = self.interpret_expression(&expression.right);

        return match left.as_ref() {
            DataType::Number(l) => match right.as_ref() {
                DataType::Number(r) => match expression.operator {
                    BinaryOperator::Add => DataType::Number(l + r),
                    BinaryOperator::Subtract => DataType::Number(l - r),
                    BinaryOperator::Divide => DataType::Number(l / r),
                    BinaryOperator::Multiply => DataType::Number(l * r),
                    BinaryOperator::Equal => DataType::Boolean(l == r),
                    BinaryOperator::NotEqual => DataType::Boolean(l != r),
                    BinaryOperator::GreaterThan => DataType::Boolean(l > r),
                    BinaryOperator::LessThan => DataType::Boolean(l < r),
                    BinaryOperator::GreaterOrEqual => DataType::Boolean(l >= r),
                    BinaryOperator::LessOrEqual => DataType::Boolean(l <= r),
                    _ => panic!(
                        "Invalid operation for number binary operation: {} {:?} {}",
                        l, expression.operator, r
                    ),
                },
                _ => panic!(
                    "Left and right types of binary expression '{:?}' don't match",
                    expression
                ),
            },
            DataType::String(l) => match right.as_ref() {
                DataType::String(r) => {
                    return match expression.operator {
                        BinaryOperator::Add => DataType::String(format!("{}{}", l, r)),
                        BinaryOperator::Equal => DataType::Boolean(l == r),
                        BinaryOperator::NotEqual => DataType::Boolean(l != r),
                        BinaryOperator::GreaterThan => DataType::Boolean(l > r),
                        BinaryOperator::LessThan => DataType::Boolean(l < r),
                        BinaryOperator::GreaterOrEqual => DataType::Boolean(l >= r),
                        BinaryOperator::LessOrEqual => DataType::Boolean(l <= r),
                        _ => panic!(
                            "Invalid operation for string binary operation: {} {:?} {}",
                            &l, expression.operator, &r
                        ),
                    };
                }
                _ => panic!(
                    "Left and right of binary expression '{:?}' don't match",
                    expression
                ),
            },
            DataType::Boolean(left_value) => match right.as_ref() {
                DataType::Boolean(right_value) => {
                    let l = *left_value;
                    let r = *right_value;
                    match expression.operator {
                        BinaryOperator::Equal => DataType::Boolean(l == r),
                        BinaryOperator::NotEqual => DataType::Boolean(l != r),
                        BinaryOperator::And => DataType::Boolean(l && r),
                        BinaryOperator::Or => DataType::Boolean(l || r),
                        _ => panic!(
                            "Invalid operation for boolean binary operation: {} {:?} {}",
                            l, expression.operator, r
                        ),
                    }
                }
                _ => panic!(
                    "Left and right of binary expression '{:?}' don't match",
                    expression
                ),
            },
            _ => panic!("Invalid DataType used for binary expression"),
        };
    }
}
