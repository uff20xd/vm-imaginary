use std::collections::BTreeSet;

fn main() -> Result {}

struct Vm {
    runtime: Runtime,
    program: Program,
}

struct Program {
    types: BTreeSet<Type>,
    const_pool: ConstPool,
}

type ConstPool = Vec<Const>;

enum Const {
    String(String)
    StringIndex(u32),
    TypeIndex(u32),
    NameAndPlainType {
        name: u32,
        desc: u32,
    },
    FieldAndMethod {
        type: u32,
        name_and_plain_type: u32,
    },
}
