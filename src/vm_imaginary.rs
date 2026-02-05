use std::collections::HashMap;
use std::sync::Arc;
use std::sync::LazyLock;
type Byte = u8;

const FILLER_TYPE: LazyLock<Arc<Type>> = LazyLock::new(|| Arc::new(Type::default()));

#[derive(Debug, Default, Clone)]
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
    local_scope_stack: Stack<usize>,
    vars: Frame,
}

impl Vm {
    pub fn new() -> Self {
        Self {
            local_space: Buffer::new(BufferType::Local { used: 0 }),
            ..Self::default()
        }
    }

    pub fn exec(&mut self, instructions: Vec<Instruction>) {
        let mut instruction_pointer = 0;

        loop {
            println!("Here");
            _ = match &instructions[instruction_pointer] {
                Instruction::Add => {
                    let rhs: i32 = self.primary_stack.pop_into();
                    let lhs: i32 = self.primary_stack.pop_into();
                    let ret: i32 = rhs + lhs;
                    let ret = i32::to_le_bytes(ret);
                    self.primary_stack.push_bytes(&ret[..])
                },
                Instruction::Sub => {
                    let rhs: i32 = self.primary_stack.pop_into();
                    let lhs: i32 = self.primary_stack.pop_into();
                    let ret: i32 = rhs - lhs;
                    let ret = i32::to_le_bytes(ret);
                    self.primary_stack.push_bytes(&ret[..])
                },
                Instruction::PushPrim(num) => {
                    self.primary_stack.push_bytes(&num.to_le_bytes()[..])
                },
                Instruction::PushName(name) => {
                    self.name_stack.push(name.to_owned())
                }
                Instruction::Get => {todo!("Get")},
                Instruction::Set => {todo!("Set")},
                Instruction::Jump => {todo!("Jump")},
                Instruction::If => {todo!("If")},
                Instruction::Eq => {todo!("Eq")},
                Instruction::PrimaryPrint => {
                    let to_debug: i32 = self.primary_stack.pop_into();
                    dbg!(to_debug);
                },
                Instruction::Let => {
                    // let of_type = self.type_stack.pop();
                    let to_push: Box<[u8]>= self.primary_stack.pop_to_slice(4);
                    let name = self.name_stack.pop();
                    let pointer = self.local_space.alloc(&*to_push, 4);
                },
                Instruction::Static => {
                    let name = self.name_stack.pop();
                },
                Instruction::Scope => {
                    let current_used = self.local_space.get_current_used();
                    self.local_scope_stack.push(current_used);
                },
                Instruction::End => {
                    let new_scope = self.local_scope_stack.pop();
                    self.local_space.scope_adjust(new_scope);
                },
                Instruction::Deref => {

                },
                _ => { todo!("Implement other operations")}
            };
            instruction_pointer += 1;
            if instruction_pointer >= instructions.len() { break; }
        }
    }
    pub async fn exec_one() -> () {todo!()}
    pub fn get_global_vars() -> HashMap<String, ()> { todo!()}
}

#[derive(Debug, Clone, Default)]
pub enum Instruction {
    // s s -> s
    Add,
    // s s -> s
    Sub,
    // -> s
    PushPrim(i32),
    // -> t
    PushType,
    // -> p
    PushPtr,
    // -> n
    PushName(String),
    // p s
    Set,
    // n -> p
    Get,
    // s n t -> p
    Let,
    // s n (t) -> p
    Static,
    // s -> p
    Jump,
    // s s -> s
    Eq,
    // s
    If,
    // p -> s
    Deref,
    // ()
    Scope,
    // ()
    End,
    // s
    PrimaryPrint,
    #[default]
    Nil,
}

#[derive(Debug, Clone, Default)]
enum BufferType {
    Local { 
        used: usize,
    },
    Constant,
    Global {

    },
    #[default]
    Nil,
}

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
struct BufferPointer {
    buf_type: BufferType,
    index: usize,
    pointee_type: Arc<Type>,
}

impl BufferPointer {
    pub fn new(index: usize, of_type: BufferType, pointee_type: Arc<Type>) -> Self {
        Self {
            index,
            buf_type: of_type,
            pointee_type,
        }
    }
    pub fn get_size(&self) -> usize {
        self.pointee_type.size_in_bytes
    }
}

#[derive(Debug, Clone, Default)]
struct Buffer {
    buf: Vec<u8>,
    empty: Vec<BufferPointer>,
    buffer_type: BufferType,
}

impl Buffer {
    pub fn new(buf_type: BufferType) -> Self {
        Self {
            buffer_type: buf_type,
            ..Self::default()
        }
    }
    pub fn with_mem(mem: &[u8]) -> Self { todo!("Implement Initializing buffers with preset memory!")}
    pub fn scope_adjust(&mut self, to_index: usize) {
        match &mut self.buffer_type {
            &mut BufferType::Local { mut used } => {
                used = to_index;
            },
            _ => { unreachable!("Yeah No!") }
        }
    }
    pub fn get_current_used(&self) -> usize {
        match &self.buffer_type {
            BufferType::Local { used } => {
                *used
            },
            _ => { unreachable!("Yeah No!") }
        }
    }
    pub fn alloc(&mut self, mem: &[u8], size: usize) -> BufferPointer {
        match &mut self.buffer_type {
            &mut BufferType::Local { mut used } => {
                let pointer = BufferPointer::new(used, BufferType::Local { used: 0 }, FILLER_TYPE.clone());
                if self.buf.len() <= used + size {
                    self.buf.extend_from_slice(&vec![0; used + size * 2][..])
                }
                for (index, element) in mem.iter().enumerate() {
                    self.buf[index + used] = *element;
                }
                used += size;
                pointer
            },
            BufferType::Global {..}=> { todo!() },
            BufferType::Constant => { todo!() },
            _ => { unreachable!("Yeah No!") }
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Field {
    name: Option<String>,
    type_of_field: Arc<Type>,
    offset: usize,
}

#[derive(Debug, Clone, Default)]
enum Primitive {
    Usize,
    Isize,
    #[default]
    I32,
    NewType(Vec<Field>),
}

#[derive(Debug, Clone, Default)]
struct Type {
    name: String,
    size_in_bytes: usize,
    fields: Vec<Field>,
    self_alias: bool,
}

impl Type {
    pub fn new(name: String, size_in_bytes: usize) -> Self {
        Self {
            name,
            size_in_bytes,
            ..Self::default()
        }
    }
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
    pub fn pop(&mut self) -> T {
        self.stack.pop().unwrap_or_else(|| panic!("Namestackunderflow!"))
    }
    pub fn push(&mut self, element: T) {
        self.stack.push(element)
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
    pub fn pop_to_slice(&mut self, size: usize) -> Box<[u8]> {
        let len = self.stack.len();
        if len < size { panic!("Primarystackunderflow!") }
        // SAFETY: We ensure that the stack is long enough and the slice is of the right size.
        let returnee = Box::from(&self.stack[len-size..len]);
        self.stack.truncate(len-size);
        returnee
    }
    pub fn push_bytes(&mut self, bytes: &[Byte]) {
        self.stack.extend_from_slice(bytes);
    }
}
