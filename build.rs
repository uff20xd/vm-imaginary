mod redoxri;
use redoxri::*;

const RUST_FLAGS: &[&str] = &["--edition=2024"];

fn main() {
    let _redoxri = Redoxri::new(&[""]);

    let vm = Mcule::new("vm-imaginary", "./vm-imaginary")
        .with(&["src/main.rs".into()])
        .add_step(&["rustc", "src/main.rs", "-o", "$out"])
        .with_flags(RUST_FLAGS)
        .compile()
        .run();
}
