use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env};

use crate::{
    errors::WaveHubError,
    storage,
    types::Wave,
};

#[contract]
pub struct WaveHubContract;

#[contractimpl]
impl WaveHubContract {
    /// Initialize the contract. Can only be called once.
    pub fn initialize(env: Env, owner: Address) -> Result<(), WaveHubError> {
        if storage::has_owner(&env) {
            return Err(WaveHubError::AlreadyInitialized);
        }
        storage::set_owner(&env, &owner);
        storage::set_wave_count(&env, 0);
        Ok(())
    }

    /// Create a new Wave. Caller must be the contract owner.
    pub fn create_wave(
        env: Env,
        caller: Address,
        duration: u64,
        total_pool: i128,
    ) -> Result<u64, WaveHubError> {
        caller.require_auth();

        let owner = storage::get_owner(&env)?;
        if caller != owner {
            return Err(WaveHubError::Unauthorized);
        }
        if duration == 0 {
            return Err(WaveHubError::InvalidDuration);
        }
        if total_pool <= 0 {
            return Err(WaveHubError::InvalidPool);
        }

        let wave_count = storage::get_wave_count(&env);
        let wave_id = wave_count + 1;

        let start_time = env.ledger().timestamp();
        let end_time = start_time
            .checked_add(duration)
            .ok_or(WaveHubError::DurationOverflow)?;

        let wave = Wave {
            total_pool,
            start_time,
            end_time,
            finalized: false,
        };

        storage::set_wave(&env, wave_id, &wave);
        storage::set_wave_count(&env, wave_id);

        env.events().publish(
            (symbol_short!("WaveNew"), wave_id),
            (total_pool, start_time, end_time),
        );

        Ok(wave_id)
    }

    /// Finalize a Wave. Caller must be the contract owner.
    pub fn finalize_wave(env: Env, caller: Address, wave_id: u64) -> Result<(), WaveHubError> {
        caller.require_auth();

        let owner = storage::get_owner(&env)?;
        if caller != owner {
            return Err(WaveHubError::Unauthorized);
        }

        let mut wave = storage::get_wave(&env, wave_id)?;
        if wave.finalized {
            return Err(WaveHubError::AlreadyFinalized);
        }

        wave.finalized = true;
        storage::set_wave(&env, wave_id, &wave);

        env.events()
            .publish((symbol_short!("WaveFin"), wave_id), ());

        Ok(())
    }

    /// Get a Wave by ID.
    pub fn get_wave(env: Env, wave_id: u64) -> Result<Wave, WaveHubError> {
        storage::get_wave(&env, wave_id)
    }

    /// Total number of Waves created.
    pub fn wave_count(env: Env) -> u64 {
        storage::get_wave_count(&env)
    }

    /// Return the contract owner address.
    pub fn owner(env: Env) -> Result<Address, WaveHubError> {
        storage::get_owner(&env)
    }
}
