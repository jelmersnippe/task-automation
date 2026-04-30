use std::rc::Rc;

use crate::{
    RuntimeContext,
    interpreter::{
        builtin::{CallInfo, ExecutionError},
        coerce::{self, Args, DataKind},
        datatype::DataType,
    },
};

pub(crate) fn has(
    receiver: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("has", &data);
    args.exact(1)?;

    let x = receiver.expect("has can only be called on a dictionary");

    let key = args.string(0)?;
    let receiver_dict = coerce::expect_dict(x.as_ref());

    match receiver_dict {
        Ok(dict) => Ok(Rc::new(DataType::Boolean(dict.has(&key)))),
        Err(err) => Err(ExecutionError::new(
            CallInfo::new("has"),
            format!(
                "Expected to be called on {:?}, instead found: {:?}",
                DataKind::Dictionary,
                err
            )
            .as_str(),
        )),
    }
}

pub(crate) fn delete(
    receiver: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("delete", &data);
    args.exact(1)?;

    let x = receiver.expect("has can only be called on a dictionary");

    let key = args.string(0)?;
    let receiver_dict = coerce::expect_dict(x.as_ref());

    match receiver_dict {
        Ok(dict) => {
            dict.delete(&key);
            Ok(Rc::new(DataType::Undefined))
        }
        Err(err) => Err(ExecutionError::new(
            CallInfo::new("delete"),
            format!(
                "Expected to be called on {:?}, instead found: {:?}",
                DataKind::Dictionary,
                err
            )
            .as_str(),
        )),
    }
}

pub(crate) fn clear(
    receiver: Option<Rc<DataType>>,
    data: Vec<Rc<DataType>>,
    _: &mut RuntimeContext,
) -> Result<Rc<DataType>, ExecutionError> {
    let args = Args::new("clear", &data);
    args.exact(0)?;

    let x = receiver.expect("has can only be called on a dictionary");

    let receiver_dict = coerce::expect_dict(x.as_ref());

    match receiver_dict {
        Ok(dict) => {
            dict.clear();
            Ok(Rc::new(DataType::Undefined))
        }
        Err(err) => Err(ExecutionError::new(
            CallInfo::new("clear"),
            format!(
                "Expected to be called on {:?}, instead found: {:?}",
                DataKind::Dictionary,
                err
            )
            .as_str(),
        )),
    }
}
