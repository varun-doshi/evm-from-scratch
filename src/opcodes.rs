use std::{ops::Add, os::windows::process};

use primitive_types::U256;

use crate::{error, stack::Stack};

pub struct Opcode;
pub struct OpcodeValue {
    pub opcode: u8,
    pub gas: u8,
}

impl Opcode {
    pub const STOP: u8 = 00;
    pub const ADD: u8 = 01;
    pub const MUL: u8 = 02;
    pub const SUB: u8 = 03;
    pub const DIV: u8 = 04;
    pub const SDIV: u8 = 05;
    pub const MOD: u8 = 06;
    pub const SMOD: u8 = 07;
    pub const ADDMOD: u8 = 08;
    pub const PUSH: u8 = 60;

    pub fn stop(stack: &mut Stack, code: &Vec<u8>) {
        println!("STOP")
    }

    pub fn add(stack: &mut Stack, code: &Vec<u8>, pc: &mut usize, gas_used: &mut u64) {
        let value1 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let value2 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let sum = value1 + value2;
        stack.push(sum);
        *gas_used += 3;
        *pc = *pc + 1;
    }

    pub fn sub(stack: &mut Stack, code: &Vec<u8>, pc: &mut usize, gas_used: &mut u64) {
        let value1 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let value2 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let difference = value1 - value2;
        stack.push(difference);
        *gas_used += 3;
        *pc = *pc + 1;
    }
    pub fn mul(stack: &mut Stack, code: &Vec<u8>, pc: &mut usize, gas_used: &mut u64) {
        let value1 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let value2 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let product = value1 * value2;
        stack.push(product);
        *gas_used += 5;
        *pc = *pc + 1;
    }
    pub fn div(stack: &mut Stack, code: &Vec<u8>, pc: &mut usize, gas_used: &mut u64) {
        let value1 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let value2 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });

        let remainder: U256;
        if value2 == U256::from(0) {
            remainder = U256::from(0);
        } else {
            remainder = value1 / value2;
        }
        stack.push(remainder);
        *gas_used += 5;
        *pc = *pc + 1;
    }

    // @note : implement SDIV
    pub fn sdiv(stack: &mut Stack, code: &Vec<u8>, pc: &mut usize, gas_used: &mut u64) {
        let value1 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let value2 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let remainder = value1 / value2;
        stack.push(remainder);
        *gas_used += 5;
        *pc = *pc + 1;
    }
    pub fn modulus(stack: &mut Stack, code: &Vec<u8>, pc: &mut usize, gas_used: &mut u64) {
        let value1 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let value2 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let mod_result: U256;
        if value2 == U256::from(0) {
            mod_result = U256::from(0);
        } else {
            mod_result = value1 % value2;
        }
        stack.push(mod_result);
        *gas_used += 5;
        *pc = *pc + 1;
    }
    // @note : implement SMOD
    pub fn smodulus(stack: &mut Stack, code: &Vec<u8>, pc: &mut usize, gas_used: &mut u64) {
        let value1 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let value2 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let mod_result: U256;
        if value2 == U256::from(0) {
            mod_result = U256::from(0);
        } else {
            mod_result = value1 % value2;
        }
        stack.push(mod_result);
        *gas_used += 5;
        *pc = *pc + 1;
    }
    pub fn addmodulus(stack: &mut Stack, code: &Vec<u8>, pc: &mut usize, gas_used: &mut u64) {
        let value1 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let value2 = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let denominator = stack.pop().unwrap_or_else(|| {
            println!("Stack underflow");
            std::process::exit(1);
        });
        let mod_result: U256;
        if denominator == U256::from(0) {
            mod_result = U256::from(0);
        } else {
            mod_result = (value1 + value2) % denominator;
        }
        stack.push(mod_result);
        *gas_used += 8;
        *pc = *pc + 1;
    }
    pub fn push(stack: &mut Stack, code: &Vec<u8>, pc: &mut usize) {
        println!("Found push");
        stack.push(code[*pc + 1].into());
        *pc = *pc + 2;
        println!("{:?}", stack.data());
    }
}
