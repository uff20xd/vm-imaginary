use vm_imaginary::{Vm, Instruction::*};

fn main() -> Result<(), ()> {
    let mut vm = Vm::new();
    let instructions = vec![
        PushName("x".to_owned()), PushPrim(317027), PushPrim(4), Add, PrimaryPrint
    ];
    vm.exec(instructions);

    // dbg!(vm);
    Ok(())
}
