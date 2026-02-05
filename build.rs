mod redoxri;
use redoxri::*;

const RUST_FLAGS: &[&str] = &["--edition=2024"];

fn main() {
    let _redoxri = Redoxri::new(&[""]);

    let out = Mcule::new("output", "out/")
        .add_step(&["mkdir", "out"])
        .compile();

    let vm = Mcule::new("vm-imaginary", "./out/vm-imaginary")
        .with(&["src/main.rs".into()])
        .add_step(&["rustc", "src/main.rs", "-o", "$out"])
        .with_flags(RUST_FLAGS)
        .compile()
        .run();

    // let vm_test1 = Mcule::new("vm-imaginary-test1", "./vm-imaginary")
    //     .with(&["src/main.rs".into()])
    //     .add_step(&["rustc", "src/main.rs", "-o", "$out"])
    //     .with_flags(RUST_FLAGS)
    //     .compile()
    //     .run();
}
