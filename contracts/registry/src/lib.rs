#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, symbol_short, token,
    Address, Env, String, Symbol,
};

const ONE_YEAR: u64 = 365 * 24 * 60 * 60;
const ADMIN: Symbol = symbol_short!("admin");
const RESOLVER: Symbol = symbol_short!("resolver");
const ASSET: Symbol = symbol_short!("asset");
const ASSET_PER_YEAR: u64 = 20;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NameAlreadyRegistered = 1,
    NameNotRegistered = 2,
    NameInvalid = 3,
    NameExpired = 4,
    NameHasNoResolveData = 5,
    ResolverAlreadySet = 6,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Domain {
    pub owner: Address,
    pub resolver: Address,
    pub expiry: u64,
}

#[contract]
pub struct Registry;

#[contractimpl]
impl Registry {
    pub fn __constructor(env: Env, asset: Address) {
        env.storage().instance().set(&ASSET, &asset);
    }

    pub fn is_name_registered(env: Env, name: String) -> bool {
        env.storage().persistent().has(&name)
    }

    pub fn set_resolver(env: Env, resolver: Address) {
        if !env.storage().instance().has(&RESOLVER) {
            env.storage().instance().set(&RESOLVER, &resolver);
        } else {
            panic_with_error!(&env, Error::ResolverAlreadySet);
        }
    }

    pub fn register_name(env: Env, name: String, owner: Address, number_of_years: u64) -> bool {
        // let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        // admin.require_auth();
        owner.require_auth();

        if Self::is_name_registered(env.clone(), name.clone()) {
            panic_with_error!(&env, Error::NameAlreadyRegistered);
        }

        token::Client::new(&env, &env.storage().instance().get(&ASSET).unwrap()).transfer(
            &owner,
            &env.current_contract_address(),
            &(number_of_years * ASSET_PER_YEAR).into(),
        );

        let domain = Domain {
            owner,
            resolver: env.storage().instance().get(&RESOLVER).unwrap(),
            expiry: env.ledger().timestamp() + (number_of_years * ONE_YEAR),
        };
        env.storage().persistent().set(&name, &domain);
        true
    }

    pub fn get_resolver(env: Env, name: String) -> Address {
        if Self::is_name_registered(env.clone(), name.clone()) {
            let domain: Domain = env.storage().persistent().get(&name).unwrap();
            if Self::is_name_expired(env.clone(), name.clone()) {
                panic_with_error!(&env, Error::NameExpired);
            }
            return domain.resolver;
        } else {
            panic_with_error!(&env, Error::NameNotRegistered);
        }
    }

    pub fn get_owner(env: Env, name: String) -> Address {
        if Self::is_name_registered(env.clone(), name.clone()) {
            let domain: Domain = env.storage().persistent().get(&name).unwrap();
            if Self::is_name_expired(env.clone(), name.clone()) {
                panic_with_error!(&env, Error::NameExpired);
            }
            return domain.owner;
        } else {
            panic_with_error!(&env, Error::NameNotRegistered);
        }
    }

    pub fn transfer(env: Env, name: String, new_owner: Address) -> bool {
        if Self::is_name_registered(env.clone(), name.clone()) {
            let mut domain: Domain = env.storage().persistent().get(&name).unwrap();
            if Self::is_name_expired(env.clone(), name.clone()) {
                panic_with_error!(&env, Error::NameExpired);
            }
            domain.owner.require_auth();
            domain.owner = new_owner;
            env.storage().persistent().set(&name, &domain);
        } else {
            panic_with_error!(&env, Error::NameNotRegistered);
        }

        true
    }

    pub fn is_name_expired(env: Env, name: String) -> bool {
        if Self::is_name_registered(env.clone(), name.clone()) {
            let domain: Domain = env.storage().persistent().get(&name).unwrap();
            if domain.expiry < env.ledger().timestamp() {
                return true;
            }
            return false;
        } else {
            panic_with_error!(&env, Error::NameNotRegistered);
        }
    }
}

mod test;
