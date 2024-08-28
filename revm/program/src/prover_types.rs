use revm::primitives::{CfgEnv, TxEnv};
use revm_interpreter::primitives::{Address, BlockEnv, Bytes, U256};
use serde::{Deserialize, Serialize};

/// The input to the generation of environment proof
/// for the execution environment.
// TODO kept getting `deserialization failed: Io(Custom { kind: UnexpectedEof, error: "" })`
// TODO when using these types.
#[derive(Serialize, Deserialize)]
pub struct EnvProvingInput {
    /// Block environment context
    pub block_env: BlockEnv,
    /// Cfg environment context
    pub cfg_env: CfgEnv,
    /// Tx environment context
    pub tx_env: TxEnv,
}

impl EnvProvingInput {
    pub fn new(block_env: BlockEnv, cfg_env: CfgEnv, tx_env: TxEnv) -> Self {
        Self {
            block_env,
            cfg_env,
            tx_env,
        }
    }
}

pub type Addresses = Vec<Address>;
pub type Bytecodes = Vec<(Address, Bytes)>;
pub type Balances = Vec<(Address, U256)>;
pub type Storages = Vec<((Address, U256), U256)>;
pub type Nonces = Vec<(Address, u64)>;

/// The input to the generation of exploit proof
/// related to storage
#[derive(Serialize, Deserialize)]
pub struct StateProvingInput {
    /// The bytecodes for the execution env
    pub bytecodes: Bytecodes,
    /// The balances for the execution env
    pub balances: Balances,
    /// The storage values for the execution env
    pub storages: Storages,
    /// The nonces for the execution env
    pub nonces: Nonces,
}

impl StateProvingInput {
    pub fn new(
        bytecodes: Vec<(Address, Bytes)>,
        balances: Vec<(Address, U256)>,
        storages: Vec<((Address, U256), U256)>,
        nonces: Vec<(Address, u64)>,
    ) -> Self {
        Self {
            bytecodes,
            balances,
            storages,
            nonces,
        }
    }
}
