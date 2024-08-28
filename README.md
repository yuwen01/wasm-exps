# Installation

I had the sp1 repo adjacent to this one. That is, this repo was at `~/wasm-exps`, and sp1 was at `~/sp1`. To run `wasmi-fibonacci`, you need to add the following lines to the top of `/sp1/crates/zkvm/entrypoint/src/libm.rs`

```
#[no_mangle]
pub extern "C" fn rint(arg: f64) -> f64 {
    libm::rint(arg)
}

#[no_mangle]
pub extern "C" fn rintf(arg: f32) -> f32 {
    libm::rintf(arg)
}
```

# Running

Go to a `script` directory and run `cargo run --release`
