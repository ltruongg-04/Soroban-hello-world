#![no_std]

use soroban_sdk::{contractimpl, symbol, Address, Env};

pub struct DigitalIdentityContract;

#[contractimpl]
impl DigitalIdentityContract {
    // Set or update an identity's metadata (string or JSON)
    pub fn set_identity(env: Env, user: Address, metadata: String) {
        // key form: "IDENTITY:<G...>"
        let key = format!("IDENTITY:{}", user.serialize(&env));
        env.storage().set(&key, &metadata);
    }

    // Get identity metadata for a given address (returns Option<String>)
    pub fn get_identity(env: Env, user: Address) -> Option<String> {
        let key = format!("IDENTITY:{}", user.serialize(&env));
        env.storage().get(&key)
    }

    // Register an item ID to an owner with metadata
    pub fn register_item(env: Env, item_id: String, owner: Address, metadata: String) {
        let k_owner = format!("ITEM_OWNER:{}", item_id);
        let k_meta = format!("ITEM_META:{}", item_id);
        env.storage().set(&k_owner, &owner);
        env.storage().set(&k_meta, &metadata);
    }

    // Get item owner and metadata; returns tuple of Option<Address> and Option<String>
    pub fn get_item(env: Env, item_id: String) -> (Option<Address>, Option<String>) {
        let k_owner = format!("ITEM_OWNER:{}", item_id);
        let k_meta = format!("ITEM_META:{}", item_id);
        let owner: Option<Address> = env.storage().get(&k_owner);
        let meta: Option<String> = env.storage().get(&k_meta);
        (owner, meta)
    }
}

// Export the contract
mod contract {
    soroban_sdk::contractimport!(file = "../../target/wasm32-unknown-unknown/release/digital_identity.wasm");
}
export_contract!(DigitalIdentityContract);
