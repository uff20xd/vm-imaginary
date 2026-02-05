mod vm_imaginary;
// mod simple;
// use simple::StackMachine;
use vm_imaginary::{Vm, Instruction::*};

// type SInstruction = simple::Instruction;

fn main() -> Result<(), ()> {
    let vm = Vm::new();

    Ok(())
}
