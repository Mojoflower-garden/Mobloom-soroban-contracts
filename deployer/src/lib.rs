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

// soroban contract install --wasm ../dao/target/wasm32-unknown-unknown/release/governance.wasm
// dao-wasm-hash -> 7fae616fe6ce00b6f79434dcd80ace4d881fb0295af7909c5617dd215544cbad

// soroban contract install --wasm ../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm
// token-wasm-hash -> c336d084f055b55562320041b7daa3ac8cc18c6be6b9ec34726ceee98dd030c7

// soroban --vv contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/soroban_deployer_contract.wasm \
//     --source juico \
//     --network futurenet
// deployer_contract_id -> CAC3OGDAHENQA22XLNDBMJ7KAWEKJWLKJVJWK3NMWGYGM6PFYDAVK5CE

// soroban contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/governance.wasm \
//     --source juico \
//     --network futurenet
// dao_contract_id -> CCZXRZQWMLRNLASAR5LOQJPIUGCVTKCRWY6FJ27TPUVBGJPN4CPYWJOM

// soroban contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/soroban_token_contract.wasm \
//     --source juico \
//     --network futurenet
// token_contract_id -> CAGEDGPP4EQM5AOJIJ6PFHMT2FABFOIEK4EFIRAPRS2WLB3QKJYAX7L3

// ------------------------------------ FOR RESTORATION ------------------------------------

// soroban --vv contract restore \
//     --id CA6B25QPWAHTMPC5NZO36PEOPMG7ENSITE24CYWC4WVAIL2UCXOSLILH \
//     --source juico \
//     --network futurenet

// soroban --vv contract invoke \
//     --id CAGGNI3F7IORBOFKEHOPVD2RLCSRTAQADUF6CVARTT2JHAZOYE2ARBKA \
//     --source juico \
//     --network futurenet -- -h

// soroban --vv contract restore \
//     --wasm-hash 0c66a365fa1d10b8aa21dcfa8f5158a51982001d0be154119cf493832ec13408 \
//     --source juico \
//     --network futurenet
