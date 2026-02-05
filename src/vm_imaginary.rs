use std::collections::HashMap;
use std::sync::Arc;
type Byte = u8;


#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone, Default)]
struct Function {
    name: String,
    parameters: Vec<(String, Arc<Type>)>,
    returns: Arc<Type>,
    frame: Frame,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, Default)]
struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
        }
    }
}

impl Stack<Byte> {
    pub fn pop_into<T: Copy>(&mut self) -> T {
        let size: usize = std::mem::size_of::<T>();
        let len = self.stack.len();
        if len < size { panic!("Primarystackunderflow!") }
        // SAFETY: We ensure that the stack is long enough and the slice is of the right size.
        let returnee: T = unsafe {
            *(self.stack[len-size..len].as_ptr() as *const T)
        };
        self.stack.truncate(len-size);
        returnee
    }
    pub fn push(&mut self, bytes: &[Byte]) {
        self.stack.extend_from_slice(bytes);
    }
}

impl Stack<String> {
    pub fn pop(&mut self) -> String {
        self.stack.pop().unwrap_or_else(|| panic!("Namestackunderflow!"))
    }
    pub fn push(&mut self, name: String) {
        self.stack.push(name)
    }
}

#[derive(Debug, Clone, Default)]
pub enum Instruction {
    Add,
    Sub,
    PushPrim(i32),
    PushType,
    PushPtr,
    PushName(String),
    Set,
    Get,
    Let,
    Static,
    Jump,
    Eq,
    If,
    #[default]
    Nil,
}

#[derive(Default, Clone)]
pub struct Vm {
    types: Vec<Arc<Type>>,
    _instructions: Vec<Instruction>,
    _instruction_pointer: usize,

    primary_stack: Stack<Byte>,
    ptr_stack: Stack<BufferPointer>,
    type_stack: Stack<Type>,
    name_stack: Stack<String>,
    global_space: Buffer,
    constant_space: Buffer,
    local_space: Buffer,
    vars: Frame,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            ..Self::default()
        }
    }

    pub fn exec(&mut self, instructions: Vec<Instruction>) -> bool {
        let instruction_pointer = 0;

        loop {
            _ = match &instructions[instruction_pointer] {
                Instruction::Add => {
                    let rhs: i32 = self.primary_stack.pop_into();
                    let lhs: i32 = self.primary_stack.pop_into();
                    let ret: i32 = rhs + lhs;
                    let test = i32::to_be_bytes(ret);
                    // self.stack.push()
                },
                Instruction::Sub => {
                    let rhs: i32 = self.primary_stack.pop_into();
                    let lhs: i32 = self.primary_stack.pop_into();
                    let ret: i32 = rhs + lhs;
                    let test = i32::to_be_bytes(ret);
                },
                Instruction::PushPrim(num) => {
                    self.primary_stack.push(&num.to_be_bytes()[..])
                },
                Instruction::PushName(name) => {
                    self.name_stack.push(name.to_owned())
                }
                Instruction::Get => {todo!("Get")},
                Instruction::Set => {todo!("Set")},
                Instruction::Jump => {todo!("Jump")},
                Instruction::If => {todo!("Jump")},
                Instruction::Eq => {todo!("Eq")},
                Instruction::Let => {todo!("Let")},
                Instruction::Static => {},
                _ => { todo!("Implement other operations")}
            };
        }
    }
    pub async fn exec_one() -> () {todo!()}
    pub fn get_global_vars() -> HashMap<String, ()> { todo!()}
}
