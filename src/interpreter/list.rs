use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub struct ListDeclaration {
    values: Rc<RefCell<Vec<Rc<super::scope::DataType>>>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct DictionaryDeclaration {
    entries: Rc<RefCell<HashMap<String, Rc<super::scope::DataType>>>>,
}

impl DictionaryDeclaration {
    pub fn new(entries: HashMap<String, Rc<super::scope::DataType>>) -> Self {
        Self {
            entries: Rc::new(RefCell::new(entries)),
        }
    }
}

impl fmt::Display for DictionaryDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;

        for (i, (key, value)) in self.entries.borrow().iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", key, value)?;
        }

        write!(f, "}}")
    }
}

impl ListDeclaration {
    pub fn new(values: Vec<Rc<super::scope::DataType>>) -> Self {
        Self {
            values: Rc::new(RefCell::new(values)),
        }
    }

    pub fn get(&self, index: f32) -> Rc<super::scope::DataType> {
        let i = index.round() as usize;
        if index.round() != index {
            panic!("Index should be an integer. Received: '{}'", index);
        }

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

    pub fn set(&self, index: f32, value: Rc<super::scope::DataType>) {
        let i = index.round() as usize;
        if index.round() != index {
            panic!("Index should be an integer. Received: '{}'", index);
        }

        if i >= self.values.borrow().len() {
            panic!("Index out of bounds");
        }

        self.values.borrow_mut()[i] = value
    }

    pub fn length(&self) -> Rc<super::scope::DataType> {
        Rc::new(super::scope::DataType::Number(
            self.values.borrow().len() as f32
        ))
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
