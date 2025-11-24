mod redoxri;
use redoxri::*;

fn main() {
    let _redoxri = Redoxri::new(&[
    ]);

    //let alloc = Mcule::new("allocator", "")
    let vm = Mcule::new("vm-imaginary", "vm-imaginary")
        .add_step(&["rustc", "src/main.rs", "-o", "$out"])
        .compile();
}
