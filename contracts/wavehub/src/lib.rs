#![no_std]

pub mod contract;
pub mod errors;
pub mod storage;
pub mod types;

pub use contract::{WaveHubContract, WaveHubContractClient};

#[cfg(test)]
mod tests {
    use soroban_sdk::{testutils::Address as _, Env};

    use crate::{
        contract::{WaveHubContract, WaveHubContractClient},
        errors::WaveHubError,
    };

    fn setup() -> (Env, soroban_sdk::Address, WaveHubContractClient<'static>) {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, WaveHubContract);
        let client = WaveHubContractClient::new(&env, &contract_id);
        let owner = soroban_sdk::Address::generate(&env);
        client.initialize(&owner).unwrap();
        (env, owner, client)
    }

    #[test]
    fn test_create_wave() {
        let (_env, owner, client) = setup();
        let wave_id = client.create_wave(&owner, &3600, &1_000_000).unwrap();
        assert_eq!(wave_id, 1);

        let wave = client.get_wave(&1).unwrap();
        assert_eq!(wave.total_pool, 1_000_000);
        assert!(!wave.finalized);
        assert_eq!(wave.end_time - wave.start_time, 3600);
    }

    #[test]
    fn test_wave_count_increments() {
        let (_env, owner, client) = setup();
        client.create_wave(&owner, &3600, &1_000_000).unwrap();
        client.create_wave(&owner, &7200, &2_000_000).unwrap();
        assert_eq!(client.wave_count(), 2);
    }

    #[test]
    fn test_non_owner_rejected() {
        let (env, _owner, client) = setup();
        let other = soroban_sdk::Address::generate(&env);
        let err = client.create_wave(&other, &3600, &1_000_000).unwrap_err();
        assert_eq!(err, WaveHubError::Unauthorized.into());
    }

    #[test]
    fn test_zero_duration_rejected() {
        let (_env, owner, client) = setup();
        let err = client.create_wave(&owner, &0, &1_000_000).unwrap_err();
        assert_eq!(err, WaveHubError::InvalidDuration.into());
    }

    #[test]
    fn test_zero_pool_rejected() {
        let (_env, owner, client) = setup();
        let err = client.create_wave(&owner, &3600, &0).unwrap_err();
        assert_eq!(err, WaveHubError::InvalidPool.into());
    }

    #[test]
    fn test_finalize_wave() {
        let (_env, owner, client) = setup();
        client.create_wave(&owner, &3600, &1_000_000).unwrap();
        client.finalize_wave(&owner, &1).unwrap();
        let wave = client.get_wave(&1).unwrap();
        assert!(wave.finalized);
    }

    #[test]
    fn test_double_finalize_rejected() {
        let (_env, owner, client) = setup();
        client.create_wave(&owner, &3600, &1_000_000).unwrap();
        client.finalize_wave(&owner, &1).unwrap();
        let err = client.finalize_wave(&owner, &1).unwrap_err();
        assert_eq!(err, WaveHubError::AlreadyFinalized.into());
    }

    #[test]
    fn test_double_initialize_rejected() {
        let (_env, owner, client) = setup();
        let err = client.initialize(&owner).unwrap_err();
        assert_eq!(err, WaveHubError::AlreadyInitialized.into());
    }
}
