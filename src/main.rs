mod error;
mod evm;
mod memory;
mod opcodes;
mod stack;
use primitive_types::U256;

use memory::Memory;

fn main() {
    let mut stack = stack::Stack::new();
    stack.push(U256::from(23));
    stack.push(U256::from(46));
    stack.push(U256::from(69));
    println!("{:?}", stack.data());
    stack.pop();
    println!("{:?}", stack.data());
    stack.set(U256::from(69), 1).unwrap();
    println!("{:?}", stack.data());
}

#[cfg(test)]
mod tests {
    use std::os::windows::process;

    use evm::EVM;

    use super::*;
    use crate::error::EvmError;

    #[test]
    fn test_hex() {
        let code = hex::decode("6005600c01").unwrap();
        println!("Hex value{:?}", code);
    }

    #[test]
    fn test_mstore() {
        let mut memory = Memory::new(None);

        let data: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ];
        memory.mstore(data, 0).unwrap();

        assert_eq!(memory.data.len(), 32);
        assert_eq!(&memory.data[0..32], &data);
    }

    #[test]
    fn test_mload() {
        let mut memory = Memory::new(None);

        let data: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ];
        memory.mstore(data, 0).unwrap();

        let loaded_data = memory.mload(0).unwrap();
        println!("{:?}", loaded_data);
        assert_eq!(loaded_data, data);
    }

    #[test]
    fn test_read() {
        let mut memory = Memory::new(None);

        let data: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ];
        memory.mstore(data, 0).unwrap();

        let read_data = memory.read(0, 32).unwrap();
        assert_eq!(read_data, data.to_vec());
    }

    #[test]
    fn test_extend_memory() {
        let mut memory = Memory::new(None);

        let data: [u8; 32] = [
            0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d,
            0x0e, 0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b,
            0x1c, 0x1d, 0x1e, 0x1f,
        ];
        memory.mstore(data, 0).unwrap();

        // Store another 32 bytes at address 64 (requires extending memory)
        let new_data: [u8; 32] = [
            0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d,
            0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b,
            0x3c, 0x3d, 0x3e, 0x3f,
        ];
        memory.mstore(new_data, 64).unwrap();

        let loaded_data = memory.read(0, 96).unwrap();
        println!("{:?}", loaded_data);

        assert_eq!(memory.data.len(), 96);
        assert_eq!(&memory.data[64..96], &new_data);
    }

    #[test]
    fn test_evm() {
        let calldata = hex::encode(vec![60, 5, 60, 4, 01]);

        let code = hex::decode(calldata).unwrap();
        println!("code:{:?}", code);
        let mut revm = EVM::new(code);
        println!("Size:{:?}", revm.stack.size());
        evm::EVM::execute(&mut revm);
        println!("Size:{:?}", revm.stack.size());
        println!("Peaked value{:?}", revm.stack.peak());
        println!("Gas used:{}", revm.gas_used);
    }
}
