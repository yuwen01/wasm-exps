#![no_main]
sp1_zkvm::entrypoint!(main);
use wasmi::*;

const WASM: &[u8] = include_bytes!("../../../fib.wasm");

pub fn main() {
  
    // Write n to public input
    println!("cycle-tracker-start: set up input");
    let n = sp1_zkvm::io::read::<i32>();
    sp1_zkvm::io::commit(&n);
    
    // First step is to create the Wasm execution engine with some config.
    // In this example we are using the default configuration.
    let engine = Engine::default();
    
    let module = Module::new(&engine, &mut &WASM[..]).unwrap();
    println!("cycle-tracker-end: set up input");

    println!("cycle-tracker-start: set up runtime");
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
    println!("cycle-tracker-start: run interpreter");
    // And finally we can call the wasm!
    let result = &hello.call(&mut store, n).unwrap();

    sp1_zkvm::io::commit(result);
    println!("cycle-tracker-end: run interpreter");
}
