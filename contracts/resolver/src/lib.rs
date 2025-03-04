#![no_std]
mod errors;
use crate::errors::*;
use soroban_sdk::{
    contract, contractimpl, contractimport, contracttype, panic_with_error, symbol_short, Address,
    Bytes, Env, Symbol,
};

mod registry {
    super::contractimport!(file = "../../target/wasm32-unknown-unknown/release/registry.wasm");
}

const REGISTRY: Symbol = symbol_short!("registry");

pub trait Base {
    fn extend_me(&self);
    fn delete_name(&self, name: &Bytes, tld: &Bytes);
}

impl Base for Env {
    fn extend_me(&self) {
        self.storage().instance().extend_ttl(17280, 17280 * 30);
    }

    fn delete_name(&self, name: &Bytes, tld: &Bytes) {
        self.storage()
            .instance()
            .remove(&DataKey::Name(name.clone(), tld.clone()));
    }
}

#[contracttype]
pub enum DataKey {
    Name(Bytes, Bytes),
}

#[contract]
pub struct Resolver;

#[contractimpl]
impl Resolver {
    pub fn __constructor(env: Env, registry: Address) {
        env.extend_me();
        env.storage().instance().set(&REGISTRY, &registry);
    }

    pub fn is_name_has_resolve_data(env: Env, name: Bytes, tld: Bytes) -> bool {
        env.extend_me();
        env.storage()
            .instance()
            .has(&DataKey::Name(name.clone(), tld.clone()))
    }

    pub fn set_resolve_data(env: Env, name: Bytes, tld: Bytes, address: Address) {
        env.extend_me();
        let client = registry::Client::new(&env, &env.storage().instance().get(&REGISTRY).unwrap());
        if client.is_name_expired(&name, &tld) {
            panic_with_error!(&env, Error::NameExpired);
        }
        let owner = client.get_owner(&name, &tld);
        owner.require_auth();

        env.storage()
            .instance()
            .set(&DataKey::Name(name.clone(), tld.clone()), &address);
    }

    pub fn resolve_name(env: Env, name: Bytes, tld: Bytes) -> Address {
        env.extend_me();
        if Self::is_name_has_resolve_data(env.clone(), name.clone(), tld.clone()) {
            let client =
                registry::Client::new(&env, &env.storage().instance().get(&REGISTRY).unwrap());
            if client.is_name_expired(&name, &tld) {
                panic_with_error!(&env, Error::NameExpired);
            }
            return env
                .storage()
                .instance()
                .get(&DataKey::Name(name.clone(), tld.clone()))
                .unwrap();
        } else {
            panic_with_error!(&env, Error::NameHasNoResolveData);
        }
    }
}

mod test;
