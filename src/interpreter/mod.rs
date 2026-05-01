pub(crate) mod builtin;
pub(crate) mod coerce;
pub(crate) mod datatype;
pub(crate) mod dictionary;
pub(crate) mod function;
pub(crate) mod list;
pub(crate) mod scope;

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    RuntimeContext,
    interpreter::{
        builtin::{CallInfo, ExecutionError, global::BUILTINS},
        coerce::Args,
        datatype::{Callable, DataType, SharedDataType},
        dictionary::DictionaryDeclaration,
        function::FunctionDeclaration,
        list::ListDeclaration,
        scope::{Scope, SharedScope},
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

pub struct Interpreter {
    pub(crate) scope: SharedScope,
    statements: Vec<StatementType>,
    pos: usize,
}

impl Interpreter {
    pub fn new(
        statements: Vec<StatementType>,
        context: &mut RuntimeContext,
    ) -> Result<Self, ExecutionError> {
        let mut scope = Scope::new(None);

        for (k, v) in BUILTINS {
            scope.set_variable(
                k.to_string(),
                (DataType::Function(Callable::new(Some(k.to_string()), *v))).to_shared(),
            )?;
        }

        println!("Hello!");
        for module in &context.module_registry.modules {
            println!("Adding module {} ", &module.name);
            scope.set_variable(
                module.name.clone(),
                (DataType::Module(module.clone())).to_shared(),
            )?;
        }

        Ok(Self {
            scope: Arc::new(Mutex::new(scope)),
            statements,
            pos: 0,
        })
    }

    pub fn interpret(&mut self, context: &mut RuntimeContext) -> Result<(), ExecutionError> {
        while self.pos < self.statements.len() {
            let statement = self.statements[self.pos].clone();
            interpret_statement(self.scope.clone(), &statement, context)?;

            self.pos += 1;
        }

        Ok(())
    }
}

pub enum StatementResult {
    Void,
    Break,
    Continue,
    Return(SharedDataType),
}

fn interpret_statement(
    scope: SharedScope,
    statement: &StatementType,
    context: &mut RuntimeContext,
) -> Result<StatementResult, ExecutionError> {
    match statement {
        StatementType::VariableDeclaration(statement) => {
            let identifier = statement.identifier.clone();
            let value = statement.value.clone();
            let expression = interpret_expression(scope.clone(), &value, context)?;

            scope.lock().unwrap().set_variable(identifier, expression)?;
            Ok(StatementResult::Void)
        }
        StatementType::FunctionDeclaration(statement) => {
            let identifier = statement.identifier.clone();
            let arguments = statement.arguments.iter().map(|x| x.name.clone()).collect();
            let statements = statement.body.statements.clone();

            scope.lock().unwrap().set_variable(
                identifier.clone(),
                (DataType::Function(
                    FunctionDeclaration::new(
                        Some(identifier.clone()),
                        arguments,
                        statements,
                        scope.clone(),
                    )
                    .into_callable(),
                ))
                .to_shared(),
            )?;

            Ok(StatementResult::Void)
        }
        StatementType::Return(expression) => Ok(StatementResult::Return(interpret_expression(
            scope.clone(),
            expression,
            context,
        )?)),
        StatementType::IfStatement(statement) => {
            let condition_result =
                interpret_expression(scope.clone(), &statement.condition, context)?;

            match condition_result.as_ref() {
                // TODO: is_truthy helper instead of strict boolean check
                DataType::Boolean(should_execute) => {
                    if !should_execute {
                        return Ok(StatementResult::Void);
                    }

                    let block_scope = Arc::new(Mutex::new(Scope::new(Some(scope.clone()))));
                    execute_statements(
                        block_scope,
                        statement.body.statements.iter().collect(),
                        context,
                    )
                }
                _ => Err(ExecutionError::new(
                    CallInfo::new(""),
                    &format!(
                        "Condition '{:?}' of if statement does not result in a boolean",
                        &statement.condition
                    ),
                )),
            }
        }
        StatementType::Expression(statement) => match statement {
            ExpressionStatement::Assignment(assignment_statement) => {
                interpret_assignment(scope.clone(), assignment_statement, context)?;
                Ok(StatementResult::Void)
            }
            ExpressionStatement::Inline(expression_type) => {
                interpret_expression(scope, expression_type, context)?;
                Ok(StatementResult::Void)
            }
        },
        StatementType::While(statement) => {
            loop {
                let condition_result =
                    interpret_expression(scope.clone(), &statement.condition, context)?;

                match *condition_result {
                    DataType::Boolean(should_execute) => {
                        if !should_execute {
                            break;
                        }

                        let block_scope = Arc::new(Mutex::new(Scope::new(Some(scope.clone()))));
                        let return_value = execute_statements(
                            block_scope,
                            statement.body.statements.iter().collect(),
                            context,
                        )?;

                        match return_value {
                            StatementResult::Return(_) => {
                                return Ok(return_value);
                            }
                            StatementResult::Break => break,
                            _ => {}
                        }
                    }
                    _ => {
                        return Err(ExecutionError::new(
                            CallInfo::new(""),
                            &format!(
                                "Condition '{:?}' of if statement does not result in a boolean",
                                &statement.condition
                            ),
                        ));
                    }
                }
            }

            Ok(StatementResult::Void)
        }
        StatementType::Break => Ok(StatementResult::Break),
        StatementType::Continue => Ok(StatementResult::Continue),
    }
}

fn interpret_assignment(
    scope: SharedScope,
    assignment: &AssignmentStatement,
    context: &mut RuntimeContext,
) -> Result<(), ExecutionError> {
    let value = interpret_expression(scope.clone(), &assignment.value, context)?;
    match &assignment.identifier {
        ExpressionType::Identifier(identifier_expression) => scope
            .lock()
            .unwrap()
            .update_variable(&identifier_expression.name, value),
        ExpressionType::Accessor(accessor_expression) => {
            let storage = interpret_expression(scope.clone(), &accessor_expression.value, context)?;

            match storage.as_ref() {
                DataType::List(list) => {
                    let key =
                        interpret_expression(scope.clone(), &accessor_expression.key, context)?;
                    list.set(key, value)
                }
                DataType::Dictionary(dict) => {
                    let key =
                        interpret_expression(scope.clone(), &accessor_expression.key, context)?;
                    dict.set(key, value)
                }
                _ => Err(ExecutionError::new(
                    CallInfo::new(""),
                    &format!("Invalid use of accessor"),
                )),
            }
        }
        _ => Err(ExecutionError::new(
            CallInfo::new(""),
            &format!("Expression is not assignable"),
        )),
    }
}

fn interpret_binary_expression(
    scope: SharedScope,
    expression: &BinaryOperationExpression,
    context: &mut RuntimeContext,
) -> Result<DataType, ExecutionError> {
    let left = interpret_expression(scope.clone(), &expression.left, context)?;
    let right = interpret_expression(scope.clone(), &expression.right, context)?;

    let result = match left.as_ref() {
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
                _ => {
                    return Err(ExecutionError::new(
                        CallInfo::new(""),
                        &format!(
                            "Invalid operation for number binary operation: {} {:?} {}",
                            l, expression.operator, r
                        ),
                    ));
                }
            },
            DataType::String(r) => {
                return match expression.operator {
                    BinaryOperator::Add => Ok(DataType::String(format!("{}{}", l, r))),
                    _ => {
                        return Err(ExecutionError::new(
                            CallInfo::new(""),
                            &format!(
                                "Invalid operation for number and string binary operation: {} {:?} {}",
                                l, expression.operator, r,
                            ),
                        ));
                    }
                };
            }
            _ => {
                return Err(ExecutionError::new(
                    CallInfo::new(""),
                    &format!(
                        "Left and right types of binary expression '{:?}' don't match",
                        expression
                    ),
                ));
            }
        },
        DataType::String(l) => match right.as_ref() {
            DataType::String(r) => match expression.operator {
                BinaryOperator::Add => DataType::String(format!("{}{}", l, r)),
                BinaryOperator::Equal => DataType::Boolean(l == r),
                BinaryOperator::NotEqual => DataType::Boolean(l != r),
                BinaryOperator::GreaterThan => DataType::Boolean(l > r),
                BinaryOperator::LessThan => DataType::Boolean(l < r),
                BinaryOperator::GreaterOrEqual => DataType::Boolean(l >= r),
                BinaryOperator::LessOrEqual => DataType::Boolean(l <= r),

                _ => {
                    return Err(ExecutionError::new(
                        CallInfo::new(""),
                        &format!(
                            "Invalid operation for string binary operation: {} {:?} {}",
                            &l, expression.operator, &r
                        ),
                    ));
                }
            },
            DataType::Number(r) => match expression.operator {
                BinaryOperator::Add => DataType::String(format!("{}{}", l, r)),
                _ => {
                    return Err(ExecutionError::new(
                        CallInfo::new(""),
                        &format!(
                            "Invalid operation for number and string binary operation: {} {:?} {}",
                            l, expression.operator, r
                        ),
                    ));
                }
            },
            _ => {
                return Err(ExecutionError::new(
                    CallInfo::new(""),
                    &format!(
                        "Left and right types of binary expression '{:?}' don't match",
                        expression
                    ),
                ));
            }
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
                    _ => {
                        return Err(ExecutionError::new(
                            CallInfo::new(""),
                            &format!(
                                "Invalid operation for boolean binary operation: {} {:?} {}",
                                l, expression.operator, r,
                            ),
                        ));
                    }
                }
            }
            _ => {
                return Err(ExecutionError::new(
                    CallInfo::new(""),
                    &format!(
                        "Left and right types of binary expression '{:?}' don't match",
                        expression
                    ),
                ));
            }
        },
        _ => {
            return Err(ExecutionError::new(
                CallInfo::new(""),
                "Invalid DataType used for binary expression",
            ));
        }
    };

    Ok(result)
}

fn execute_function(
    scope: SharedScope,
    statement: &CallExpression,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let value = interpret_expression(scope.clone(), &statement.value, context)?;

    if let DataType::Function(callable) = value.as_ref() {
        let parameters = statement
            .parameters
            .iter()
            .map(|x| interpret_expression(scope.clone(), x, context))
            .collect::<Result<Vec<_>, _>>()?;
        callable.execute(parameters, context)
    } else {
        Err(ExecutionError::new(
            CallInfo::new(""),
            "Expression is not callable",
        ))
    }
}

pub fn interpret_expression(
    scope: SharedScope,
    expression: &ExpressionType,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    match expression {
        ExpressionType::Literal(literal_type) => Ok(match literal_type {
            LiteralType::String(x) => (DataType::String(x.clone())).to_shared(),
            LiteralType::Number(x) => (DataType::Number(x.clone())).to_shared(),
            LiteralType::Boolean(x) => (DataType::Boolean(x.clone())).to_shared(),
            LiteralType::Undefined => (DataType::Undefined).to_shared(),
        }),
        ExpressionType::Identifier(identifier_expression) => scope
            .lock()
            .unwrap()
            .get_variable(&identifier_expression.name),
        ExpressionType::FunctionCall(function_call_expression) => {
            execute_function(scope, function_call_expression, context)
        }
        ExpressionType::FunctionDeclaration(function_declaration_expression) => {
            let arguments = function_declaration_expression
                .parameters
                .iter()
                .map(|x| x.name.clone())
                .collect();
            let statements = function_declaration_expression.body.statements.clone();

            Ok((DataType::Function(
                FunctionDeclaration::new(None, arguments, statements, scope.clone())
                    .into_callable(),
            ))
            .to_shared())
        }
        ExpressionType::BinaryOperation(binary_operation_expression) => Ok(
            interpret_binary_expression(scope.clone(), binary_operation_expression, context)?
                .to_shared(),
        ),
        ExpressionType::UnaryOperation(unary_operation_expression) => Ok(
            interpret_unary_expression(scope.clone(), unary_operation_expression, context)?
                .to_shared(),
        ),
        ExpressionType::List(list_expression) => Ok((DataType::List(interpret_list_expression(
            scope.clone(),
            list_expression,
            context,
        )?))
        .to_shared()),
        ExpressionType::Dictionary(dictionary_expression) => Ok((DataType::Dictionary(
            interpret_dictionary_expression(scope.clone(), dictionary_expression, context)?,
        ))
        .to_shared()),
        ExpressionType::Accessor(accessor_expression) => {
            let value = interpret_expression(scope.clone(), &accessor_expression.value, context)?;

            match value.as_ref() {
                DataType::List(list) => {
                    let key =
                        interpret_expression(scope.clone(), &accessor_expression.key, context)?;

                    list.get(key)
                }
                DataType::Dictionary(dict) => {
                    let key =
                        interpret_expression(scope.clone(), &accessor_expression.key, context)?;

                    let args = Args::new("get", &vec![key]);
                    Ok(dict.get(&args.string(0)?))
                }
                _ => Err(ExecutionError::new(
                    CallInfo::new(""),
                    &format!("Can't use accessor on {}", value),
                )),
            }
        }
        ExpressionType::Property(property_expression) => {
            let value = interpret_expression(scope, &property_expression.value, context)?;
            let property = property_expression.key.name.clone();

            value.get_method(&property)
        }
    }
}

fn interpret_dictionary_expression(
    scope: SharedScope,
    dictionary_expression: &crate::parser::expressions::DictionaryExpression,
    context: &mut RuntimeContext,
) -> Result<DictionaryDeclaration, ExecutionError> {
    let mut keys: Vec<String> = vec![];

    for key in dictionary_expression.keys.iter() {
        let resolved_key = interpret_expression(scope.clone(), &key, context)?;

        match resolved_key.as_ref() {
            DataType::Number(x) => keys.push(x.to_string()),
            DataType::String(x) => keys.push(x.clone()),
            DataType::Boolean(x) => keys.push(x.to_string()),
            _ => {
                return Err(ExecutionError::new(
                    CallInfo::new(""),
                    "Can only use literals or functions returning literals as dictionary keys",
                ));
            }
        }
    }

    let values: Vec<SharedDataType> = dictionary_expression
        .values
        .iter()
        .map(|x| interpret_expression(scope.clone(), x, context))
        .collect::<Result<Vec<_>, _>>()?;

    let entries: Vec<(String, SharedDataType)> = keys.into_iter().zip(values).collect();

    let mut map = HashMap::new();

    for (key, value) in entries {
        map.insert(key, value);
    }

    Ok(DictionaryDeclaration::new(map))
}

fn interpret_list_expression(
    scope: SharedScope,
    list_expression: &ListExpression,
    context: &mut RuntimeContext,
) -> Result<ListDeclaration, ExecutionError> {
    let values = list_expression
        .values
        .iter()
        .map(|x| interpret_expression(scope.clone(), x, context))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ListDeclaration::new(values))
}

fn execute_statements(
    scope: SharedScope,
    statements: Vec<&StatementType>,
    context: &mut RuntimeContext,
) -> Result<StatementResult, ExecutionError> {
    for x in statements {
        let statement_result = interpret_statement(scope.clone(), x, context)?;

        match statement_result {
            StatementResult::Return(_) => {
                return Ok(statement_result);
            }
            StatementResult::Continue => return Ok(StatementResult::Continue),
            StatementResult::Break => return Ok(StatementResult::Break),
            _ => {}
        }
    }

    Ok(StatementResult::Void)
}

fn interpret_unary_expression(
    scope: SharedScope,
    expression: &UnaryOperationExpression,
    context: &mut RuntimeContext,
) -> Result<DataType, ExecutionError> {
    let value = interpret_expression(scope, &expression.expression, context)?;

    match *value {
        DataType::Number(x) => {
            if expression.operator == UnaryOperator::Minus {
                return Ok(DataType::Number(-x));
            }

            Err(ExecutionError::new(
                CallInfo::new(""),
                format!(
                    "Unary operator '{:?}' not supported for number",
                    expression.operator
                )
                .as_str(),
            ))
        }
        DataType::Boolean(x) => {
            if expression.operator == UnaryOperator::Bang {
                return Ok(DataType::Boolean(!x));
            }

            Err(ExecutionError::new(
                CallInfo::new(""),
                format!(
                    "Unary operator '{:?}' not supported for boolean",
                    expression.operator
                )
                .as_str(),
            ))
        }
        _ => Err(ExecutionError::new(
            CallInfo::new(""),
            "Unsupported expression type for unary processing",
        )),
    }
}
