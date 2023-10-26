#![cfg(test)]
#![cfg(test_prop_execution)]

use crate::{contract::DaoContract, DaoContractClient};
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, IntoVal, Map, String, Vec};

// The contract that will be deployed by the deployer contract.
mod contract {
    soroban_sdk::contractimport!(
        file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
    );
}

#[test]
fn test() {
    let env = Env::default();
    let client = DaoContractClient::new(&env, &env.register_contract(None, DaoContract));

    // Install the WASM code to be deployed from the deployer contract.
    let token_wasm_hash = env.deployer().upload_contract_wasm(contract::WASM);
    let admin1 = Address::random(&env);
    let mut shareholder: Map<Address, i128> = Map::new(&env);
    shareholder.set(admin1.clone(), 200000i128);

    // Deploy contract using deployer, and include an init function to call.
    let token_name = "token_name";
    let token_symbol = "token_symbol";
    let salt = BytesN::from_array(&env, &[0; 32]);
    let voting_power = 2;
    let proposal_power = 2;
    let mut shareholders: Map<Address, i128> = Map::new(&env);
    shareholders.set(admin1.clone(), 200000i128);

    let val = client.init(
        &salt,
        &token_wasm_hash,
        &String::from_slice(&env, token_name),
        &String::from_slice(&env, token_symbol),
        &voting_power,
        &proposal_power,
        &shareholders,
    );

    // assert_eq!(val)
}

#[test_prop_execution]
fn test_prop_execution() {
    let env = Env::default();
    let client = DaoContractClient::new(&env, &env.register_contract(None, DaoContract));

    client.execute(&1);
}
