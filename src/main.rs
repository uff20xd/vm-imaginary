mod vm_imaginary;
// mod simple;
// use simple::StackMachine;
use vm_imaginary::{VM, Instruction::*};

// type SInstruction = simple::Instruction;

fn main() -> Result<(), ()> {
    let vm = VM::new();
    let instructions = vec![
        Push(100)
    ];
    Ok(())
}
