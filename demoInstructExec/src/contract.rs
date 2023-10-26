use soroban_sdk::{
    contract, contractimpl, panic_with_error, symbol_short, vec, Address, Bytes, BytesN, Env,
    IntoVal, Map, String, Symbol, Val, Vec,
};

pub trait DemoContractTrait {
    fn demo_exec(env: Env, greeting: String) -> String;
}

#[contract]
pub struct DemoContract;

#[contractimpl]
impl DemoContractTrait for DemoContract {
    fn demo_exec(env: Env, greeting: String) -> String {
        greeting
    }
}
