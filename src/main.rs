fn main() {

}

type Byte = u8;

struct ProgrammLoader {
    bytes: Vec<Byte>,
    counter: usize,
}

impl ProgrammLoader{
    pub fn new() -> Result<Self, std::io::Error> { 
        let mut bytes = Vec::new();
        Ok(Self{
            bytes,
            counter: 0,
        })
    }
    pub fn get_bytes(&self, n: usize) -> &[Byte] {
        &self.bytes[self.counter..n]
    }
}

struct Bucket {
    bucket: [Byte; 8],
    full: bool,
}
struct Memory<const SIZE: usize> {
    memory: [Bucket; SIZE],
}

struct Field {}

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
}

