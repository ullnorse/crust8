use thiserror::Error;

#[derive(Debug, Error)]
pub enum EmulatorError {
    #[error("Unknown opcode: {0:#X}")]
    UnknownOpcode(u16),
    #[error("Stack overflow")]
    StackOverflow,
    #[error("Stack underflow")]
    StackUnderflow,
}