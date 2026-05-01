use crate::{
    interpreter::{
        builtin::{CallInfo, ExecutionError},
        coerce::{self, Args, DataKind},
        datatype::{DataType, SharedDataType},
    },
    RuntimeContext,
};

pub(crate) fn has(
    receiver: Option<SharedDataType>,
    data: Vec<SharedDataType>,
    _: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("has", &data);
    args.exact(1)?;

    let x = receiver.expect("has can only be called on a dictionary");

    let key = args.string(0)?;
    let receiver_dict = coerce::expect_dict(x.as_ref());

    match receiver_dict {
        Ok(dict) => Ok((DataType::Boolean(dict.has(&key))).to_shared()),
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
    receiver: Option<SharedDataType>,
    data: Vec<SharedDataType>,
    _: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("delete", &data);
    args.exact(1)?;

    let x = receiver.expect("has can only be called on a dictionary");

    let key = args.string(0)?;
    let receiver_dict = coerce::expect_dict(x.as_ref());

    match receiver_dict {
        Ok(dict) => {
            dict.delete(&key);
            Ok((DataType::Undefined).to_shared())
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
    receiver: Option<SharedDataType>,
    data: Vec<SharedDataType>,
    _: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("clear", &data);
    args.exact(0)?;

    let x = receiver.expect("clear can only be called on a dictionary");

    let receiver_dict = coerce::expect_dict(x.as_ref());

    match receiver_dict {
        Ok(dict) => {
            dict.clear();
            Ok((DataType::Undefined).to_shared())
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

pub(crate) fn len(
    receiver: Option<SharedDataType>,
    data: Vec<SharedDataType>,
    _: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("len", &data);
    args.exact(0)?;

    let x = receiver.expect("len can only be called on a dictionary");

    let receiver_dict = coerce::expect_dict(x.as_ref());

    match receiver_dict {
        Ok(dict) => Ok(dict.length()),
        Err(err) => Err(ExecutionError::new(
            CallInfo::new("len"),
            format!(
                "Expected to be called on {:?}, instead found: {:?}",
                DataKind::Dictionary,
                err
            )
            .as_str(),
        )),
    }
}
