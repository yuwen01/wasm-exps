#![no_main]
sp1_zkvm::entrypoint!(main);
use std::io::stdout;

use wain_exec::{DefaultImporter, Runtime, Value};
use wain_syntax_binary::parse;

const WASM: &[u8] = include_bytes!("../../../fib.wasm");
pub fn main() {

    println!("cycle-tracker-start: set up input");

    let n = sp1_zkvm::io::read::<i32>();
    sp1_zkvm::io::commit(&n);

    let tree = match parse(WASM) {
        Ok(tree) => tree,
        Err(err) => {
            eprintln!("Could not parse: {}", err);
            panic!();
        }
    };
    println!("cycle-tracker-end: set up input");

    println!("cycle-tracker-start: set up runtime");
    
    let input = "some dummy text";
    let stdout = stdout();
    let importer = DefaultImporter::with_stdio(input.as_bytes(), stdout.lock());
    // Instantiate the module
    let mut runtime = match Runtime::instantiate(&tree.module, importer) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("could not instantiate module: {}", err);
            panic!();
        }
    };

    println!("cycle-tracker-end: set up runtime");
    println!("cycle-tracker-start: run interpreter");

    // Find the exported function
    let x = match runtime.invoke("fib", &[Value::I32(n)]) {
        Ok(ret) => {
            if let Some(Value::I32(i)) = ret {
                i
            } else {
                panic!();
            }
        }

        Err(trap) => {
            eprintln!("Function execution failed: {}", trap);
            panic!();
        }
    };
    sp1_zkvm::io::commit(&x);

    println!("cycle-tracker-end: run interpreter");
}
