#[derive(Debug, Clone)]
pub enum VMImagineError {
    StackUnderflow,
    UnimplementedInstructionUsed,
}
