fn main() {
    let wasm_bytes = {
        let status = std::process::Command::new("cargo")
            .args(&[
                "build",
                "--release",
                "--lib",
                "--target=wasm32-unknown-unknown",
            ])
            .status()
            .unwrap();
        assert!(status.success());
        std::fs::read("./target/wasm32-unknown-unknown/release/benchmarks.wasm").unwrap()
    };

    let jit = wasmer::JIT::new(wasmer::Singlepass::new());
    let store = wasmer::Store::new(&jit.engine());
    let module = wasmer::Module::new(&store, wasm_bytes).unwrap();

    // The module doesn't import anything, so we create an empty import object.
    let import_object = wasmer::imports! {
        "env" => { "host" => wasmer::Function::new_native(&store, host) }
    };
    let instance = wasmer::Instance::new(&module, &import_object).unwrap();

    let skewed_sum = instance.exports.get_function("f").unwrap();
    for _ in 0..100_000 {
        let res = skewed_sum.call(&[]);
        println!("res.is_ok() = {}", res.is_ok());
    }
}

fn host() {
    eprintln!("calling host function");
    let xs = vec![92; 1024 * 1024 * 100];
    eat_stack(0)
}

fn eat_stack(lvl: u32) {
    eat_stack(lvl.wrapping_add(1))
}
