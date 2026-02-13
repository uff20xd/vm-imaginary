type Name = String;
type Byte = u8;

#[derive(Debug, Clone, Default)]
struct CustomType {
    fields: Vec<CustomType>,
    size: usize,
}

#[derive(Debug, Clone, Default)]
enum Primitives {
    Null
    Unit,
    Usize,
    Raw(u32),
    I64,
    F64,
    CustomType(Arc<CustomType>),
}

#[derive(Debug, Clone, Default)]
enum Instruction {
    Bottom,
    Push(u32),
    Raw,
    Let,
    Get,
    Deref,
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
    scope: vec<scope>,
}

#[derive(Debug, Clone, Default)]
enum BufPointer {
    Const(usize),
    Mut(usize),
}

struct Runtime {
    types: Vec<Arc<CustomType>>,
    vars: HashTable<Name, (BufPointer, Primitives>,
}

impl<T> Stack<T> {
    fn push(&mut self, val: T) -> () {
        self.stack.push(val);
    }
    fn pop(&mut self) -> T {
        self.stack.pop()
    }
}

impl Stack<Byte> {
    fn push_bytes(bytes: &[Byte]) -> () { todo!() }
    fn pop_bytes(size: usize) -> Box<[Byte]> { todo!() }
}

impl Buffer {
    fn alloc(bytes: &[Byte]) -> () { todo!() }
    fn write(size: usize) -> Box<[Byte]> { todo!() }
    fn deref(pointer: BufPointer, size: usize, off_set: usize) -> Box<[u8]> { todo!() }
}
