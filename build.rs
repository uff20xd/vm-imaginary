mod redoxri;
use redoxri::*;

fn main() {
    let _redoxri = Redoxri::new(&[
    ]);

    //let alloc = Mcule::new("allocator", "")
    let test_vm = Mcule::new("jvm_vm-imaginary", "./jvm_vm-imaginary")
        .with(&["jmv_src/main.rs".into()])
        .add_step(&["rustc", "jvm_src/main.rs", "-o", "$out"])
        .compile()
        .run();

    let vm = Mcule::new("vm-imaginary", "./vm-imaginary")
        .with(&["src/main.rs".into()])
        .add_step(&["rustc", "src/main.rs", "-o", "$out"])
        .compile()
        .run();

}
