use crate::interpreter::{
    datatype::{Callable, DataType},
    dictionary::DictionaryDeclaration,
    list::ListDeclaration,
};

pub fn expect_string(data: &DataType) -> String {
    match data {
        DataType::Number(x) => x.to_string(),
        DataType::String(x) => x.clone(),
        DataType::Boolean(x) => x.to_string(),
        _ => panic!("Expected a string"),
    }
}

pub fn expect_callable(data: &DataType) -> &Callable {
    if let DataType::Function(callable) = data {
        return callable;
    }

    panic!("Can only use user defined functions")
}

pub fn expect_dict(data: &DataType) -> &DictionaryDeclaration {
    match data {
        DataType::Dictionary(x) => x,
        _ => panic!("Expected a dictionary"),
    }
}

pub fn expect_list(data: &DataType) -> &ListDeclaration {
    match data {
        DataType::List(x) => x,
        _ => panic!("Expected a list"),
    }
}

pub fn expect_int(data: &DataType) -> usize {
    if let DataType::Number(number) = data {
        let i = number.round() as usize;
        if *number as usize != i {
            panic!("Number should be an integer. Received: '{}'", number);
        }

        return *number as usize;
    }

    panic!("Not an integer number");
}
