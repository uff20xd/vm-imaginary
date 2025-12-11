use std::collections::BTreeSet;

fn main() -> Result<(), ()> {
    Ok(())
}

struct Vm {
    runtime: Runtime,
    program: Program,
}

struct Runtime {
    frame: Frame,
}

struct Frame {

}

struct Program {
    types: BTreeSet<Type>,
    const_pool: ConstPool,
}

type ConstPool = Vec<Const>;

struct Type;
struct Field;

enum Const {
    String(String),
    StringIndex(u32),
    TypeIndex(u32),
    NameAndPlainType {
        name: u32,
        desc: u32,
    },
    FieldAndMethod {
        r#type: u32,
        name_and_plain_type: u32,
    },
}
