mod redoxri;
use redoxri::*;

const RUST_FLAGS: &[&str] = &["--edition=2024"];
const LIB_FLAGS: &[&str] = &["-Awarnings"];

fn main() {
    let _redoxri = Redoxri::new(&[""]);

    let out = Mcule::new("output", "out/")
        .add_step(&["mkdir", "out"])
        .compile();

    let vm = Mcule::new("vm_imaginary", "./out/libvm_imaginary.rlib")
        .with(&["src/lib.rs".into(), "src/vm_imaginary.rs".into()])
        .add_step(&["rustc", "src/lib.rs", "--crate-type=rlib", "-o", "$out"])
        .with_flags(RUST_FLAGS)
        .with_flags(LIB_FLAGS)
        .compile();

    let vm_test1 = Mcule::new("vm-imaginary-test1", "./out/tests")
        .with(&["tests/machine_test.rs".into(), "src/lib.rs".into(), "src/vm_imaginary.rs".into()])
        .add_step(&["rustc", "tests/machine_test.rs", "-o", "$out", "--extern", &(vm.name.clone() + "=" + &vm.outpath)])
        .with_flags(RUST_FLAGS)
        .compile()
        .run();
}
