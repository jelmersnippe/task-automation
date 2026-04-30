use crate::{RuntimeContext, interpreter::datatype::DataType};
use std::{rc::Rc, sync::Arc};

pub(crate) mod dictionary;
pub(crate) mod global;
pub(crate) mod list;

pub type BuiltinFn =
    fn(Option<Rc<DataType>>, Vec<Rc<DataType>>, &mut RuntimeContext) -> Rc<DataType>;
pub type Executable =
    Arc<dyn Fn(Option<Rc<DataType>>, Vec<Rc<DataType>>, &mut RuntimeContext) -> Rc<DataType>>;
