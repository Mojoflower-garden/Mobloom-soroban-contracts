#![no_std]

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Symbol, Val, Vec};

#[contract]
pub struct Deployer;

#[contractimpl]
impl Deployer {
    /// Deploy the contract wasm and after deployment invoke the init function
    /// of the contract with the given arguments. Returns the contract ID and
    /// result of the init function.
    pub fn deploy(
        env: Env,
        salt: BytesN<32>,
        wasm_hash: BytesN<32>,
        init_fn: Symbol,
        init_args: Vec<Val>,
    ) -> (Address, Val) {
        // Deploy the contract using the installed WASM code with given hash.
        let id = env.deployer().with_current_contract(salt).deploy(wasm_hash);
        // Invoke the init function with the given arguments.
        let res: Val = env.invoke_contract(&id, &init_fn, init_args);
        // Return the contract ID of the deployed contract and the result of
        // invoking the init result.
        (id, res)
    }
}

mod test;

// soroban contract install --wasm ./target/wasm32-unknown-unknown/release/soroban_deployer_contract.wasm
// deployer-wasm-hash -> 95900fed357813b41cc8b7f39aabaae2c4a52de29b3f32d6da471a2def55f5b7

// soroban contract install --wasm ../dao/target/wasm32-unknown-unknown/release/governance.wasm
// dao-wasm-hash -> 3b8b33b24ed6cf32fde3c5291632f1b4545228c13706cbb14abfea77b50fc9f7

// soroban contract install --wasm ../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm
// token-wasm-hash -> cf2318b87338b80ce75d0276244f9be3a131d74656a7fecd1d92da0eb8ab09e3

// soroban --vv contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/soroban_deployer_contract.wasm \
//     --source juico \
//     --network futurenet
// deployer_contract_id -> CBQHB5XVIJH5XYRHN5VCZZTWDZC2ORILRMKYIZASOE6J73BQ24DPEDYF

soroban --vv contract deploy \
    --wasm target/wasm32-unknown-unknown/release/governance.wasm \
    --source juico \
    --network futurenet
// dao_contract_id -> CARW6CGBX3TFV3VMEFPNRC5VNHWQOXZARWBHYK4PEBREXS2QEAWN6BKX

// soroban --vv contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/soroban_token_contract.wasm \
//     --source juico \
//     --network futurenet
// token_contract_id -> CDXW3LGCXU57ZJ3XSF4VAGBTDJAQG5X5AG6NE5YEFDGMQQHN5Y3E7HC3

// ------------------------------------ FOR PROPOSAL EXECUTION ------------------------------------

// soroban --vv contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/demo_instruct_exec.wasm \
//     --source juico \
//     --network futurenet
// token_contract_id -> CB3HRDYSR3VIHLB3CZCN4GCMR6BDFALK43VR2LH5YLWTRKX7V3NH3DXU

// ------------------------------------ FOR RESTORATION ------------------------------------

// soroban --vv contract restore \
//     --id CCQNGCPYYJXIGOY4KEWVN6QGKN7N2EFJIRNKUJDVEPTT5ANKITGXBNE3 \
//     --source juico \
//     --network futurenet \
//     --key-xdr AAAAFA==

// soroban --vv contract invoke \
//     --id CCHONWYCG5HWW4R6WT3M26TUE7TDOT6CTJKWDUS74VWSQW27345G42NN \
//     --source juico \
//     --network futurenet -- -h \

// soroban --vv contract restore \
//     --wasm-hash cf2318b87338b80ce75d0276244f9be3a131d74656a7fecd1d92da0eb8ab09e3 \
//     --source juico \
//     --network futurenet
