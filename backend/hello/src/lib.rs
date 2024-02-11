use candid::{CandidType, Result};
use ic_cdk::storage;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const IDENTITIES_KEY: &str = "identities";

#[derive(CandidType, Deserialize, Serialize, Default)]
struct Identity {
    name: String,
    age: u32,
}

#[derive(CandidType, Deserialize, Serialize, Default)]
struct IdentityMap(HashMap<u64, Identity>);

impl IdentityMap {
    fn new() -> Self {
        IdentityMap(HashMap::new())
    }

    fn save(&self) {
        storage::stable_save((IDENTITIES_KEY, &self.0));
    }

    fn load() -> Self {
        let data: Option<HashMap<u64, Identity>> = storage::stable_restore(IDENTITIES_KEY);
        data.map_or_else(|| IdentityMap::new(), IdentityMap)
    }

    fn update_identity(&mut self, id: u64, updated_identity: Identity) -> Result<(), String> {
        if let Some(existing_identity) = self.0.get_mut(&id) {
            // Update the fields of existing_identity with updated_identity
            *existing_identity = updated_identity;
            self.save();
            Ok(())
        } else {
            Err("Identity not found".to_string())
        }
    }
}

#[export_name = "canister_update register_identity"]
fn register_identity(identity: Identity) -> Result<(), String> {
    let caller = ic_cdk::caller();
    let mut identities = IdentityMap::load();

    identities.save();
    Ok(())
}

#[export_name = "canister_query get_identity"]
fn get_identity(id: u64) -> Option<Identity> {
    let identities = IdentityMap::load();

    identities.0.get(&id).cloned()
}

#[export_name = "canister_update update_identity"]
fn update_identity(id: u64, updated_identity: Identity) -> Result<(), String> {
    let mut identities = IdentityMap::load();
    identities.update_identity(id, updated_identity)
}

#[export_name = "canister_update delete_identity"]
fn delete_identity(id: u64) -> Result<(), String> {
    let mut identities = IdentityMap::load();


    identities.save();
    Ok(())
}
