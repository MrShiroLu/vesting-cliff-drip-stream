use soroban_sdk::{Address, Env};

use crate::types::{DataKey, VestingSchedule};

/// Number of ledgers to extend TTL on persistent storage entries.
/// ~30 days at ~5s per ledger (6 * 60 * 24 * 30 = 259_200).
const PERSISTENT_LEDGER_THRESHOLD: u32 = 259_200;
const PERSISTENT_BUMP_AMOUNT: u32 = 518_400; // ~60 days

// ── Read ─────────────────────────────────────────────────────────────────────

/// Returns the vesting schedule for `recipient`, or `None` if absent.
///
/// Bumps the entry's TTL, since a schedule fetched on this path is about to
/// be mutated (claim/cancel/drain) and must not expire mid-stream. Read-only
/// views should call [`get_schedule_readonly`] instead to skip the extra
/// storage-write instructions.
pub fn get_schedule(env: &Env, recipient: &Address) -> Option<VestingSchedule> {
    let key = DataKey::Schedule(recipient.clone());
    let schedule = env
        .storage()
        .persistent()
        .get::<DataKey, VestingSchedule>(&key)?;
    env.storage()
        .persistent()
        .extend_ttl(&key, PERSISTENT_LEDGER_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
    Some(schedule)
}

/// Returns the vesting schedule for `recipient` without bumping its TTL.
///
/// For pure read-only views (`claimable_amount`, `get_status`, ...) that are
/// called far more often than the contract's mutating entry points and gain
/// nothing from refreshing the entry's expiry on every call.
pub fn get_schedule_readonly(env: &Env, recipient: &Address) -> Option<VestingSchedule> {
    env.storage()
        .persistent()
        .get::<DataKey, VestingSchedule>(&DataKey::Schedule(recipient.clone()))
}

/// Returns `true` if a schedule already exists for `recipient`.
pub fn has_schedule(env: &Env, recipient: &Address) -> bool {
    env.storage()
        .persistent()
        .has(&DataKey::Schedule(recipient.clone()))
}

// ── Write ─────────────────────────────────────────────────────────────────────

/// Persists `schedule` for `recipient` and bumps its TTL.
pub fn set_schedule(env: &Env, recipient: &Address, schedule: &VestingSchedule) {
    let key = DataKey::Schedule(recipient.clone());
    env.storage().persistent().set(&key, schedule);
    env.storage().persistent().extend_ttl(
        &key,
        PERSISTENT_LEDGER_THRESHOLD,
        PERSISTENT_BUMP_AMOUNT,
    );
}

/// Removes the schedule for `recipient` (called after full stream exhaustion
/// or cancellation).
pub fn remove_schedule(env: &Env, recipient: &Address) {
    env.storage()
        .persistent()
        .remove(&DataKey::Schedule(recipient.clone()));
}

// ── Admin ────────────────────────────────────────────────────────────────────

/// Returns the contract's admin address, or `None` if `initialize` has not
/// been called yet.
pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&DataKey::Admin)
}

/// Persists `admin` in instance storage.
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}
