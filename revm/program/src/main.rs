// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use std::u64;

// use alloy::primitives::U32;
// use alloy_sol_types::sol_data::Address;
use revm::primitives::{Bytecode, CancunSpec};
use revm::primitives::Bytes;
use revm_interpreter::analysis::to_analysed;
use revm_interpreter::opcode::InstructionTable;
use revm_interpreter::DummyHost;
use revm_interpreter::{Contract, Interpreter, EMPTY_SHARED_MEMORY};


pub fn main() {
    // let input = Bytes::from(format!("{:032x}", 10)); //
    // println!("length: {}", input.len()); 
    // let mut file = File::open("test/exploits/integer_overflow/integer_overflow_1_sol_MyFib.bin").unwrap();
    // let mut buf = Vec::new();
    // file.read_to_end(&mut buf).unwrap();
    println!("cycle-tracker-start: set up input");
    let n = sp1_zkvm::io::read::<u32>();
    sp1_zkvm::io::commit(&n);
    // let n = 257u32;
    // println!("n: {}", n);

    // let n = 1000u32;
    
    let mut padded_bytes = [0u8; 32];
    padded_bytes[28..32].copy_from_slice(&n.to_be_bytes());
    // println!("padded bytes: {:#x?}", padded_bytes);

    let buf = hex::decode("608060405234801561000f575f80fd5b5060043610610029575f3560e01c8063f9b7c7e51461002d575b5f80fd5b6100476004803603810190610042919061010b565b61005d565b6040516100549190610145565b60405180910390f35b5f808263ffffffff1603610073575f90506100c9565b5f60019050600191505f600290505b8363ffffffff168163ffffffff1610156100c6575f611eef84846100a6919061018b565b6100b091906101ef565b9050839250809350508080600101915050610082565b50505b919050565b5f80fd5b5f63ffffffff82169050919050565b6100ea816100d2565b81146100f4575f80fd5b50565b5f81359050610105816100e1565b92915050565b5f602082840312156101205761011f6100ce565b5b5f61012d848285016100f7565b91505092915050565b61013f816100d2565b82525050565b5f6020820190506101585f830184610136565b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601160045260245ffd5b5f610195826100d2565b91506101a0836100d2565b9250828201905063ffffffff8111156101bc576101bb61015e565b5b92915050565b7f4e487b71000000000000000000000000000000000000000000000000000000005f52601260045260245ffd5b5f6101f9826100d2565b9150610204836100d2565b925082610214576102136101c2565b5b82820690509291505056fea2646970667358221220f70970a5468d801d3ea74899b70d5b05ab9a40b447e4bc77f406f3d3370ce79964736f6c634300081a0033")
    .unwrap();    
    let function_selector = hex::decode("f9b7c7e5").unwrap();

    let mut call_data_raw = Vec::new();
    call_data_raw.extend(function_selector);
    call_data_raw.extend(padded_bytes);
    let call_data = Bytes::from(call_data_raw);
    // println!("call data {:?}", call_data);

    let bytecode = Bytecode::new_raw_checked(Bytes::copy_from_slice(&buf)).unwrap();

    let analyzed = to_analysed(bytecode.clone());

    println!("cycle-tracker-end: set up input");

    println!("cycle-tracker-start: set up runtime");

    // println!("analyzed: {:?}", analyzed.is_eof());

    let mut interp = Interpreter::new(
        Contract{
            input: call_data,
            bytecode: analyzed,
            ..Default::default()
        },
        u64::MAX,
        true,
    );

    let mut host = crate::DummyHost::default();
    let table: &InstructionTable<DummyHost> =
        &revm_interpreter::opcode::make_instruction_table::<DummyHost, CancunSpec>();
    println!("cycle-tracker-end: set up runtime");
    println!("cycle-tracker-start: interpreter");
    let raw_out = interp.run(EMPTY_SHARED_MEMORY, table, &mut host);
    println!("cycle-tracker-end: interpreter");

    // println!("output: {:?}", out);

    let out = raw_out.into_result_return().unwrap().output;
    // println!("output: {:?}", out);
    // commit to output
    sp1_zkvm::io::commit(&out);

}
