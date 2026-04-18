use std::{fmt, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub struct ListDeclaration {
    values: Vec<Rc<super::scope::DataType>>,
}

impl ListDeclaration {
    pub fn new(values: Vec<Rc<super::scope::DataType>>) -> Self {
        Self { values }
    }

    pub fn get(&self, index: f32) -> Rc<super::scope::DataType> {
        let i = index.round();
        if index.round() != index {
            panic!("Index should be an integer. Received: '{}'", index);
        }

        return Rc::clone(
            self.values
                .iter()
                .nth(i as usize)
                .expect("Index out of range"),
        );
    }
}

impl fmt::Display for ListDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;

        for (i, value) in self.values.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", value)?;
        }

        write!(f, "]")
    }
}
