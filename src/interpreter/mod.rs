pub(crate) mod builtin;
pub(crate) mod coerce;
pub(crate) mod dictionary;
pub(crate) mod function;
pub(crate) mod list;
pub(crate) mod scope;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    RuntimeContext,
    interpreter::{
        builtin::{Builtin, global::BUILTINS},
        dictionary::DictionaryDeclaration,
        function::FunctionDeclaration,
        list::ListDeclaration,
        scope::{Callable, DataType, Scope},
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
    scope: Rc<RefCell<Scope>>,
    statements: Vec<StatementType>,
    pos: usize,
}

impl Interpreter {
    pub fn new(statements: Vec<StatementType>) -> Self {
        let mut scope = Scope::new(None);

        for (k, v) in BUILTINS {
            scope.set_variable(
                k.to_string(),
                Rc::new(DataType::Function(Callable::BuiltIn(Builtin::new(
                    k,
                    v.clone(),
                )))),
            );
        }

        Self {
            scope: Rc::new(RefCell::new(scope)),
            statements,
            pos: 0,
        }
    }

    pub fn interpret(&mut self, context: &RuntimeContext) {
        while self.pos < self.statements.len() {
            let statement = self.statements[self.pos].clone();
            interpret_statement(self.scope.clone(), &statement, context);

            self.pos += 1;
        }
    }
}

pub enum StatementResult {
    Void,
    Break,
    Continue,
    Return(Rc<DataType>),
}

fn interpret_statement(
    scope: Rc<RefCell<Scope>>,
    statement: &StatementType,
    context: &RuntimeContext,
) -> StatementResult {
    match statement {
        StatementType::VariableDeclaration(statement) => {
            let identifier = statement.identifier.clone();
            let value = statement.value.clone();
            let expression = interpret_expression(scope.clone(), &value, context);

            scope.borrow_mut().set_variable(identifier, expression);
            StatementResult::Void
        }
        StatementType::FunctionDeclaration(statement) => {
            let identifier = statement.identifier.clone();
            let arguments = statement.arguments.iter().map(|x| x.name.clone()).collect();
            let statements = statement.body.statements.clone();

            scope.borrow_mut().set_variable(
                identifier.clone(),
                Rc::new(DataType::Function(Callable::User(
                    FunctionDeclaration::new(
                        Some(identifier.clone()),
                        arguments,
                        statements,
                        scope.clone(),
                    ),
                ))),
            );

            StatementResult::Void
        }
        StatementType::Return(expression) => {
            StatementResult::Return(interpret_expression(scope.clone(), expression, context))
        }
        StatementType::IfStatement(statement) => {
            let condition_result =
                interpret_expression(scope.clone(), &statement.condition, context);

            match *condition_result {
                // TODO: is_truthy helper instead of strict boolean check
                DataType::Boolean(should_execute) => {
                    if !should_execute {
                        return StatementResult::Void;
                    }

                    let block_scope = Rc::new(RefCell::new(Scope::new(Some(scope.clone()))));
                    let return_value = execute_statements(
                        block_scope,
                        statement.body.statements.iter().collect(),
                        context,
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
                interpret_assignment(scope.clone(), assignment_statement, context);
                StatementResult::Void
            }
            ExpressionStatement::Inline(expression_type) => {
                interpret_expression(scope, expression_type, context);
                StatementResult::Void
            }
        },
        StatementType::While(statement) => {
            loop {
                let condition_result =
                    interpret_expression(scope.clone(), &statement.condition, context);

                match *condition_result {
                    DataType::Boolean(should_execute) => {
                        if !should_execute {
                            return StatementResult::Void;
                        }

                        let block_scope = Rc::new(RefCell::new(Scope::new(Some(scope.clone()))));
                        let return_value = execute_statements(
                            block_scope,
                            statement.body.statements.iter().collect(),
                            context,
                        );

                        match return_value {
                            StatementResult::Return(_) => {
                                return return_value;
                            }
                            StatementResult::Break => break,
                            _ => {}
                        }
                    }
                    _ => panic!(
                        "Condition '{:?}' of if statement does not result in a boolean",
                        &statement.condition
                    ),
                }
            }

            StatementResult::Void
        }
        StatementType::Break => StatementResult::Break,
        StatementType::Continue => StatementResult::Continue,
    }
}

fn interpret_assignment(
    scope: Rc<RefCell<Scope>>,
    assignment: &AssignmentStatement,
    context: &RuntimeContext,
) {
    let value = interpret_expression(scope.clone(), &assignment.value, context);
    match &assignment.identifier {
        ExpressionType::Identifier(identifier_expression) => {
            scope
                .borrow_mut()
                .update_variable(&identifier_expression.name, value);
        }
        ExpressionType::Accessor(accessor_expression) => {
            let storage = interpret_expression(scope.clone(), &accessor_expression.value, context);

            match storage.as_ref() {
                DataType::List(list) => {
                    let key =
                        interpret_expression(scope.clone(), &accessor_expression.key, context);
                    list.set(key, value);
                }
                DataType::Dictionary(dict) => {
                    let key =
                        interpret_expression(scope.clone(), &accessor_expression.key, context);
                    dict.set(key, value);
                }
                _ => panic!("Invalid use of accessor"),
            };
        }
        _ => panic!("Expression is not assignable"),
    }
}

fn interpret_binary_expression(
    scope: Rc<RefCell<Scope>>,
    expression: &BinaryOperationExpression,
    context: &RuntimeContext,
) -> DataType {
    let left = interpret_expression(scope.clone(), &expression.left, context);
    let right = interpret_expression(scope.clone(), &expression.right, context);

    match left.as_ref() {
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
            DataType::String(r) => {
                return match expression.operator {
                    BinaryOperator::Add => DataType::String(format!("{}{}", l, r)),
                    _ => panic!(
                        "Invalid operation for number and string binary operation: {} {:?} {}",
                        &l, expression.operator, &r
                    ),
                };
            }
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
            DataType::Number(r) => {
                return match expression.operator {
                    BinaryOperator::Add => DataType::String(format!("{}{}", l, r)),
                    _ => panic!(
                        "Invalid operation for string and number binary operation: {} {:?} {}",
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
    }
}

fn execute_function(
    scope: Rc<RefCell<Scope>>,
    statement: &CallExpression,
    context: &RuntimeContext,
) -> Rc<DataType> {
    let value = interpret_expression(scope.clone(), &statement.value, context);

    if let DataType::Function(function_declaration) = value.as_ref() {
        function_declaration.execute(
            &Parameters::new(statement.parameters.clone()),
            scope.clone(),
            context,
        )
    } else {
        panic!("Expression is not callable");
    }
}

pub fn interpret_expression(
    scope: Rc<RefCell<Scope>>,
    expression: &ExpressionType,
    context: &RuntimeContext,
) -> Rc<DataType> {
    match expression {
        ExpressionType::Literal(literal_type) => match literal_type {
            LiteralType::String(x) => Rc::new(DataType::String(x.clone())),
            LiteralType::Number(x) => Rc::new(DataType::Number(x.clone())),
            LiteralType::Boolean(x) => Rc::new(DataType::Boolean(x.clone())),
            LiteralType::Undefined => Rc::new(DataType::Undefined),
        },
        ExpressionType::Identifier(identifier_expression) => {
            scope.borrow().get_variable(&identifier_expression.name)
        }
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

            Rc::new(DataType::Function(Callable::User(
                FunctionDeclaration::new(None, arguments, statements, scope.clone()),
            )))
        }
        ExpressionType::BinaryOperation(binary_operation_expression) => Rc::new(
            interpret_binary_expression(scope.clone(), binary_operation_expression, context),
        ),
        ExpressionType::UnaryOperation(unary_operation_expression) => Rc::new(
            interpret_unary_expression(scope.clone(), unary_operation_expression, context),
        ),
        ExpressionType::List(list_expression) => Rc::new(DataType::List(
            interpret_list_expression(scope.clone(), list_expression, context),
        )),
        ExpressionType::Dictionary(dictionary_expression) => Rc::new(DataType::Dictionary(
            interpret_dictionary_expression(scope.clone(), dictionary_expression, context),
        )),
        ExpressionType::Accessor(accessor_expression) => {
            let value = interpret_expression(scope.clone(), &accessor_expression.value, context);

            match value.as_ref() {
                DataType::List(list) => {
                    let key =
                        interpret_expression(scope.clone(), &accessor_expression.key, context);

                    list.get(key)
                }
                DataType::Dictionary(dict) => {
                    let key =
                        interpret_expression(scope.clone(), &accessor_expression.key, context);

                    dict.get(&coerce::expect_string(&key))
                }
                _ => panic!("Can't use accessor on {}", value),
            }
        }
        ExpressionType::Property(property_expression) => {
            let value = interpret_expression(scope, &property_expression.value, context);
            let property = property_expression.key.name.clone();

            value.get_method(&property)
        }
    }
}

fn interpret_dictionary_expression(
    scope: Rc<RefCell<Scope>>,
    dictionary_expression: &crate::parser::expressions::DictionaryExpression,
    context: &RuntimeContext,
) -> DictionaryDeclaration {
    let mut keys: Vec<String> = vec![];

    for key in dictionary_expression.keys.iter() {
        let resolved_key = interpret_expression(scope.clone(), &key, context);

        match resolved_key.as_ref() {
            DataType::Number(x) => keys.push(x.to_string()),
            DataType::String(x) => keys.push(x.clone()),
            DataType::Boolean(x) => keys.push(x.to_string()),
            _ => panic!("Can only use literals or functions returning literals as dictionary keys"),
        }
    }

    let values: Vec<Rc<DataType>> = dictionary_expression
        .values
        .iter()
        .map(|x| interpret_expression(scope.clone(), x, context))
        .collect();

    let entries: Vec<(String, Rc<DataType>)> = keys.into_iter().zip(values).collect();

    let mut map = HashMap::new();

    for (key, value) in entries {
        map.insert(key, value);
    }

    DictionaryDeclaration::new(map)
}

fn interpret_list_expression(
    scope: Rc<RefCell<Scope>>,
    list_expression: &ListExpression,
    context: &RuntimeContext,
) -> ListDeclaration {
    let values = list_expression
        .values
        .iter()
        .map(|x| interpret_expression(scope.clone(), x, context))
        .collect();

    ListDeclaration::new(values)
}

fn execute_statements(
    scope: Rc<RefCell<Scope>>,
    statements: Vec<&StatementType>,
    context: &RuntimeContext,
) -> StatementResult {
    let mut executed_statements: Vec<StatementType> = vec![];
    for x in statements {
        let statement_result = interpret_statement(scope.clone(), x, context);
        executed_statements.push(x.clone());

        match statement_result {
            StatementResult::Return(_) => {
                return statement_result;
            }
            StatementResult::Continue => return StatementResult::Continue,
            StatementResult::Break => return StatementResult::Break,
            _ => {}
        }
    }

    StatementResult::Void
}

fn interpret_unary_expression(
    scope: Rc<RefCell<Scope>>,
    expression: &UnaryOperationExpression,
    context: &RuntimeContext,
) -> DataType {
    let value = interpret_expression(scope, &expression.expression, context);

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

#[derive(PartialEq, Debug, Clone)]
pub struct Parameters {
    values: Vec<ExpressionType>,
}

impl Parameters {
    pub fn new(values: Vec<ExpressionType>) -> Self {
        Self { values }
    }

    pub fn resolve(
        &self,
        scope: Rc<RefCell<Scope>>,
        context: &RuntimeContext,
    ) -> Vec<Rc<DataType>> {
        return self
            .values
            .iter()
            .map(|x| interpret_expression(scope.clone(), x, context))
            .collect();
    }

    pub fn len(&self) -> usize {
        return self.values.len();
    }
}
