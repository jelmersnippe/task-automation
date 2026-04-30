use std::rc::Rc;

use crate::{
    interpreter::{
        builtin::{CallInfo, ExecutionError},
        coerce::{self, Args, DataKind},
        datatype::DataType,
    },
    RuntimeContext,
};

pub(crate) fn clear(
    receiver: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("clear", &data);
    args.exact(0)?;

    let x = receiver.expect("clear can only be called on a list");

    let list_receiver = coerce::expect_list(x.as_ref());

    match list_receiver {
        Ok(list) => {
            list.clear();
            Ok(Rc::new(DataType::Undefined))
        }
        Err(err) => Err(ExecutionError::new(
            CallInfo::new("clear"),
            format!(
                "Expected to be called on {:?}, instead found: {:?}",
                DataKind::List,
                err
            )
            .as_str(),
        )),
    }
}

pub(crate) fn push(
    receiver: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("push", &data);
    args.exact(1)?;
    let data = args.any(0)?;

    let x = receiver.expect("push can only be called on a list");

    let list_receiver = coerce::expect_list(x.as_ref());

    match list_receiver {
        Ok(list) => {
            list.push(data.clone());
            Ok(Rc::new(DataType::Undefined))
        }
        Err(err) => Err(ExecutionError::new(
            CallInfo::new("push"),
            format!(
                "Expected to be called on {:?}, instead found: {:?}",
                DataKind::List,
                err
            )
            .as_str(),
        )),
    }
}

pub(crate) fn pop(
    receiver: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("clear", &data);
    args.exact(0)?;

    let x = receiver.expect("pop can only be called on a list");

    let list_receiver = coerce::expect_list(x.as_ref());

    match list_receiver {
        Ok(list) => {
            let result = list.pop();
            match result {
                Some(data) => Ok(data),
                None => Ok(Rc::new(DataType::Undefined)),
            }
        }
        Err(err) => Err(ExecutionError::new(
            CallInfo::new("pop"),
            format!(
                "Expected to be called on {:?}, instead found: {:?}",
                DataKind::List,
                err
            )
            .as_str(),
        )),
    }
}
