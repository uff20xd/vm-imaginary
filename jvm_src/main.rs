use std::fs;
use std::io::Read;
use std::path::Path;
use std::convert::TryFrom;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vm = VmImaginary::<256>::new();
    let bucket = Bucket::default();
    let mut programm_loader = ProgrammLoader::new("tests/Add.class")?;
    //dbg!(&programm_loader);
    let _bla = programm_loader.get_next_bytes(8);
    let const_pool = programm_loader.parse_pool();
    //let flags = programm_loader.get_next_bytes_as_usize(2);
    //let name = resolve(&const_pool, programm_loader.get_next_bytes_as_usize(2));
    //let ssuper = resolve(&const_pool, programm_loader.get_next_bytes_as_usize(2));
    //let interfaces = programm_loader.parse_interfaces(&const_pool);
    //let fields = programm_loader.parse_fields(&const_pool);
    //let methods = programm_loader.parse_fields(&const_pool);
    //let _interfaces = programm_loader.parse_interfaces(const_pool);
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
        let bytes = self.get_next_bytes(n);
        let mut id_bytes: [Byte; 8] = [0; 8];
        let bytes_len = bytes.len();
        for i in 0..bytes_len {
            id_bytes[bytes_len - i - 1] = bytes[i];
        }
        let num: usize = usize::from_le_bytes(id_bytes);
        num
    }
    pub fn parse_pool(&mut self) -> ConstPool {
        let mut const_pool = ConstPool::new();
        let len: usize = self.get_next_bytes_as_usize(2)-1;
        for i in 0..len {
            //println!("Iteration for const Pool: {}", i);
            const_pool.push(self.parse_const());
        }
        const_pool
    }
    pub fn parse_const(&mut self) -> Const {
        let tag = self.get_next_bytes(1)[0];
        let mut c = Const::default();
        match tag {
            0x01 => {
                let len = self.get_next_bytes_as_usize(2);
                c.tag = ConstTag::String;
                c.string = str::from_utf8(self.get_next_bytes(len)).unwrap().to_owned();
            },
            0x07 => {
                c.tag = ConstTag::ClassIndex;
                c.name_index = self.get_next_bytes_as_usize(2) as u16;
            },
            0x08 => {
                c.tag = ConstTag::StringRefrenceIndex;
                c.string_index = self.get_next_bytes_as_usize(2) as u16;
            },
            0x09 | 0x0a => {
                c.tag = ConstTag::FieldAndMethod;
                c.class_index = self.get_next_bytes_as_usize(2) as u16;
                c.name_and_type_index = self.get_next_bytes_as_usize(2) as u16;
            },
            0x0c => {
                c.tag = ConstTag::NameAndTypeIndex;
                c.name_index = self.get_next_bytes_as_usize(2) as u16;
                c.desc_index = self.get_next_bytes_as_usize(2) as u16;
            },
            _ => {
                dbg!(&tag);
                panic!("Fuck you")
            },
        }
        c
    }
    fn parse_interfaces(&mut self, const_pool: &ConstPool) -> Vec<String> {
        let interface_count = self.get_next_bytes_as_usize(2);
        let mut interfaces = Vec::new();
        for i in 0..interface_count {
            interfaces.push(resolve(const_pool, i));
        }
        interfaces
    }
    fn parse_fields(&mut self, const_pool: &ConstPool) -> Vec<Field> {
        let field_count = self.get_next_bytes_as_usize(2);
        let mut fields = Vec::new();
        for i in 0..field_count {
            fields.push(Field {
                flags: self.get_next_bytes_as_usize(2) as u16,
                name: resolve(const_pool, self.get_next_bytes_as_usize(2)),
                descriptors: resolve(const_pool, self.get_next_bytes_as_usize(2)),
                attributes: self.parse_attributes(&const_pool),
            }
            );
        }
        fields
    }

    fn parse_attributes(&mut self, const_pool: &ConstPool) ->  Vec<Attribute> {
        let attribute_count = self.get_next_bytes_as_usize(2);
        let mut attributes = Vec::new();
        for i in 0..attribute_count {
            attributes.push(Attribute {
                name: resolve(const_pool, self.get_next_bytes_as_usize(2)),
                data: self.get_next_bytes(4).into(),
            }
            );
        }
        attributes
    }
}

#[derive(Default)]
struct Programm {
    const_pool: ConstPool,
    name: String,
    ssuper: String,
    flags: u16,
    interfaces: Vec<String>,
    fields: Vec<Field>,
    methods: Vec<Field>,
    attributes: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default)]
struct Bucket {
    bucket: [Byte; 8],
    full: bool,
}

#[derive(Default, Debug, Clone)]
enum ConstTag {
    String,
    ClassIndex,
    StringRefrenceIndex,
    FieldAndMethod,
    NameAndTypeIndex,
    #[default]
    NonState,
}

#[derive(Default, Debug, Clone)]
struct Const {
    tag: ConstTag,
    string: String,
    name_index: u16,
    class_index: u16,
    name_and_type_index: u16,
    string_index: u16,
    desc_index: u16,
}

impl Const {
    pub fn get_str(&self) -> Result<String, VmError> {
        match self.tag {
            ConstTag::String => {return Ok(self.string.clone())},
            _ => {
                println!("{:?}", self.tag);
                return Err("Not a String in Const::get_str".to_owned())
            },
        }
    }
}

type ConstPool = Vec<Const>;

fn resolve(const_pool: &ConstPool, mut index: usize) -> String {
    let teva = const_pool[index-1].get_str();
    match teva {
        Ok(val) => {val},
        Err(_) => { 
            println!("{}", index - 1);
            return "".to_owned();
        }
    }
}

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

struct Class {
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
            p: Programm { ..Default::default() },
        }
    }
    pub fn exec_program(programm: Programm) -> Result<(), Box<dyn std::error::Error>> { todo!() }
}

