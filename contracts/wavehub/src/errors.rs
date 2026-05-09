use soroban_sdk::contracterror;

#[contracterror]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum WaveHubError {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    Unauthorized = 3,
    InvalidDuration = 4,
    InvalidPool = 5,
    WaveNotFound = 6,
    AlreadyFinalized = 7,
    DurationOverflow = 8,
}
