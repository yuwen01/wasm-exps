#![no_main]
sp1_zkvm::entrypoint!(main);
use std::io::stdout;

use wain_exec::{DefaultImporter, Runtime, Value};
use wain_syntax_text::parse;

const WAT: &str = r#"
  (module
    ;; Import the memory to store our results
    ;; (memory 1)
    ;; (export "memory" (memory 0))
  
    ;; Import the 'print' function for outputting the result
    ;; (import "env" "print" (func $print (param i32)))
  
    ;; Function to calculate the n-th Fibonacci number
      (func $fib (param $n i32) (result i32)
      (local $a i32)
      (local $b i32)
      (local $temp i32)
  
      ;; If n <= 1, return n
      (if (i32.le_s (local.get $n) (i32.const 1))
        (then
          (local.get $n)
          (return)
        )
      )
  
      ;; Set initial values for the loop
      (local.set $a (i32.const 0))
      (local.set $b (i32.const 1))
  
      ;; Loop from 2 to n
      (loop $loop
        (local.set $temp (local.get $b))
            (local.set $b 
              (i32.rem_s 
                (i32.add (local.get $a) (local.get $b)) 
                (i32.const 7919)
              )
            )
      (local.set $a (local.get $temp))
        (local.set $n (i32.sub (local.get $n) (i32.const 1)))
        (br_if $loop (i32.gt_s (local.get $n) (i32.const 1)))
      )
  
      ;; Return the result
      (local.get $b)
    )
  
    ;; Export the Fibonacci function
    (export "fib" (func $fib))
  )
"#;

pub fn main() {

    println!("cycle-tracker-start: set up input");

    let n = sp1_zkvm::io::read::<i32>();
    sp1_zkvm::io::commit(&n);

    println!("cycle-tracker-start: parse WAT");
    // Parse the WAT code to a WebAssembly module
    let tree = match parse(WAT) {
        Ok(tree) => tree,
        Err(err) => {
            eprintln!("Could not parse: {}", err);
            panic!();
        }
    };
    println!("cycle-tracker-end: parse WAT");
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
