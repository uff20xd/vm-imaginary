use std::collections::BTreeSet;
type Byte = u8;

fn main() -> Result<(), ()> {
    Ok(())
}


struct Frame {

}


struct Runtime {
    frame: Frame,
}

struct Field {
    type_of_field: Type,
    pointer: usize,
}

struct Type {
    size_in_bytes: usize,
    fields: Vec<Field>,
}

struct Stack {
    stack: VecDeque<Byte>,
}

#[derive(Clone, Copy)]
enum Instruction {
    Add,
    Sub,
    Pop,
}

struct Program {
    types: BTreeSet<Type>,
    const_pool: ConstPool,
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
}

struct Vm {
    runtime: Runtime,
    program: Program,
}


impl Vm {
    pub fn new() -> Self {
        todo!()
    }
    pub fn exec(&mut self) -> bool {
        let instructions = &self.program.instructions;

        loop {
            _ = match instructions[self.program.instruction_pointer] {
                Add => {},
                Sub => {},
                Pop => {},
            };
        }
    }
}
