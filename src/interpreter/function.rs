use std::rc::Rc;

use crate::{
    interpreter::scope::DataType,
    parser::{expressions::FunctionCallExpression, statements::StatementType},
};

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    arguments: Vec<String>,
    body: Vec<StatementType>,
}

impl FunctionDeclaration {
    pub fn new(arguments: Vec<String>, body: Vec<StatementType>) -> Self {
        Self { body, arguments }
    }

    pub fn execute(
        &self,
        call_info: &FunctionCallExpression,
        scope: &super::scope::Scope,
    ) -> Option<Rc<DataType>> {
        let expected_arguments = self.arguments.len();
        let received_arguments = call_info.arguments.len();

        if expected_arguments != received_arguments {
            panic!(
                "Argument count for function '{}' is invalid. Expect: {}, received {}",
                &call_info.name, expected_arguments, received_arguments
            );
        }

        let resolved_arguments: Vec<Rc<DataType>> = call_info
            .arguments
            .iter()
            .map(|x| super::interpret_expression(scope, x))
            .collect();

        let mut function_scope = super::scope::Scope::new(Some(scope));

        // Set arguments as available variables
        for (identifier, value) in self.arguments.iter().zip(resolved_arguments) {
            function_scope.set_variable(identifier.clone(), value);
        }

        let return_value =
            super::execute_statements(&mut function_scope, self.body.iter().collect());

        // Remove argument variables afer scope ended
        for argument in self.arguments.iter() {
            function_scope.remove_variable(argument);
        }

        return return_value;
    }
}
