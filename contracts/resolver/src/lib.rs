#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contractimport, panic_with_error, symbol_short, Address,
    Env, String, Symbol,
};

mod registry {
    super::contractimport!(file = "../../target/wasm32-unknown-unknown/release/registry.wasm");
}

const REGISTRY: Symbol = symbol_short!("registry");

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NameAlreadyRegistered = 1,
    NameNotRegistered = 2,
    NameInvalid = 3,
    NameExpired = 4,
    NameHasNoResolveData = 5,
}

#[contract]
pub struct Resolver;

#[contractimpl]
impl Resolver {
    pub fn __constructor(env: Env, registry: Address) {
        env.storage().instance().set(&REGISTRY, &registry);
    }

    pub fn is_name_has_resolve_data(env: Env, name: String) -> bool {
        env.storage().persistent().has(&name)
    }

    pub fn set_resolve_data(env: Env, name: String, address: Address) -> bool {
        let client = registry::Client::new(&env, &env.storage().instance().get(&REGISTRY).unwrap());

        let owner = client.get_owner(&name);
        owner.require_auth();

        env.storage().persistent().set(&name, &address);

        true
    }

    pub fn resolve_name(env: Env, name: String) -> Address {
        if Self::is_name_has_resolve_data(env.clone(), name.clone()) {
            let client =
                registry::Client::new(&env, &env.storage().instance().get(&REGISTRY).unwrap());
            if client.is_name_expired(&name) {
                panic_with_error!(&env, Error::NameExpired);
            }
            return env.storage().persistent().get(&name).unwrap();
        } else {
            panic_with_error!(&env, Error::NameHasNoResolveData);
        }
    }
}

mod test;
