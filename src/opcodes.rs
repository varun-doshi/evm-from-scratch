use crate::{error, stack::Stack};

pub struct Opcode;

impl Opcode {
    pub const STOP: u8 = 0x00;
    pub const ADD: u8 = 0x01;
    pub const SUB: u8 = 0x02;
    pub const PUSH: u8 = 60;

    pub fn stop(stack: &mut Stack, code: &Vec<u8>) {
        println!("STOP")
    }

    pub fn add(stack: &mut Stack, code: &Vec<u8>) {
        println!("ADD")
    }

    pub fn sub(stack: &mut Stack, code: &Vec<u8>) {
        println!("SUB")
    }
    pub fn push(stack: &mut Stack, code: &Vec<u8>, pc: &mut usize) {
        println!("Found push");
        stack.push(code[*pc + 1].into());
        *pc = *pc + 2;
        println!("{:?}", stack.data());
    }
}
