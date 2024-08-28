# Wasm / EVM interpreters in SP1

These experiments demonstrate that running a WASM interpreter in SP1 is feasible. We write a simple program that computes the n'th fibonacci number mod 7919, and compile it to a .wat and evm bytecode. As a baseline, we implement this program in plain rust. We run the evm bytecode through `revm-interpreter`, and we run the .wat through the `wasmi` and `wain` interpreters. 

We evaluate the number of sp1 cycles it takes to execute the fibonacci program in various interpreters.

## Running

For the experiment you want to run, go to the `[experiment]/script` directory and run `RUST_LOG=info cargo run --release`. For example, if you run `wasmi-fibonacci`, you might get a result like this.

```
2024-08-28T17:30:03.205258Z  INFO execute: clk = 0 pc = 0x32fdc8    
2024-08-28T17:30:03.205383Z  INFO execute: ┌╴set up input    
stdout: WARNING: Using insecure random number generator.
2024-08-28T17:30:03.205859Z  INFO execute: │ ┌╴parse WAT    
2024-08-28T17:30:03.214236Z  INFO execute: │ └╴254,941 cycles    
2024-08-28T17:30:03.214266Z  INFO execute: └╴265,762 cycles    
2024-08-28T17:30:03.214282Z  INFO execute: ┌╴set up runtime    
2024-08-28T17:30:03.215770Z  INFO execute: └╴37,384 cycles    
2024-08-28T17:30:03.215787Z  INFO execute: ┌╴interpreter    
2024-08-28T17:30:03.220264Z  INFO execute: └╴171,361 cycles    
2024-08-28T17:30:03.255805Z  INFO execute: close time.busy=99.7ms time.idle=9.33µs
...
```

This means that 254,941 cycles were spent parsing the WAT, 37,384 cycles were spent setting up the runtime, and 171,361 cycles were spent executing the wasm. 

## Results

| Experiment Name | Set up input | Set up runtime | Interpreter loop | Total cycles |
|-----------------|--------------|----------------|------------------|--------------|
| Baseline        | --           | --             | --               | 16,944       |
| Wasmi           | 264,332      | 37,349         | 171,362          | 423,981      |
| Wain            | 141,943      | 1,025          | 2,520,476        | 2,667,383    |
| Revm            | 42,395       | 11,262         | 9,095,092        | 9,153,940    |

## Other notes

`fib.sol` compiles into the EVM bytecode I used for `revm`. 
