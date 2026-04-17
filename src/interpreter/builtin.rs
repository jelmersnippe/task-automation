use std::{collections::HashMap, rc::Rc, sync::OnceLock};

static BUILTINS: OnceLock<HashMap<&'static str, BuiltinFn>> = OnceLock::new();

pub(crate) fn get_builtins() -> &'static HashMap<&'static str, BuiltinFn> {
    BUILTINS.get_or_init(|| HashMap::from([("print", print as BuiltinFn)]))
}

type BuiltinFn = fn(Vec<Rc<super::scope::DataType>>) -> Option<Rc<super::scope::DataType>>;

fn print(data: Vec<Rc<super::scope::DataType>>) -> Option<Rc<super::scope::DataType>> {
    if data.len() != 1 {
        panic!("print only takes 1 argument. Received: {:?}", data)
    }

    match data[0].as_ref() {
        super::scope::DataType::Number(x) => println!("{}", x),
        super::scope::DataType::String(x) => println!("{:?}", x),
        super::scope::DataType::Boolean(x) => println!("{}", x),
        super::scope::DataType::Function(x) => println!("{}", x),
    }

    return None;
}

pub(crate) fn execute_builtin(
    builtin: &BuiltinFn,
    arguments: Vec<Rc<super::scope::DataType>>,
) -> Option<Rc<super::scope::DataType>> {
    return builtin(arguments);
}
