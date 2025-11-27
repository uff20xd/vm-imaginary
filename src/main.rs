use std::fs;
use std::io::Read;
use std::path::Path;
use std::convert::TryFrom;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vm = VmImaginary::<256>::new();
    let bucket = Bucket::default();
    let mut programm_loader = ProgrammLoader::new("tests/Add.class")?;
    dbg!(&programm_loader);
    dbg!(programm_loader.get_next_bytes(2));
    dbg!(programm_loader.get_next_bytes(2));
    Ok(())
}

type VmError = String;

type Byte = u8;

#[derive(Debug, Clone)]
struct ProgrammLoader {
    bytes: Vec<Byte>,
    counter: usize,
}

impl ProgrammLoader{
    pub fn new<P: AsRef<Path>>(file_name: P) -> Result<Self, std::io::Error> { 
        let mut bytes = Vec::new();
        let mut file = fs::File::open(file_name)?;
        _ = file.read_to_end(&mut bytes);
        Ok(Self{
            bytes,
            counter: 0,
        })
    }
    fn get_bytes(&self, from: usize, to: usize) -> &[Byte] {
        &self.bytes[from..to]
    }

    fn get_next_bytes(&mut self, n:usize) -> &[Byte] {
        let teva = &self.bytes[self.counter..self.counter + n];
        self.counter += n;
        teva
    }
    fn get_next_bytes_as_usize(&mut self, n: usize) -> usize {
        let id_bytes = <[u8; 8]>::try_from(self.get_next_bytes(n)).unwrap();
        let num: usize = usize::from_le_bytes(id_bytes);
        num
    }
    pub fn parse_pool(&mut self) -> ConstPool {
        let mut const_pool = ConstPool::new();
        let mut id_bytes = <[u8; 8]>::try_from(self.get_next_bytes(2)).unwrap();
        let len: usize = usize::from_le_bytes(id_bytes);
        for i in 0..len {
            const_pool.push(self.parse_const());
        }
        const_pool
    }
    pub fn parse_const(&mut self) -> Const {
        let tag = self.get_next_bytes(1)[0];
        match tag {
            0x01 => {Const::String(String::from(self.get_next_bytes(self.get_next_bytes_as_usize(2))))},
            0x07 => {Const::NameIndex(self.get_next_bytes_as_usize(2) as u16)},
            0x08 => {Const::StringIndex(self.get_next_bytes_as_usize(2) as u16)},
            0x09 || 0x0a => {todo!()},
            0x0c => {todo!()},
            _ => {panic!("Fuck you")},
        }
    }
}

struct Programm {}

#[derive(Debug, Clone, Copy, Default)]
struct Bucket {
    bucket: [Byte; 8],
    full: bool,
}

enum Const {
    String(String),
    NameIndex(u16),
    TypeAndNameIndex(u16),
    StringIndex(u16),
    DescIndex(u16),
    ClassIndex(u16),
}
impl Const {
    pub fn parse(loader: &ProgrammLoader) -> Result<Self, VmError> {
        todo!()
    }
    pub fn get_str(&self) -> Result<String, VmError> {
        match self {
            Const::String(str) => {return Ok(str.clone())},
            _ => {return Err("Not a String in Const::get_str".to_owned())},
        }
    }
}

type ConstPool = Vec<Const>;

struct Memory<const SIZE: usize> {
    memory: [Bucket; SIZE],
}

impl<const SIZE: usize> Memory<SIZE> {
    pub fn init() -> Self {
        Self { memory: [Bucket::default(); SIZE] }
    }
}

struct Field {
    flags: u16,
    name: String,
    descriptors: String,
    attributes: Vec<Attribute>,
}

struct Attribute {
    name: String,
    data: Vec<Byte>,
}

struct Struct {
    name: String,
    fields: Vec<Field>,
}

struct VmImaginary<const SIZE: usize> {
    memory: Memory<SIZE>,
    pc: usize,
    p: Programm,
}

impl<const SIZE: usize> VmImaginary<SIZE> {
    pub fn new() -> Self {
        Self {
            memory: Memory::<SIZE>::init(),
            pc: 0,
            p: Programm {},
        }
    }
    pub fn exec_program(programm: Programm) -> Result<(), Box<dyn std::error::Error>> { todo!() }
}

