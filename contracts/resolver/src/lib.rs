#![no_std]
mod errors;
mod types;
mod utils;
use crate::errors::*;
use crate::utils::*;
use soroban_sdk::BytesN;
use soroban_sdk::{
    contract, contractimpl, contractimport, panic_with_error, symbol_short, Address, Bytes, Env,
    Symbol,
};
use types::*;

mod registry {
    super::contractimport!(file = "../../target/wasm32-unknown-unknown/release/registry.wasm");
}

const REGISTRY: Symbol = symbol_short!("registry");
const ADMIN: Symbol = symbol_short!("admin");

#[contract]
pub struct Resolver;

#[contractimpl]
impl Resolver {
    pub fn __constructor(env: Env, admin: Address, registry: Address) {
        env.extend_me();
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&REGISTRY, &registry);
    }

    pub fn is_name_has_record(env: Env, name: Bytes, tld: Bytes) -> bool {
        env.extend_me();
        env.storage()
            .instance()
            .has(&RecordKeys::Name(name.clone(), tld.clone()))
    }

    pub fn set_record(env: Env, name: Bytes, tld: Bytes, record_type: Bytes, data: Bytes) {
        env.extend_me();
        name.validate_name(&env, true);
        let client = registry::Client::new(&env, &env.storage().instance().get(&REGISTRY).unwrap());
        if client.is_name_expired(&name, &tld) {
            panic_with_error!(&env, Error::NameExpired);
        }
        let owner = client.get_owner(&name, &tld);
        owner.require_auth();

        let mut record = Record::NullRecord;
        if record_type == Bytes::from_slice(&env, "name".as_bytes()) {
            record = Record::Name(Address::from_string_bytes(&data));
        } else if record_type == Bytes::from_slice(&env, "ipfs".as_bytes()) {
            record = Record::Ipfs(data.clone());
        } else if record_type == Bytes::from_slice(&env, "text".as_bytes()) {
            record = Record::Text(data.clone());
        } else {
            panic_with_error!(&env, Error::RecordTypeInvalid);
        }
        env.storage()
            .instance()
            .set(&RecordKeys::Name(name.clone(), tld.clone()), &record);
    }

    pub fn resolve_name(env: Env, name: Bytes, tld: Bytes) -> Record {
        env.extend_me();
        if Self::is_name_has_record(env.clone(), name.clone(), tld.clone()) {
            let client =
                registry::Client::new(&env, &env.storage().instance().get(&REGISTRY).unwrap());
            if client.is_name_expired(&name, &tld) {
                panic_with_error!(&env, Error::NameExpired);
            }
            return env
                .storage()
                .instance()
                .get(&RecordKeys::Name(name.clone(), tld.clone()))
                .unwrap();
        } else {
            panic_with_error!(&env, Error::NameHasNoRecord);
        }
    }

    pub fn upgrade(e: Env, new_wasm_hash: BytesN<32>) {
        let admin: Address = e.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        e.deployer().update_current_contract_wasm(new_wasm_hash);
    }
}

mod test;