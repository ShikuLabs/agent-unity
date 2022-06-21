use anyhow::{anyhow, Result};
use ic_types::Principal;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

pub const IC_MAIN_NET: &str = "https://ic0.app";

lazy_static! {
    static ref IDL_LOOKUP: Mutex<HashMap<Principal, String>> = Mutex::new(HashMap::new());
}

pub fn register_idl(canister_id: Principal, idl_file: String) -> Result<()> {
    IDL_LOOKUP
        .lock()
        .map_err(|e| anyhow!(e.to_string()))
        .and_then(|mut lookup| {
            if lookup.contains_key(&canister_id) != true {
                lookup.insert(canister_id, idl_file);

                Ok(())
            } else {
                Err(anyhow!("{} has been added already.", canister_id))
            }
        })
}

pub fn remove_idl(canister_id: &Principal) -> Result<String> {
    IDL_LOOKUP
        .lock()
        .map_err(|e| anyhow!(e.to_string()))
        .and_then(|mut lookup| {
            lookup
                .remove(canister_id)
                .ok_or(anyhow!("{} is not exist.", canister_id))
        })
}

pub fn get_idl(canister_id: &Principal) -> Result<String> {
    IDL_LOOKUP
        .lock()
        .map_err(|e| anyhow!(e.to_string()))
        .and_then(|lookup| {
            lookup
                .get(canister_id)
                .cloned()
                .ok_or(anyhow!("{} is not exist.", canister_id))
        })
}

pub fn list_idl() -> Result<Vec<Principal>> {
    IDL_LOOKUP
        .lock()
        .map_err(|e| anyhow!(e.to_string()))
        .and_then(|lookup| Ok(lookup.keys().map(|cid| cid.clone()).collect()))
}

pub fn query() {
    todo!()
}

pub fn update() {
    todo!()
}
