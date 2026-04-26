use std::{cell::RefCell, fmt, rc::Rc};

use crate::interpreter::{coerce, scope::DataType};

#[derive(Debug, PartialEq, Clone)]
pub struct ListDeclaration {
    values: Rc<RefCell<Vec<Rc<DataType>>>>,
}

impl ListDeclaration {
    pub fn new(values: Vec<Rc<DataType>>) -> Self {
        Self {
            values: Rc::new(RefCell::new(values)),
        }
    }

    pub fn get(&self, index: Rc<DataType>) -> Rc<DataType> {
        let i = coerce::expect_int(&index);

        if i >= self.values.borrow().len() {
            panic!("Index out of bounds");
        }

        return Rc::clone(
            self.values
                .borrow()
                .iter()
                .nth(i)
                .expect("Index out of range"),
        );
    }

    pub fn set(&self, index: Rc<DataType>, value: Rc<DataType>) {
        let i = coerce::expect_int(&index);

        if i >= self.values.borrow().len() {
            panic!("Index out of bounds");
        }

        self.values.borrow_mut()[i] = value
    }

    pub fn length(&self) -> Rc<DataType> {
        Rc::new(DataType::Number(self.values.borrow().len() as f32))
    }

    pub fn clear(&self) {
        self.values.borrow_mut().clear();
    }

    pub fn pop(&self) -> Option<Rc<DataType>> {
        self.values.borrow_mut().pop()
    }

    pub fn push(&self, value: Rc<DataType>) {
        self.values.borrow_mut().push(value);
    }
}

impl fmt::Display for ListDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        for (i, value) in self.values.borrow().iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", value)?;
        }

        write!(f, "]")
    }
}
