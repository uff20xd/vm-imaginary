use std::collections::BTreeSet;
use std::collections::VecDeque;
type Byte = u8;

fn main() -> Result<(), ()> {
    Ok(())
}


struct Frame {
    locals: BTreeSet<&str, >,
}

struct Runtime {
    frame: Frame,
}

struct Field {
    type_of_field: Type,
    offset: usize,
}

struct Type {
    size_in_bytes: usize,
    fields: Vec<Field>,
    self_alias: bool,
}

struct Function {
    name: String,
    parameters: Vec<(String, Type)>,
    returns: Type,
    frame: Frame,
    instructions: Vec<Instruction>,
}

struct Variable {
    type: Type
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
    // const_pool: ConstPool,
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
                Instruction::Add => {},
                Instruction::Sub => {},
                Instruction::Pop => {},
            };
        }
    }
}
