pub(crate) mod function;
pub(crate) mod scope;

use std::rc::Rc;

use crate::parser::{
    expressions::{
        BinaryOperationExpression, BinaryOperator, ExpressionType, FunctionCallExpression,
        UnaryOperationExpression, UnaryOperator,
    },
    statements::{BuiltInStatement, StatementType},
};

#[cfg(test)]
mod tests;

pub struct Interpreter<'a> {
    scope: scope::Scope<'a>,
    statements: Vec<StatementType>,
    pos: usize,
}

impl Interpreter<'_> {
    pub fn new(statements: Vec<StatementType>) -> Self {
        Self {
            scope: scope::Scope::new(None),
            statements,
            pos: 0,
        }
    }

    pub fn interpret(&mut self) {
        while self.pos < self.statements.len() {
            let statement = self.statements[self.pos].clone();
            interpret_statement(&mut self.scope, &statement);

            self.pos += 1;
        }
    }
}

fn interpret_statement(
    scope: &mut scope::Scope,
    statement: &StatementType,
) -> Option<Rc<scope::DataType>> {
    match statement {
        StatementType::VariableDeclaration(statement) => {
            let identifier = statement.identifier.clone();
            let value = statement.value.clone();
            let expression = interpret_expression(scope, &value);

            scope.set_variable(identifier, expression);
            return None;
        }
        StatementType::FunctionDeclaration(statement) => {
            let identifier = statement.identifier.clone();
            let arguments = statement.arguments.iter().map(|x| x.name.clone()).collect();
            let statements = statement.body.statements.clone();

            scope.set_variable(
                identifier,
                Rc::new(scope::DataType::Function(
                    function::FunctionDeclaration::new(arguments, statements),
                )),
            );

            return None;
        }
        StatementType::Return(expression) => {
            return Some(interpret_expression(scope, expression));
        }
        StatementType::FunctionCall(statement) => {
            return execute_function(scope, statement);
        }
        StatementType::BuiltIn(statement) => match statement {
            BuiltInStatement::Print(print_statement) => {
                println!(
                    "{:?}",
                    interpret_expression(scope, &print_statement.argument)
                );

                return None;
            }
        },
        StatementType::IfStatement(statement) => {
            let condition_result = interpret_expression(scope, &statement.condition);

            match *condition_result {
                scope::DataType::Boolean(should_execute) => {
                    if !should_execute {
                        return None;
                    }

                    let mut block_scope = scope::Scope::new(Some(scope));
                    let return_value = execute_statements(
                        &mut block_scope,
                        statement.body.statements.iter().collect(),
                    );

                    return return_value;
                }
                _ => panic!(
                    "Condition '{:?}' of if statement does not result in a boolean",
                    &statement.condition
                ),
            }
        }
        StatementType::VariableAssignment(statement) => {
            scope.update_variable(
                statement.identifier.clone(),
                interpret_expression(scope, &statement.value),
            );
            return None;
        }
    }
}
fn interpret_binary_expression(
    scope: &scope::Scope,
    expression: &BinaryOperationExpression,
) -> scope::DataType {
    let left = interpret_expression(scope, &expression.left);
    let right = interpret_expression(scope, &expression.right);

    return match left.as_ref() {
        scope::DataType::Number(l) => match right.as_ref() {
            scope::DataType::Number(r) => match expression.operator {
                BinaryOperator::Add => scope::DataType::Number(l + r),
                BinaryOperator::Subtract => scope::DataType::Number(l - r),
                BinaryOperator::Divide => scope::DataType::Number(l / r),
                BinaryOperator::Multiply => scope::DataType::Number(l * r),
                BinaryOperator::Equal => scope::DataType::Boolean(l == r),
                BinaryOperator::NotEqual => scope::DataType::Boolean(l != r),
                BinaryOperator::GreaterThan => scope::DataType::Boolean(l > r),
                BinaryOperator::LessThan => scope::DataType::Boolean(l < r),
                BinaryOperator::GreaterOrEqual => scope::DataType::Boolean(l >= r),
                BinaryOperator::LessOrEqual => scope::DataType::Boolean(l <= r),
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
        scope::DataType::String(l) => match right.as_ref() {
            scope::DataType::String(r) => {
                return match expression.operator {
                    BinaryOperator::Add => scope::DataType::String(format!("{}{}", l, r)),
                    BinaryOperator::Equal => scope::DataType::Boolean(l == r),
                    BinaryOperator::NotEqual => scope::DataType::Boolean(l != r),
                    BinaryOperator::GreaterThan => scope::DataType::Boolean(l > r),
                    BinaryOperator::LessThan => scope::DataType::Boolean(l < r),
                    BinaryOperator::GreaterOrEqual => scope::DataType::Boolean(l >= r),
                    BinaryOperator::LessOrEqual => scope::DataType::Boolean(l <= r),
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
        scope::DataType::Boolean(left_value) => match right.as_ref() {
            scope::DataType::Boolean(right_value) => {
                let l = *left_value;
                let r = *right_value;
                match expression.operator {
                    BinaryOperator::Equal => scope::DataType::Boolean(l == r),
                    BinaryOperator::NotEqual => scope::DataType::Boolean(l != r),
                    BinaryOperator::And => scope::DataType::Boolean(l && r),
                    BinaryOperator::Or => scope::DataType::Boolean(l || r),
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

fn execute_function(
    scope: &scope::Scope,
    statement: &FunctionCallExpression,
) -> Option<Rc<scope::DataType>> {
    let function_declaration = if let Some(x) = scope.get_variable(&statement.name) {
        match x.as_ref() {
            scope::DataType::Function(function_declaration) => function_declaration.clone(),
            _ => panic!("Identifier '{}' is not callable", &statement.name),
        }
    } else {
        panic!("Identifier '{}' not found", &statement.name)
    };

    return function_declaration.execute(statement, scope);
}

fn interpret_expression(scope: &scope::Scope, expression: &ExpressionType) -> Rc<scope::DataType> {
    match expression {
        ExpressionType::Literal(literal_type) => {
            return match literal_type {
                crate::parser::expressions::LiteralType::String(x) => {
                    Rc::new(scope::DataType::String(x.clone()))
                }
                crate::parser::expressions::LiteralType::Number(x) => {
                    Rc::new(scope::DataType::Number(x.clone()))
                }
                crate::parser::expressions::LiteralType::Boolean(x) => {
                    Rc::new(scope::DataType::Boolean(x.clone()))
                }
            };
        }
        ExpressionType::Identifier(identifier_expression) => {
            if let Some(var) = scope.get_variable(&identifier_expression.name) {
                return var;
            } else {
                panic!(
                    "Variable for identifier {} not found",
                    identifier_expression.name
                )
            }
        }
        ExpressionType::FunctionCall(function_call_expression) => {
            if let Some(return_value) = execute_function(scope, function_call_expression) {
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

            return Rc::new(scope::DataType::Function(
                function::FunctionDeclaration::new(arguments, statements),
            ));
        }
        ExpressionType::BinaryOperation(binary_operation_expression) => Rc::new(
            interpret_binary_expression(scope, binary_operation_expression),
        ),
        ExpressionType::UnaryOperation(unary_operation_expression) => Rc::new(
            interpret_unary_expression(scope, unary_operation_expression),
        ),
    }
}

fn execute_statements(
    scope: &mut scope::Scope,
    statements: Vec<&StatementType>,
) -> Option<Rc<scope::DataType>> {
    let mut return_value: Option<Rc<scope::DataType>> = None;
    let mut executed_statements: Vec<StatementType> = vec![];
    for x in statements {
        let statement_result = interpret_statement(scope, x);
        executed_statements.push(x.clone());

        if let Some(value) = statement_result {
            return_value = Some(value);
            break;
        }
    }

    return return_value;
}

fn interpret_unary_expression(
    scope: &scope::Scope,
    expression: &UnaryOperationExpression,
) -> scope::DataType {
    let value = interpret_expression(scope, &expression.expression);

    match *value {
        scope::DataType::Number(x) => {
            if expression.operator == UnaryOperator::Minus {
                scope::DataType::Number(-x);
            }

            panic!(
                "Unary operator '{:?}' not supported for number",
                expression.operator
            );
        }
        scope::DataType::Boolean(x) => {
            if expression.operator == UnaryOperator::Bang {
                scope::DataType::Boolean(!x);
            }

            panic!(
                "Unary operator '{:?}' not supported for number",
                expression.operator
            )
        }
        _ => panic!("Unsupported expression type for unary processing"),
    }
}
