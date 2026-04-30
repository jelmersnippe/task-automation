use std::{cell::RefCell, fmt, rc::Rc, sync::Arc};

use crate::{
    interpreter::{
        builtin::{CallInfo, ExecutionError},
        datatype::{Callable, DataType},
        execute_statements,
        scope::Scope,
        StatementResult,
    },
    parser::statements::StatementType,
    RuntimeContext,
};

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    name: Option<String>,
    arguments: Vec<String>,
    body: Vec<StatementType>,
    scope: Rc<RefCell<Scope>>,
}

impl PartialEq for FunctionDeclaration {
    fn eq(&self, other: &Self) -> bool {
        // scope intentionally ignored due to circular deps
        self.name == other.name && self.arguments == other.arguments && self.body == other.body
    }
}

impl fmt::Display for FunctionDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn ")?;

        if let Some(name) = &self.name {
            write!(f, "{}", name)?;
        }

        write!(f, "(")?;

        for (i, value) in self.arguments.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", value)?;
        }

        write!(f, ") {{ }}")
    }
}

impl FunctionDeclaration {
    pub fn new(
        name: Option<String>,
        arguments: Vec<String>,
        body: Vec<StatementType>,
        scope: Rc<RefCell<Scope>>,
    ) -> Self {
        Self {
            name,
            body,
            arguments,
            scope,
        }
    }

    pub fn execute(
        &self,
        parameters: Vec<Rc<DataType>>,
        context: &mut RuntimeContext,
    ) -> Result<Rc<DataType>, ExecutionError> {
        let expected_arguments = self.arguments.len();
        let received_arguments = parameters.len();

        if expected_arguments != received_arguments {
            panic!(
                "Argument count for function is invalid. Expect: {}, received {}",
                expected_arguments, received_arguments
            );
        }

        let function_scope = Rc::new(RefCell::new(Scope::new(Some(self.scope.clone()))));

        // Set arguments as available variables
        for (identifier, value) in self.arguments.iter().zip(parameters) {
            function_scope
                .borrow_mut()
                .set_variable(identifier.clone(), value)?;
        }

        let return_value =
            execute_statements(function_scope.clone(), self.body.iter().collect(), context)?;

        match return_value {
            StatementResult::Return(data_type) => Ok(data_type),
            StatementResult::Void => Ok(Rc::new(DataType::Undefined)),
            // Break and Continue are disallowed in Parser. This is just safety
            StatementResult::Break => Err(ExecutionError::new(
                CallInfo::new(if let Some(name) = &self.name {
                    &name
                } else {
                    ""
                }),
                "Break is not supported in function body",
            )),
            StatementResult::Continue => Err(ExecutionError::new(
                CallInfo::new(if let Some(name) = &self.name {
                    &name
                } else {
                    ""
                }),
                "Continue is not supported in function body",
            )),
        }
    }

    pub fn into_callable(self) -> Callable {
        Callable::from_executable(
            self.name.clone(),
            Arc::new(move |_receiver, args, context| self.execute(args, context)),
        )
    }
}
