use std::{fmt, rc::Rc};

use crate::{
    interpreter::scope::DataType,
    parser::{
        expressions::{CallExpression, Parameters},
        statements::StatementType,
    },
};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    arguments: Vec<String>,
    body: Vec<StatementType>,
}

impl fmt::Display for FunctionDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn (")?;

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
    pub fn new(arguments: Vec<String>, body: Vec<StatementType>) -> Self {
        Self { body, arguments }
    }

    pub fn execute(
        &self,
        parameters: &Parameters,
        scope: &super::scope::Scope,
    ) -> Option<Rc<DataType>> {
        let expected_arguments = self.arguments.len();
        let received_arguments = parameters.len();

        if expected_arguments != received_arguments {
            panic!(
                "Argument count for function is invalid. Expect: {}, received {}",
                expected_arguments, received_arguments
            );
        }

        let mut function_scope = super::scope::Scope::new(Some(scope));

        // Set arguments as available variables
        for (identifier, value) in self.arguments.iter().zip(parameters.resolve(scope)) {
            function_scope.set_variable(identifier.clone(), value);
        }

        let return_value =
            super::execute_statements(&mut function_scope, self.body.iter().collect());

        return return_value;
    }
}
