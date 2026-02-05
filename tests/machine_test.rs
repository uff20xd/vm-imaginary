use vm_imaginary::{Vm, Instruction::*};

fn main() -> Result<(), ()> {
    let mut vm = Vm::new();
    let instructions = vec![
        PushName("x".to_owned()), PushPrim(-8), Let
    ];
    vm.exec(instructions);

    dbg!(vm);
    Ok(())
}
