mod simple;
use std::collections::HashMap;
use std::sync::Arc;
type Byte = u8;

use simple::StackMachine;
type SInstruction = simple::Instruction;

fn main() -> Result<(), ()> {
    let program = vec![ SInstruction::Push(10), SInstruction::Push(20), SInstruction::Add];
    let mut stack_machine = StackMachine::new(program);

    stack_machine.exec();
    dbg!(&stack_machine);
    Ok(())
}


#[derive(Debug, Clone)]
struct Frame {
    vars: HashMap<(String, BufferType), BufferPointer>,
}

impl Frame {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Buffer {
    buf: Vec<u8>,
    empty: Vec<BufferPointer>,
}

impl Buffer {

}

#[derive(Debug, Clone, Default)]
struct BufferPointer {
    buf_type: BufferType,
    index: usize,
    pointee_type: Arc<Type>,
}

#[derive(Debug, Clone, Default)]
enum BufferType {
    Local,
    Constant,
    #[default]
    Global,
}

#[derive(Debug, Clone, Default)]
struct Field {
    name: Option<String>,
    type_of_field: Arc<Type>,
    offset: usize,
}

#[derive(Debug, Clone, Default)]
struct Type {
    size_in_bytes: usize,
    fields: Vec<Field>,
    self_alias: bool,
}

struct Function {
    name: String,
    parameters: Vec<(String, Arc<Type>)>,
    returns: Arc<Type>,
    frame: Frame,
    instructions: Vec<Instruction>,
}

struct Stack<T> {
    stack: Vec<T>,
}

impl Stack<Byte> {
    pub fn pop_into<T: Copy>(&mut self) -> T {
        let size: usize = std::mem::size_of::<T>();
        let len = self.stack.len();
        // SAFETY: We ensure that the stack is long enough and the slice is of the right size.
        let returnee: T = unsafe {
            *(self.stack[len-size..len].as_ptr() as *const T)
        };
        self.stack.truncate(len-size);
        returnee
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Add,
    Sub,
    PushPrim(i32),
    PushType,
    PushPtr,
    PushName,
    Set,
    Get,
    Let,
    Static,
}

#[derive(Clone)]
struct Program {
    types: Vec<Arc<Type>>,
    instructions: Vec<Instruction>,
    instruction_pointer: usize,
}

struct Vm {
    primary_stack: Stack<Byte>,
    ptr_stack: Stack<BufferPointer>,
    type_stack: Stack<Type>,
    name_stack: Stack<String>,
    global_space: Buffer,
    constant_space: Buffer,
    local_space: Buffer,
    vars: Frame,
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
                Instruction::Add => {
                    let rhs: i32 = self.primary_stack.pop_into();
                    let lhs: i32 = self.primary_stack.pop_into();
                    let ret: i32 = rhs + lhs;
                    let test = i32::to_be_bytes(ret);
                    // self.stack.push()
                },
                Instruction::Sub => {},
                Instruction::PushPrim(num) => {},
                Instruction::Get => {},
                Instruction::Set => {},
                Instruction::Let => {todo!()},
                Instruction::Static => {},
                _ => { todo!("Implement other operations")}
            };
        }
    }
    pub async fn exec_one() -> () {todo!()}
    pub fn get_global_vars() -> HashMap<String, ()> { todo!()}
}
