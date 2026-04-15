use crate::core::traits::Function;

pub struct Piecewise {
    functions: Vec<Box<dyn Function>>,
}

impl Piecewise {
    pub fn count(&self) -> usize {
        self.functions.len()
    }

    pub fn functions(&self) -> &[Box<dyn Function>] {
        &self.functions
    }
}