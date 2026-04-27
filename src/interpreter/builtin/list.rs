use std::rc::Rc;

use crate::{
    RuntimeContext,
    interpreter::{coerce, datatype::DataType},
};

pub(crate) fn clear(
    receiver: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &RuntimeContext,
) -> Rc<DataType> {
    if !data.is_empty() {
        panic!(
            "clear takes no arguments. received: {}",
            data.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    let x = receiver.expect("clear can only be called on a list");

    let list = coerce::expect_list(x.as_ref());

    list.clear();

    Rc::new(DataType::Undefined)
}

pub(crate) fn push(
    receiver: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &RuntimeContext,
) -> Rc<DataType> {
    let [data] = data.as_slice() else {
        panic!(
            "delete only takes 1 argument. received: {}",
            data.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    };

    let x = receiver.expect("clear can only be called on a list");

    let list = coerce::expect_list(x.as_ref());

    list.push(data.clone());

    Rc::new(DataType::Undefined)
}

pub(crate) fn pop(
    receiver: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &RuntimeContext,
) -> Rc<DataType> {
    if !data.is_empty() {
        panic!(
            "pop takes no arguments. received: {}",
            data.iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    let x = receiver.expect("clear can only be called on a list");

    let list = coerce::expect_list(x.as_ref());

    if let Some(value) = list.pop() {
        value
    } else {
        Rc::new(DataType::Undefined)
    }
}
