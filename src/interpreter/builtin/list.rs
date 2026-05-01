use crate::{
    RuntimeContext,
    interpreter::{
        builtin::{CallInfo, ExecutionError},
        coerce::{self, Args, DataKind},
        datatype::{DataType, SharedDataType},
    },
};

pub(crate) fn clear(
    receiver: Option<SharedDataType>,
    data: Vec<SharedDataType>,
    _: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("clear", &data);
    args.exact(0)?;

    let x = receiver.expect("clear can only be called on a list");

    let list_receiver = coerce::expect_list(x.as_ref());

    match list_receiver {
        Ok(list) => {
            list.clear();
            Ok((DataType::Undefined).to_shared())
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
    receiver: Option<SharedDataType>,
    data: Vec<SharedDataType>,
    _: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("push", &data);
    args.exact(1)?;
    let data = args.any(0)?;

    let x = receiver.expect("push can only be called on a list");

    let list_receiver = coerce::expect_list(x.as_ref());

    match list_receiver {
        Ok(list) => {
            list.push(data.clone());
            Ok((DataType::Undefined).to_shared())
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
    receiver: Option<SharedDataType>,
    data: Vec<SharedDataType>,
    _: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("clear", &data);
    args.exact(0)?;

    let x = receiver.expect("pop can only be called on a list");

    let list_receiver = coerce::expect_list(x.as_ref());

    match list_receiver {
        Ok(list) => {
            let result = list.pop();
            match result {
                Some(data) => Ok(data),
                None => Ok((DataType::Undefined).to_shared()),
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

pub(crate) fn len(
    receiver: Option<SharedDataType>,
    data: Vec<SharedDataType>,
    _: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("len", &data);
    args.exact(0)?;

    let x = receiver.expect("len can only be called on a list");

    let list_receiver = coerce::expect_list(x.as_ref());

    match list_receiver {
        Ok(list) => Ok(list.length()),
        Err(err) => Err(ExecutionError::new(
            CallInfo::new("len"),
            format!(
                "Expected to be called on {:?}, instead found: {:?}",
                DataKind::List,
                err
            )
            .as_str(),
        )),
    }
}
