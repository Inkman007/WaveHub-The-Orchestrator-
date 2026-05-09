use soroban_sdk::{symbol_short, Address, Env, Symbol};

use crate::{
    errors::WaveHubError,
    types::{DataKey, Wave},
};

const OWNER: Symbol = symbol_short!("OWNER");
const WAVE_COUNT: Symbol = symbol_short!("WCOUNT");

pub fn get_owner(env: &Env) -> Result<Address, WaveHubError> {
    env.storage()
        .instance()
        .get(&OWNER)
        .ok_or(WaveHubError::NotInitialized)
}

pub fn set_owner(env: &Env, owner: &Address) {
    env.storage().instance().set(&OWNER, owner);
}

pub fn has_owner(env: &Env) -> bool {
    env.storage().instance().has(&OWNER)
}

pub fn get_wave_count(env: &Env) -> u64 {
    env.storage().instance().get(&WAVE_COUNT).unwrap_or(0)
}

pub fn set_wave_count(env: &Env, count: u64) {
    env.storage().instance().set(&WAVE_COUNT, &count);
}

pub fn get_wave(env: &Env, wave_id: u64) -> Result<Wave, WaveHubError> {
    env.storage()
        .persistent()
        .get(&DataKey::Wave(wave_id))
        .ok_or(WaveHubError::WaveNotFound)
}

pub fn set_wave(env: &Env, wave_id: u64, wave: &Wave) {
    env.storage()
        .persistent()
        .set(&DataKey::Wave(wave_id), wave);
}
