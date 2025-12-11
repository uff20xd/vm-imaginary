mod redoxri;
use redoxri::*;

fn main() {
    let _redoxri = Redoxri::new(&[
    ]);

    let vm = Mcule::new("vm-imaginary", "./vm-imaginary")
        .with(&["src/main.rs".into()])
        .add_step(&["rustc", "src/main.rs", "-o", "$out"])
        .compile()
        .run();

}
