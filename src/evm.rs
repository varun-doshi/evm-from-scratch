use crate::{error, memory::Memory, opcodes::Opcode, stack::Stack};

pub struct EVM {
    pub stack: Stack,
    pub memory: Memory,
    pub code: Vec<u8>,
}

impl EVM {
    pub fn new(code: Vec<u8>) -> Self {
        Self {
            stack: Stack::new(),
            memory: Memory::new(None),
            code,
        }
    }
    pub fn execute(&mut self) {
        // opcodes::Opcode::execute(&self, self.stack, self.code);
        println!("Executing EVM");
        let mut pc = 0;

        while &pc < &self.code.len() {
            let opcode = self.code[pc];
            println!("Found opcode:{:?}", opcode);

            match opcode {
                Opcode::STOP => Opcode::stop(&mut self.stack, &self.code),
                Opcode::ADD => Opcode::add(&mut self.stack, &self.code),
                Opcode::SUB => Opcode::sub(&mut self.stack, &self.code),
                Opcode::PUSH => Opcode::push(&mut self.stack, &self.code, &mut pc),
                _ => (),
            }
        }
    }
}
