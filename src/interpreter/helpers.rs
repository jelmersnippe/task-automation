pub fn expect_string(data: &super::scope::DataType) -> String {
    match data {
        super::scope::DataType::Number(x) => x.to_string(),
        super::scope::DataType::String(x) => x.clone(),
        super::scope::DataType::Boolean(x) => x.to_string(),
        _ => panic!("Only literals or functions returning literals can be converted to string"),
    }
}
