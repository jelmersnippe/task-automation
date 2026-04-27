use std::rc::Rc;

use crate::{
    RuntimeContext,
    interpreter::{coerce, datatype::DataType},
};

pub(crate) fn has(
    receiver: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &RuntimeContext,
) -> Rc<DataType> {
    let [key] = data.as_slice() else {
        panic!(
            "has only takes 1 argument. received: {:?}",
            data.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };

    let x = receiver.expect("has can only be called on a dictionary");

    let arg = coerce::expect_string(key);
    let dict = coerce::expect_dict(x.as_ref());

    Rc::new(DataType::Boolean(dict.has(&arg)))
}

pub(crate) fn delete(
    receiver: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &RuntimeContext,
) -> Rc<DataType> {
    let [key] = data.as_slice() else {
        panic!(
            "delete only takes 1 argument. received: {:?}",
            data.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };

    let x = receiver.expect("delete can only be called on a dictionary");

    let arg = coerce::expect_string(key);
    let dict = coerce::expect_dict(x.as_ref());

    dict.delete(&arg);

    Rc::new(DataType::Undefined)
}

pub(crate) fn clear(
    receiver: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &RuntimeContext,
) -> Rc<DataType> {
    if !data.is_empty() {
        panic!(
            "clear takes no arguments. received: {:?}",
            data.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    let x = receiver.expect("clear can only be called on a dictionary");

    let dict = coerce::expect_dict(x.as_ref());

    dict.clear();

    Rc::new(DataType::Undefined)
}
