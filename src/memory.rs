use crate::error;

pub const WORD_BYTES: usize = 32;
pub struct Memory {
    pub data: Vec<u8>,
}

impl Memory {
    pub fn new(data: Option<Vec<u8>>) -> Self {
        Self {
            data: if data.is_some() {
                data.unwrap()
            } else {
                Vec::new()
            },
        }
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.to_vec().into_iter().collect()
    }

    pub fn extend(&mut self, size: usize) {
        self.data.extend(vec![0; size])
    }

    pub fn read(
        &mut self,
        address: usize,
        target_length: usize,
    ) -> Result<Vec<u8>, error::EvmError> {
        if address + target_length > self.data.len() {
            let nearest_multiple = if address % 32 == 0 {
                address + 32
            } else {
                (address + 32) + (32 - (address + 32) % 32)
            };

            // Extend memory heap
            self.extend(nearest_multiple - self.data.len());
        }
        let data = self.data[address..(address + target_length)].to_vec();
        Ok(data)
    }

    pub fn mload(&mut self, address: usize) -> Result<[u8; 32], error::EvmError> {
        if address + 32 > self.data.len() {
            // Calculate the nearest multiple of 32
            let nearest_multiple = if address % 32 == 0 {
                address + 32
            } else {
                (address + 32) + (32 - (address + 32) % 32)
            };

            // Extend memory heap
            self.extend(nearest_multiple - self.data.len());
        }
        let mut result = [0u8; 32];
        result.copy_from_slice(&self.data[address..address + 32]);
        Ok(result)
    }

    pub fn mstore(&mut self, value: [u8; 32], address: usize) -> Result<(), error::EvmError> {
        if address + 32 > self.data.len() {
            // Calculate the nearest multiple of 32
            let nearest_multiple = if address % 32 == 0 {
                address + 32
            } else {
                (address + 32) + (32 - (address + 32) % 32)
            };

            // Extend memory heap
            self.extend(nearest_multiple - self.data.len());
        }
        // Copy data into the heap
        self.data[address..address + 32].copy_from_slice(&value);

        Ok(())
    }
}
