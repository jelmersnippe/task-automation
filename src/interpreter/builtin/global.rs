use std::thread::{self};

use crate::{
    RuntimeContext,
    interpreter::{
        builtin::{BuiltinFn, ExecutionError},
        coerce::{Args, ArgumentError, DataKind, expect_callable},
        datatype::{DataType, SharedDataType},
    },
};

pub static BUILTINS: &[(&str, BuiltinFn)] = &[
    ("print", print),
    ("register_task", register_task),
    ("parallel", parallel),
];

fn print(
    _: Option<SharedDataType>,
    data: Vec<SharedDataType>,
    _: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("print", &data);

    args.exact(1)?;
    let arg = args.any(0)?;

    println!("{}", arg);

    Ok((DataType::Undefined).to_shared())
}

fn register_task(
    _: Option<SharedDataType>,
    data: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("register_task", &data);
    args.exact(2)?;
    let task_name = args.string(0)?;
    let task_block = args.callable(1)?;

    context
        .task_registry
        .register(task_name, task_block.clone());

    Ok((DataType::Undefined).to_shared())
}

fn parallel(
    _: Option<SharedDataType>,
    data: Vec<SharedDataType>,
    context: &mut RuntimeContext,
) -> Result<SharedDataType, ExecutionError> {
    let args = Args::new("parallel", &data);
    args.exact(1)?;
    let list = args.list(0)?;

    let locked = list.values.lock().unwrap();
    let callables = locked
        .iter()
        .enumerate()
        .map(|(i, x)| {
            expect_callable(x)
                .map(|x| x.clone())
                .map_err(|e| ArgumentError::InvalidType {
                    fn_name: String::from("parallel"),
                    index: i,
                    expected_type: DataKind::Callable,
                    found_type: e,
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let (sender, receiver) = std::sync::mpsc::channel();

    for (i, callable) in callables.into_iter().enumerate() {
        let mut cloned_context = context.clone();
        let tx = sender.clone();
        thread::spawn(move || {
            let result = callable.execute(vec![], &mut cloned_context);
            tx.send((i, result)).unwrap();
        });
    }
    drop(sender);

    for (i, result) in receiver {
        match result {
            Err(e) => eprintln!(
                "[parallel-{}] task failed: {}",
                i,
                e.to_string().replace('\r', "\n")
            ),
            Ok(_) => {}
        }
    }

    Ok((DataType::Undefined).to_shared())
}
