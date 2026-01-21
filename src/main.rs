mod simple;
use std::collections::BTreeMap;
use std::collections::VecDeque;
use std::rc::Rc;
use std::marker::PhantomData;
type Byte = u8;

fn main() -> Result<(), ()> {
    Ok(())
}


struct Frame {
    locals: BTreeMap<String, Value>,
}

struct Runtime {
    frame: Frame,
    const_pool: (),
}

#[derive(Clone)]
struct Field {
    type_of_field: Type,
    offset: usize,
}

#[derive(Clone)]
struct Type {
    size_in_bytes: usize,
    fields: Vec<Field>,
    self_alias: bool,
}

struct Function {
    name: String,
    parameters: Vec<(String, Type)>,
    returns: ConstPoolPtr<Type>,
    frame: Frame,
    instructions: Vec<Instruction>,
}

#[derive(Clone)]
struct Value {
    val: Rc<[u8]>,
    is_of_type: ConstPoolPtr<Type>,
}

struct Variable {
    name: String,

}

struct Stack {
    stack: VecDeque<Byte>,
}

#[derive(Clone, Copy)]
enum Instruction {
    Add,
    Sub,
    Const(),
}

/// Points to a constant inside the Pool
#[derive(Clone)]
struct ConstPoolPtr<T> {
    index: usize,
    const_pool: Rc<ConstPool>,
    _phantom_data: PhantomData<T>,
}

#[derive(Clone)]
struct Program {
    types: Vec<Type>,
    // const_pool: ConstPool,
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
}

struct Vm {
    runtime: Runtime,
    program: Program,
}

struct ConstPool {
    types: Vec<Type>,
    constants: Vec<Value>,
    string: Vec<String>,
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
                Instruction::Const() => {},
            };
        }
    }
}
