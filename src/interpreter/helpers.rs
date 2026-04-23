use crate::interpreter::list::DictionaryDeclaration;

pub fn expect_string(data: &super::scope::DataType) -> String {
    match data {
        super::scope::DataType::Number(x) => x.to_string(),
        super::scope::DataType::String(x) => x.clone(),
        super::scope::DataType::Boolean(x) => x.to_string(),
        _ => panic!("Expected a string"),
    }
}

pub fn expect_dict(data: &super::scope::DataType) -> &DictionaryDeclaration {
    match data {
        super::scope::DataType::Dictionary(x) => x,
        _ => panic!("Expected a dictionary"),
    }
}

pub fn expect_int(data: &super::scope::DataType) -> usize {
    if let super::scope::DataType::Number(number) = data {
        let i = number.round() as usize;
        if *number as usize != i {
            panic!("Number should be an integer. Received: '{}'", number);
        }

        return *number as usize;
    }

    panic!("Not an integer number");
}
