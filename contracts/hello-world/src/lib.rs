#![no_std]

use soroban_sdk::{
    contractimpl, contracttype, symbol_short, vec, vec::Vec, Address, Env, Symbol, BytesN, 
    Map, Bytes, Panic, panic_with_error
};

#[contracttype]
pub enum DataKey {
    Identity(Symbol),
    Metadata((Symbol, Symbol)), // (id, meta_key) -> Bytes
}

#[contracttype]
pub struct Identity {
    owner: Address,
    data_hash: BytesN<32>,
    revoked: bool,
}

pub struct DigitalIdentityContract;

#[contractimpl]
impl DigitalIdentityContract {
    // Create a new identity. Fails if identity already exists.
    pub fn create(env: Env, id: Symbol, owner: Address, data_hash: BytesN<32>) {
        // require the owner to have authorized this creation
        owner.require_auth();

        let key = DataKey::Identity(id.clone());
        if env.storage().has(&key) {
            panic_with_error!(&env, "IdentityAlreadyExists");
        }

        let record = Identity {
            owner: owner.clone(),
            data_hash,
            revoked: false,
        };
        env.storage().set(&key, &record);
        // emit a log for off-chain indexing
        env.events().publish((symbol_short!("identity"), symbol_short!("created")), vec![&env, id, owner]);
    }

    // Update the data hash of an identity (owner only)
    pub fn update(env: Env, id: Symbol, owner: Address, new_data_hash: BytesN<32>) {
        owner.require_auth();

        let key = DataKey::Identity(id.clone());
        let mut record: Identity = env.storage().get(&key).unwrap_or_else(|| panic_with_error!(&env, "IdentityNotFound"));
        // verify owner matches
        if record.owner != owner {
            panic_with_error!(&env, "Unauthorized");
        }
        if record.revoked {
            panic_with_error!(&env, "IdentityRevoked");
        }

        record.data_hash = new_data_hash;
        env.storage().set(&key, &record);
        env.events().publish((symbol_short!("identity"), symbol_short!("updated")), vec![&env, id]);
    }

    // Revoke an identity (owner only)
    pub fn revoke(env: Env, id: Symbol, owner: Address) {
        owner.require_auth();

        let key = DataKey::Identity(id.clone());
        let mut record: Identity = env.storage().get(&key).unwrap_or_else(|| panic_with_error!(&env, "IdentityNotFound"));
        if record.owner != owner {
            panic_with_error!(&env, "Unauthorized");
        }
        if record.revoked {
            panic_with_error!(&env, "AlreadyRevoked");
        }
        record.revoked = true;
        env.storage().set(&key, &record);
        env.events().publish((symbol_short!("identity"), symbol_short!("revoked")), vec![&env, id]);
    }

    // Read identity metadata: returns (owner, data_hash, revoked)
    pub fn get(env: Env, id: Symbol) -> (Address, BytesN<32>, bool) {
        let key = DataKey::Identity(id.clone());
        let record: Identity = env.storage().get(&key).unwrap_or_else(|| panic_with_error!(&env, "IdentityNotFound"));
        (record.owner, record.data_hash, record.revoked)
    }

    // Verify that a given hash matches the stored hash and that the identity is not revoked
    pub fn verify(env: Env, id: Symbol, expected_hash: BytesN<32>) -> bool {
        let key = DataKey::Identity(id.clone());
        match env.storage().get::<DataKey, Identity>(&key) {
            Some(record) => (!record.revoked) && (record.data_hash == expected_hash),
            None => false
        }
    }

    // Optional: set simple metadata key/value for identity (owner only)
    pub fn set_meta(env: Env, id: Symbol, owner: Address, meta_key: Symbol, meta_val: Bytes) {
        owner.require_auth();
        let key = DataKey::Identity(id.clone());
        let record: Identity = env.storage().get(&key).unwrap_or_else(|| panic_with_error!(&env, "IdentityNotFound"));
        if record.owner != owner {
            panic_with_error!(&env, "Unauthorized");
        }
        let meta_key_global = DataKey::Metadata((id, meta_key));
        env.storage().set(&meta_key_global, &meta_val);
        env.events().publish((symbol_short!("identity"), symbol_short!("meta_set")), vec![&env, id]);
    }

    pub fn get_meta(env: Env, id: Symbol, meta_key: Symbol) -> Bytes {
        let meta_key_global = DataKey::Metadata((id, meta_key));
        env.storage().get(&meta_key_global).unwrap_or_else(|| Bytes::new(&env))
    }
}
