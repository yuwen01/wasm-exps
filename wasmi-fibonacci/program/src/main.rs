#![no_main]
sp1_zkvm::entrypoint!(main);

// use std::{fs::File, io::Write};

use wasmi::*;
// #[sp1_derive::cycle_tracker]

pub fn main() {
    // First step is to create the Wasm execution engine with some config.
    // In this example we are using the default configuration.

    // Write n to public input
    println!("cycle-tracker-start: set up input");
    let n = sp1_zkvm::io::read::<i32>();
    sp1_zkvm::io::commit(&n);

    // let n = 1000;

    let engine = Engine::default();
    let wat = r#"
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
    // Wasmi does not yet support parsing `.wat` so we have to convert
    // out `.wat` into `.wasm` before we compile and validate it.
    let wasm = wat::parse_str(&wat).unwrap();

    // let mut file = File::open("wasm/fib.wasm").unwrap();   // Create or truncate the file
    // file.write_all(&wasm).unwrap();                              // Write the data to the file
    println!("cycle-tracker-end: set up input");
    println!("cycle-tracker-start: set up runtime");

    let module = Module::new(&engine, &mut &wasm[..]).unwrap();
    // All Wasm objects operate within the context of a `Store`.
    // Each `Store` has a type parameter to store host-specific data,
    // which in this case we are using `42` for.
    type HostState = u32;
    let mut store = Store::new(&engine, 42);
    // In order to create Wasm module instances and link their imports
    // and exports we require a `Linker`.
    let linker = <Linker<HostState>>::new(&engine);
    // Instantiation of a Wasm module requires defining its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    //
    // Also before using an instance created this way we need to start it.
    let instance = linker
        .instantiate(&mut store, &module).unwrap()
        .start(&mut store).unwrap();
    let hello = instance.get_typed_func::<i32, i32>(&store, "fib").unwrap();

    println!("cycle-tracker-end: set up runtime");
    println!("cycle-tracker-start: interpreter");
    // And finally we can call the wasm!
    let result = &hello.call(&mut store, n).unwrap();

    // println!("Result: {}", result);
    sp1_zkvm::io::commit(result);
    println!("cycle-tracker-end: interpreter");
}
