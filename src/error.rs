#[derive(Debug, Clone)]
pub enum EvmError {
    StackOverflow,
    StackUnderflow,
    InvalidOpcode,
    OutOfGas,
    ExecutionError,
}
