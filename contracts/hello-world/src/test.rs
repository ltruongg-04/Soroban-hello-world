#![cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Env, testutils::Address as _, BytesN, IntoVal, Vec, symbol_short};

    #[test]
    fn create_update_revoke_flow() {
        let env = Env::default();
        let owner = Address::random(&env);
        let id = symbol_short!("alice_id");
        let h1 = BytesN::from_array(&env, &[1u8;32]);
        // register contract in env
        let contract_id = env.register_contract(None, DigitalIdentityContract);
        let client = DigitalIdentityContractClient::new(&env, &contract_id);

        // Create - requires owner's auth
        env.set_source_account(&owner);
        client.create(&id, owner.clone(), h1.clone());

        // Read back
        let (on, hash, revoked) = client.get(&id);
        assert_eq!(on, owner);
        assert_eq!(hash, h1);
        assert!(!revoked);

        // Update
        let h2 = BytesN::from_array(&env, &[2u8;32]);
        env.set_source_account(&owner);
        client.update(&id, owner.clone(), h2.clone());
        let (_on2, hash2, _rv) = client.get(&id);
        assert_eq!(hash2, h2);

        // Revoke
        env.set_source_account(&owner);
        client.revoke(&id, owner.clone());
        let (_on3, _hash3, rv3) = client.get(&id);
        assert!(rv3);
    }
}
