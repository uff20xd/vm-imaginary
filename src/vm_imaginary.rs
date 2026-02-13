use std::sync::Arc;
use std::collections::HashMap;

type Name = String;
type Byte = u8;

#[derive(Debug, Clone)]
struct CustomType {
    fields: Vec<CustomType>,
    size: usize,
}

#[derive(Debug, Clone, Default)]
enum Primitive {
    #[default]
    Null,
    Unit,
    String,
    Usize,
    Raw(u32),
    I64,
    F64,
    CustomType(Arc<CustomType>),
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Null,
    Bottom,
    Push(u32),
    Raw,
    Let,
    Get,
    Deref,
    Eq,
    If,
    Label,
    Jump,
}

#[derive(Debug, Clone, Default)]
struct Stack<T> {
    stack: Vec<T>,
}

#[derive(Debug, Clone, Default)]
struct Buffer {
    buf: Vec<u8>,
    const_space: usize,
    const_counter: usize,
    buf_counter: usize,
    scope: Vec<usize>,
}

#[derive(Debug, Clone)]
enum BufPointer {
    Const(usize),
    Mut(usize),
    String(usize),
    Invalid,
}

#[derive(Debug, Clone)]
struct Function {
    ret: Primitive,
    parameters: Vec<(Name, Primitive)>,
}

#[derive(Debug, Clone, Default)]
struct Runtime {
    types: Vec<Arc<CustomType>>,
    vars: HashMap<Name, (BufPointer, Primitive)>,
    functions: HashMap<Name, Function>,
    labels: HashMap<Name, usize>,
}

#[derive(Debug, Clone, Default)]
pub struct VM {
    stack_bottom: Primitive,
    runtime: Runtime,
    name_stack: Stack<Name>,
    type_stack: Stack<Primitive>,
    primitive_stack: Stack<Byte>,
    buf: Buffer,
    string_space: Vec<Name>,
}

impl<T> Stack<T> {
    fn push(&mut self, val: T) -> () {
        self.stack.push(val);
    }
    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
}

impl Stack<Byte> {
    fn push_bytes(&mut self , bytes: &[Byte]) -> () {
        self.stack.extend_from_slice(bytes)
    }
    fn pop_bytes(&mut self, size: usize) -> Box<[Byte]> {
        let len = self.stack.len();
        if len < size { panic!("Primary Stack: Stackunderflow!") }
        let returnee = Box::from(&self.stack[len-size..len]);
        self.stack.truncate(len-size);
        returnee
    }
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
}

impl Buffer {
    fn alloc(bytes: &[Byte], size: usize) -> () { todo!() }
    fn write(size: usize) -> Box<[Byte]> { todo!() }
    fn deref(pointer: BufPointer, size: usize, off_set: usize) -> Box<[u8]> { todo!() }
    fn add_scope() { todo!() }
    fn pop_scope() { todo!() }
}

impl VM {
    pub fn new() -> Self {todo!()}
}
