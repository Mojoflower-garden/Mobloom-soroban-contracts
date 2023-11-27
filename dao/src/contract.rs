use crate::proposal::{
    add_abstain_votes, add_against_votes, add_for_votes, add_proposal, check_min_vote_power,
    check_voted, executed, for_votes_win, get_proposal, get_voted, is_passed_deadline,
    min_quorum_met, set_executed, set_min_vote_power, set_voted, votes_counts, Proposal,
    ProposalInstr, VotesCount,
};
use crate::storage::core::CoreState;
use crate::storage::core::{
    PERSISTENT_BUMP_AMOUNT_HIGH_WATERMARK, PERSISTENT_BUMP_AMOUNT_LOW_WATERMARK,
};
use crate::storage::proposal_storage::ProposalStorageKey;

use crate::utils::core::{can_init_contract, get_core_state, set_core_state};

use soroban_sdk::{
    contract, contractimpl, panic_with_error, symbol_short, vec, Address, Bytes, BytesN, Env,
    IntoVal, Map, String, Symbol, Val, Vec,
};

use crate::errors::VoteError;
use crate::errors::{ContractError, ExecutionError};

pub trait DaoContractTrait {
    fn init(
        env: Env,
        gov_token_salt: BytesN<32>,
        token_wasm_hash: BytesN<32>,
        gov_token_name: String,
        gov_token_symbol: String,
        voting_power: u32,
        proposal_power: u32,
        shareholders: Map<Address, i128>,
    ) -> Address;
    fn create_proposal(env: Env, from: Address, proposal: Proposal) -> u32;
    fn vote(
        env: Env,
        from: Address,
        prop_id: u32,
        power: u32,
        vote: u32,
        tokens: i128,
        t_contract: Address,
    );
    fn get_votes(env: Env, prop_id: u32) -> VotesCount;
    fn have_voted(env: Env, prop_id: u32, vote: Address) -> bool;
    // fn execute(env: Env, prop_id: u32) -> Vec<Val>;
    fn execute(env: Env, prop_id: u32) -> bool;
}

#[contract]
pub struct DaoContract;

#[contractimpl]
impl DaoContractTrait for DaoContract {
    fn init(
        env: Env,
        gov_token_salt: BytesN<32>,
        token_wasm_hash: BytesN<32>,
        gov_token_name: String,
        gov_token_symbol: String,
        voting_power: u32,
        proposal_power: u32,
        shareholders: Map<Address, i128>,
    ) -> Address {
        can_init_contract(&env);
        // Deploy the contract using the installed WASM code with given hash.
        let id = env
            .deployer()
            .with_current_contract(gov_token_salt.to_val())
            .deploy(token_wasm_hash.to_val());

        let init_fn: Symbol = Symbol::new(&env, "initialize");
        let admin: Val = env.current_contract_address().to_val();
        let init_args: Vec<Val> = vec![
            &env,
            admin,
            18u32.into(),
            gov_token_name.into(),
            gov_token_symbol.into(),
        ] as Vec<Val>;

        // Invoke the init function with the given arguments.
        let res: Val = env.invoke_contract(&id, &init_fn, init_args);

        let mint_fn: Symbol = symbol_short!("mint");
        let authorize_fn: Symbol = symbol_short!("set_auth");

        set_min_vote_power(&env, voting_power);

        let mut shareholdersVector: Vec<Address> = Vec::new(&env);
        for (shareholder_address, amount) in shareholders.iter() {
            let shareholder_address_raw: Val = shareholder_address.to_val();
            shareholdersVector.push_front(shareholder_address);
            let auth_args: Vec<Val> = vec![&env, shareholder_address_raw, true.into_val(&env)];
            let auth_res: Val = env.invoke_contract(&id, &authorize_fn, auth_args);

            let mint_args: Vec<Val> =
                vec![&env, shareholder_address_raw, amount.into_val(&env)] as Vec<Val>;
            let mint_res: Val = env.invoke_contract(&id, &mint_fn, mint_args);
        }

        set_core_state(
            &env,
            &CoreState {
                governance_token: id.clone(),
                shareholders: shareholdersVector,
            },
        );

        id
    }

    fn create_proposal(env: Env, from: Address, proposal: Proposal) -> u32 {
        // verify
        // verify nonce

        // check_min_duration(&env, &proposal);
        // check_min_prop_power(&env, get_dao_token_client(&env).power(&from));
        //todo, store the token supply at this point
        add_proposal(&env, proposal)
    }

    fn vote(
        env: Env,
        from: Address,
        prop_id: u32,
        power: u32,
        vote: u32,
        tokens: i128,
        t_contract: Address,
    ) {
        let key = ProposalStorageKey::Proposal(prop_id);
        // 1. Check if DAO member
        let core_state = get_core_state(&env);
        if !core_state.shareholders.contains(from.clone()) {
            panic_with_error!(env, VoteError::NotAMember)
        }

        // 2. Check if already voted
        check_voted(&env, prop_id, from.clone());

        // 3. Check if has enough voting power to vote
        check_min_vote_power(&env, power);

        // 4. Check deadline
        let proposal = get_proposal(&env, prop_id);
        if is_passed_deadline(&env, &proposal) {
            panic_with_error!(env, ContractError::MinDurationNotSatisfied)
        }

        // 5. Check amount of tokens
        let balance_fn: Symbol = symbol_short!("balance");
        let shareholder_address_raw: Val = from.to_val();
        let balance_args: Vec<Val> = vec![&env, shareholder_address_raw] as Vec<Val>;
        let balance_res: i128 = env.invoke_contract(&t_contract, &balance_fn, balance_args);

        if balance_res < tokens {
            panic_with_error!(env, VoteError::TokenAmountSurpassed)
        }

        // 6. Vote
        if vote == 0 {
            add_against_votes(&env, prop_id, tokens);
            set_voted(&env, prop_id, from);
        } else if vote == 1 {
            add_for_votes(&env, prop_id, tokens);
            set_voted(&env, prop_id, from);
        } else if vote == 2 {
            add_abstain_votes(&env, prop_id, tokens);
            set_voted(&env, prop_id, from);
        } else {
            panic_with_error!(env, VoteError::WrongVoteParam)
        }
        env.storage().persistent().bump(
            &key,
            PERSISTENT_BUMP_AMOUNT_LOW_WATERMARK,
            PERSISTENT_BUMP_AMOUNT_HIGH_WATERMARK,
        );
    }

    // fn execute(env: Env, prop_id: u32) -> Vec<Val> {
    fn execute(env: Env, prop_id: u32) -> bool {
        let proposal = get_proposal(&env, prop_id);
        // 1. Check deadline
        if !is_passed_deadline(&env, &proposal) {
            panic_with_error!(env, ContractError::TooEarlyToExecute)
        }

        // 2. Check if min quorum is met
        min_quorum_met(&env, prop_id);

        // 3. Check if "for" votes beat "against" votes
        for_votes_win(&env, prop_id);

        // 4. Check if executed
        if executed(&env, prop_id) {
            panic_with_error!(env, ContractError::AllreadyExecuted)
        }
        // 5. Execute
        let mut exec_results: Vec<Val> = Vec::new(&env);

        for (instruct) in proposal.instr.iter() {
            let res: Val = env.invoke_contract(&instruct.c_id, &instruct.fun_name, instruct.args);
            exec_results.push_front(res);
        }

        // 6. Set executed to true
        set_executed(&env, prop_id);
        true

        // // 7. Return results
        // exec_results
    }

    fn get_votes(env: Env, prop_id: u32) -> VotesCount {
        votes_counts(&env, prop_id)
    }

    fn have_voted(env: Env, prop_id: u32, voter: Address) -> bool {
        get_voted(&env, prop_id, voter)
    }
}
