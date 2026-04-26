use std::{cell::RefCell, fmt, rc::Rc};

use crate::{
    interpreter::{
        StatementResult,
        scope::{DataType, Scope},
    },
    parser::{expressions::Parameters, statements::StatementType},
};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    name: Option<String>,
    arguments: Vec<String>,
    body: Vec<StatementType>,
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
    pub fn new(name: Option<String>, arguments: Vec<String>, body: Vec<StatementType>) -> Self {
        Self {
            name,
            body,
            arguments,
        }
    }

    pub fn execute(&self, parameters: &Parameters, scope: Rc<RefCell<Scope>>) -> Rc<DataType> {
        let expected_arguments = self.arguments.len();
        let received_arguments = parameters.len();

        if expected_arguments != received_arguments {
            panic!(
                "Argument count for function is invalid. Expect: {}, received {}",
                expected_arguments, received_arguments
            );
        }

        let function_scope = Rc::new(RefCell::new(Scope::new(Some(scope.clone()))));

        // Set arguments as available variables
        for (identifier, value) in self.arguments.iter().zip(parameters.resolve(scope)) {
            function_scope
                .borrow_mut()
                .set_variable(identifier.clone(), value);
        }

        let return_value =
            super::execute_statements(function_scope.clone(), self.body.iter().collect());

        match return_value {
            StatementResult::Return(data_type) => data_type,
            _ => Rc::new(DataType::Undefined),
        }
    }
}
