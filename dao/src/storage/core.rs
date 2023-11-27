use soroban_sdk::{contracttype, Address, Vec};

pub(crate) const PERSISTENT_BUMP_AMOUNT_HIGH_WATERMARK: u32 = 34560; // 2 days
pub(crate) const PERSISTENT_BUMP_AMOUNT_LOW_WATERMARK: u32 = 17280; // 1 day

#[contracttype]
pub struct CoreState {
    pub governance_token: Address,
    pub shareholders: Vec<Address>
}

#[contracttype]
pub enum CoreStorageKeys {
    CoreState,
}
