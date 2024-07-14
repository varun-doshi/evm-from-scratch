use crate::error;
use primitive_types::U256;

#[derive(Clone)]
pub struct Stack {
    data: Vec<U256>,
}

impl Stack {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
    pub fn push(&mut self, value: U256) {
        self.data.push(value)
    }
    pub fn pop(&mut self) {
        self.data.pop();
    }
    pub fn size(&mut self) -> usize {
        self.data.len()
    }

    pub fn set(&mut self, value: U256, i: usize) -> Result<(), error::EvmError> {
        let size = self.size();
        if size > i {
            self.data[size - i - 1] = value;
            Ok(())
        } else {
            Err(error::EvmError::StackUnderflow)
        }
    }

    pub fn data(&self) -> Vec<U256> {
        self.data.to_vec().into_iter().rev().collect()
    }
}
