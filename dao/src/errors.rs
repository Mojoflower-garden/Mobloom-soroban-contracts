use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum SCErrors {
    // Core Errors
    ContractAlreadyInitiated = 10001,
    InitError = 10002,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    NotInit = 0,
    MinDurationNotSatisfied = 1,
    CannotAddNegativeVote = 2,
    InvalidNonce = 3,
    AlreadyVoted = 4,
    InvalidProposalId = 5,
    NotEnoughPower = 6,
    TooEarlyToExecute = 7,
    AllreadyExecuted = 8,
    ForVotesLessThanAgainstVotes = 9,
    PropDeadlinePassed = 10,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum VoteError {
    NotAMember = 0,
    ProposalExpired = 1,
    WrongVoteParam = 2,
    TokenAmountSurpassed = 3,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ExecutionError {
    MinParticipNotMet = 0,
    ForProposalFail = 1,
    MinQuoNotMet = 2,
}
