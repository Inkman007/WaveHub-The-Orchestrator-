use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Wave {
    pub total_pool: i128,
    pub start_time: u64,
    pub end_time: u64,
    pub finalized: bool,
}

#[contracttype]
pub enum DataKey {
    Wave(u64),
}
