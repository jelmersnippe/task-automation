pub(crate) mod builtin;
pub(crate) mod function;
pub(crate) mod list;
pub(crate) mod scope;

use std::rc::Rc;

use crate::{
    interpreter::{
        builtin::{execute_builtin, get_builtins},
        list::ListDeclaration,
        scope::DataType,
    },
    parser::{
        expressions::{
            BinaryOperationExpression, BinaryOperator, CallExpression, ExpressionType,
            ListExpression, LiteralType, UnaryOperationExpression, UnaryOperator,
        },
        statements::{AssignmentStatement, ExpressionStatement, StatementType},
    },
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

pub enum StatementResult {
    Void(),
    Return(Rc<DataType>),
}

fn interpret_statement(scope: &mut scope::Scope, statement: &StatementType) -> StatementResult {
    match statement {
        StatementType::VariableDeclaration(statement) => {
            let identifier = statement.identifier.clone();
            let value = statement.value.clone();
            let expression = interpret_expression(scope, &value);

            scope.set_variable(identifier, expression);
            StatementResult::Void()
        }
        StatementType::FunctionDeclaration(statement) => {
            let identifier = statement.identifier.clone();
            let arguments = statement.arguments.iter().map(|x| x.name.clone()).collect();
            let statements = statement.body.statements.clone();

            scope.set_variable(
                identifier.clone(),
                Rc::new(scope::DataType::Function(
                    function::FunctionDeclaration::new(
                        Some(identifier.clone()),
                        arguments,
                        statements,
                    ),
                )),
            );

            StatementResult::Void()
        }
        StatementType::Return(expression) => {
            StatementResult::Return(interpret_expression(scope, expression))
        }
        StatementType::IfStatement(statement) => {
            let condition_result = interpret_expression(scope, &statement.condition);

            match *condition_result {
                scope::DataType::Boolean(should_execute) => {
                    if !should_execute {
                        return StatementResult::Void();
                    }

                    let mut block_scope = scope::Scope::new(Some(scope));
                    let return_value = execute_statements(
                        &mut block_scope,
                        statement.body.statements.iter().collect(),
                    );

                    return_value
                }
                _ => panic!(
                    "Condition '{:?}' of if statement does not result in a boolean",
                    &statement.condition
                ),
            }
        }
        StatementType::Expression(statement) => match statement {
            ExpressionStatement::Assignment(assignment_statement) => {
                interpret_assignment(scope, assignment_statement);
                StatementResult::Void()
            }
            ExpressionStatement::Inline(expression_type) => {
                interpret_expression(scope, expression_type);
                StatementResult::Void()
            }
        },
    }
}

fn interpret_assignment(scope: &mut scope::Scope, assignment: &AssignmentStatement) {
    match &assignment.identifier {
        ExpressionType::Identifier(identifier_expression) => {
            scope.update_variable(
                identifier_expression.name.clone(),
                interpret_expression(scope, &assignment.value),
            );
        }
        // Will need a resolver function that runs untill we reach {value: identifier | list, key: x}
        ExpressionType::Accessor(accessor_expression) => {
            let value = interpret_expression(scope, &accessor_expression.value);

            match value.as_ref() {
                DataType::List(list) => {
                    let key = interpret_expression(scope, &accessor_expression.key);

                    match key.as_ref() {
                        DataType::Number(index) => {
                            list.set(*index, interpret_expression(scope, &assignment.value))
                        }
                        _ => panic!("Invalid index for list"),
                    };
                }
                _ => panic!("Invalid use of accessor"),
            };
        }
        ExpressionType::FunctionCall(call_expression) => {
            let value = interpret_expression(scope, &call_expression.value);

            match value.as_ref() {
                DataType::Reference(data) => {
                    scope.update_variable(
                        data.identifier.clone(),
                        interpret_expression(scope, &assignment.value),
                    );
                }
                _ => panic!("{} is not callable", value),
            }
        }
        _ => panic!("Expression is not assignable"),
    }
}

fn interpret_binary_expression(
    scope: &scope::Scope,
    expression: &BinaryOperationExpression,
) -> scope::DataType {
    let left = interpret_expression(scope, &expression.left);
    let right = interpret_expression(scope, &expression.right);

    match left.as_ref() {
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
    }
}

fn execute_function(scope: &scope::Scope, statement: &CallExpression) -> Rc<scope::DataType> {
    // Check if it's a builtin
    match statement.value.as_ref() {
        ExpressionType::Identifier(identifier) => {
            if let Some(var) = get_builtins().get(identifier.name.as_str()) {
                return execute_builtin(var, statement.parameters.resolve(scope));
            }
        }
        _ => {}
    }

    let value = interpret_expression(scope, &statement.value);

    match value.as_ref() {
        scope::DataType::Function(function_declaration) => {
            function_declaration.execute(&statement.parameters, scope)
        }
        _ => panic!("Expression is not callable"),
    }
}

pub fn interpret_expression(
    scope: &scope::Scope,
    expression: &ExpressionType,
) -> Rc<scope::DataType> {
    match expression {
        ExpressionType::Literal(literal_type) => match literal_type {
            LiteralType::String(x) => Rc::new(scope::DataType::String(x.clone())),
            LiteralType::Number(x) => Rc::new(scope::DataType::Number(x.clone())),
            LiteralType::Boolean(x) => Rc::new(scope::DataType::Boolean(x.clone())),
        },
        ExpressionType::Identifier(identifier_expression) => {
            scope.get_variable(&identifier_expression.name)
        }
        ExpressionType::FunctionCall(function_call_expression) => {
            execute_function(scope, function_call_expression)
        }
        ExpressionType::FunctionDeclaration(function_declaration_expression) => {
            let arguments = function_declaration_expression
                .parameters
                .iter()
                .map(|x| x.name.clone())
                .collect();
            let statements = function_declaration_expression.body.statements.clone();

            Rc::new(scope::DataType::Function(
                function::FunctionDeclaration::new(None, arguments, statements),
            ))
        }
        ExpressionType::BinaryOperation(binary_operation_expression) => Rc::new(
            interpret_binary_expression(scope, binary_operation_expression),
        ),
        ExpressionType::UnaryOperation(unary_operation_expression) => Rc::new(
            interpret_unary_expression(scope, unary_operation_expression),
        ),
        ExpressionType::List(list_expression) => Rc::new(scope::DataType::List(
            interpret_list_expression(scope, list_expression),
        )),
        ExpressionType::Accessor(accessor_expression) => {
            let value = interpret_expression(scope, &accessor_expression.value);

            if let DataType::List(list) = value.as_ref() {
                let key = interpret_expression(scope, &accessor_expression.key);

                if let DataType::Number(index) = key.as_ref() {
                    return list.get(*index);
                }
            }

            panic!("Can't use accessor on {}", value);
        }
    }
}

fn interpret_list_expression(
    scope: &scope::Scope,
    list_expression: &ListExpression,
) -> ListDeclaration {
    let values = list_expression
        .values
        .iter()
        .map(|x| interpret_expression(scope, x))
        .collect();

    ListDeclaration::new(values)
}

fn execute_statements(
    scope: &mut scope::Scope,
    statements: Vec<&StatementType>,
) -> StatementResult {
    let mut executed_statements: Vec<StatementType> = vec![];
    for x in statements {
        let statement_result = interpret_statement(scope, x);
        executed_statements.push(x.clone());

        if let StatementResult::Return(_) = statement_result {
            return statement_result;
        }
    }

    StatementResult::Void()
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
