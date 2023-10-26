use core::ops::Add;

use soroban_sdk::xdr::DataEntry;
use soroban_sdk::{
    contracttype, panic_with_error, symbol_short, vec, Address, BytesN, Env, String, Symbol, Val,
    Vec,
};

use crate::utils::core::get_core_state;
use crate::{
    errors::ContractError, settings::get_min_prop_duration,
    storage::proposal_storage::ProposalStorageKey,
};

use crate::errors::ExecutionError;
use crate::storage::core::CoreState;

#[contracttype]
#[derive(Clone, Debug)]
pub struct ProposalVoted {
    pub voter: Address,
    pub prop_id: u32,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct ProposalInstr {
    //contract id
    pub c_id: Address,
    pub fun_name: Symbol,
    pub args: Vec<Val>,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Proposal {
    pub end_time: u64,
    // instrunctions will be executed in sequence
    pub url: String,
    pub instr: Vec<ProposalInstr>,
    pub min_quo: i128,
}

#[contracttype]
#[derive(Clone)]
pub struct VotesCount {
    pub v_for: i128,
    pub v_against: i128,
    pub v_abstain: i128,
}
// add prop and return its id
pub fn add_proposal(env: &Env, proposal: Proposal) -> u32 {
    let prop_id = get_and_inc_prop_id(env);
    env.storage()
        .persistent()
        .set(&ProposalStorageKey::Proposal(prop_id), &proposal);
    set_prop_start_ledger(env, prop_id, env.ledger().sequence());

    prop_id
}

pub fn get_proposal(env: &Env, prop_id: u32) -> Proposal {
    env.storage()
        .persistent()
        .get(&ProposalStorageKey::Proposal(prop_id))
        .unwrap_or_else(|| panic_with_error!(env, ContractError::InvalidProposalId))
}

fn get_and_inc_prop_id(env: &Env) -> u32 {
    let prev = env
        .storage()
        .persistent()
        .get(&ProposalStorageKey::ProposalId)
        .unwrap_or(0u32);

    env.storage()
        .persistent()
        .set(&ProposalStorageKey::ProposalId, &(prev + 1));
    prev
}

pub fn check_min_duration(env: &Env, proposal: &Proposal) {
    let min_duration = get_min_prop_duration(env);
    if proposal.end_time - env.ledger().timestamp() < (min_duration as u64) {
        panic_with_error!(env, ContractError::MinDurationNotSatisfied)
    }
}

pub fn set_voted(env: &Env, prop_id: u32, voter: Address) {
    env.storage().persistent().set(
        &ProposalStorageKey::Voted(ProposalVoted { voter, prop_id }),
        &true,
    )
}

pub fn get_voted(env: &Env, prop_id: u32, voter: Address) -> bool {
    env.storage()
        .persistent()
        .get(&ProposalStorageKey::Voted(ProposalVoted { voter, prop_id }))
        .unwrap_or(false)
}

pub fn check_voted(env: &Env, prop_id: u32, voter: Address) {
    if get_voted(env, prop_id, voter) {
        panic_with_error!(env, ContractError::AlreadyVoted)
    }
}

pub fn set_prop_start_ledger(env: &Env, prop_id: u32, start_ledger: u32) {
    env.storage()
        .persistent()
        .set(&ProposalStorageKey::PropStart(prop_id), &start_ledger)
}

pub fn get_prop_start_ledger(env: &Env, prop_id: u32) -> u32 {
    env.storage()
        .persistent()
        .get(&ProposalStorageKey::PropStart(prop_id))
        .unwrap()
}

pub fn get_for_votes(env: &Env, prop_id: u32) -> i128 {
    env.storage()
        .persistent()
        .get(&ProposalStorageKey::ForVotes(prop_id))
        .unwrap_or(0)
}

fn set_for_votes(env: &Env, prop_id: u32, amount: i128) {
    env.storage()
        .persistent()
        .set(&ProposalStorageKey::ForVotes(prop_id), &amount)
}

pub fn add_for_votes(env: &Env, prop_id: u32, amount: i128) {
    if amount < 0 {
        panic_with_error!(env, ContractError::CannotAddNegativeVote)
    }

    let curr_votes = get_for_votes(env, prop_id);
    set_for_votes(env, prop_id, curr_votes + amount)
}

pub fn get_against_votes(env: &Env, prop_id: u32) -> i128 {
    env.storage()
        .persistent()
        .get(&ProposalStorageKey::AgainstV(prop_id))
        .unwrap_or(0)
}

fn set_against_votes(env: &Env, prop_id: u32, amount: i128) {
    env.storage()
        .persistent()
        .set(&ProposalStorageKey::AgainstV(prop_id), &amount)
}

pub fn add_against_votes(env: &Env, prop_id: u32, amount: i128) {
    if amount < 0 {
        panic_with_error!(env, ContractError::CannotAddNegativeVote)
    }

    let curr_votes = get_against_votes(env, prop_id);
    set_against_votes(env, prop_id, curr_votes + amount)
}

pub fn get_abstain_votes(env: &Env, prop_id: u32) -> i128 {
    env.storage()
        .persistent()
        .get(&ProposalStorageKey::AbstainV(prop_id))
        .unwrap_or(0)
}

fn set_abstain_votes(env: &Env, prop_id: u32, amount: i128) {
    env.storage()
        .persistent()
        .set(&ProposalStorageKey::AbstainV(prop_id), &amount)
}

pub fn add_abstain_votes(env: &Env, prop_id: u32, amount: i128) {
    if amount < 0 {
        panic_with_error!(env, ContractError::CannotAddNegativeVote)
    }

    let curr_votes = get_abstain_votes(env, prop_id);
    set_abstain_votes(env, prop_id, curr_votes + amount)
}

pub fn set_min_proposal_power(env: &Env, min_power: i128) {
    env.storage()
        .persistent()
        .set(&ProposalStorageKey::MinPropP, &min_power)
}

pub fn set_min_vote_power(env: &Env, min_power: u32) {
    env.storage()
        .persistent()
        .set(&ProposalStorageKey::MinVoteP, &min_power)
}

pub fn get_min_proposal_power(env: &Env) -> i128 {
    env.storage()
        .persistent()
        .get(&ProposalStorageKey::MinPropP)
        .unwrap_or(0)
}

pub fn get_min_vote_power(env: &Env) -> u32 {
    env.storage()
        .persistent()
        .get(&ProposalStorageKey::MinVoteP)
        .unwrap_or(0)
}

pub fn check_min_prop_power(env: &Env, power: i128) {
    if get_min_proposal_power(env) > power {
        panic_with_error!(env, ContractError::NotEnoughPower)
    }
}

pub fn check_min_vote_power(env: &Env, power: u32) {
    if get_min_vote_power(env) > power {
        panic_with_error!(env, ContractError::NotEnoughPower)
    }
}

pub fn votes_counts(env: &Env, prop_id: u32) -> VotesCount {
    let for_votes = get_for_votes(&env, prop_id);
    let against_votes = get_against_votes(&env, prop_id);
    let abstain_votes = get_abstain_votes(&env, prop_id);

    VotesCount {
        v_for: for_votes,
        v_against: against_votes,
        v_abstain: abstain_votes,
    }
}

pub fn set_executed(env: &Env, prop_id: u32) {
    env.storage()
        .persistent()
        .set(&ProposalStorageKey::Executed(prop_id), &true)
}

pub fn executed(env: &Env, prop_id: u32) -> bool {
    env.storage()
        .persistent()
        .get(&ProposalStorageKey::Executed(prop_id))
        .unwrap_or(false)
}

pub fn min_quorum_met(env: &Env, prop_id: u32) {
    let proposal = get_proposal(&env, prop_id);
    let votes = votes_counts(&env, prop_id);
    let t_votes = votes.v_abstain + votes.v_against + votes.v_for;
    if t_votes < proposal.min_quo {
        panic_with_error!(env, ExecutionError::MinQuoNotMet)
    }
}

pub fn for_votes_win(env: &Env, prop_id: u32) {
    let for_votes = get_for_votes(&env, prop_id);
    let agains_votes = get_against_votes(&env, prop_id);
    if for_votes >= agains_votes {
        panic_with_error!(env, ExecutionError::ForProposalFail)
    }
}
