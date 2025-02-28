#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, panic_with_error, symbol_short,
     Address, Env, String, Symbol,
};

const ONE_YEAR: u64 = 365 * 24 * 60 * 60;
const ADMIN: Symbol = symbol_short!("admin");

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
    pub fn __constructor(env: Env, admin: Address) {
        env.storage().instance().set(&ADMIN, &admin);
    }

    pub fn is_name_registered(env: Env, name: String) -> bool {
        env.storage().persistent().has(&name)
    }

    pub fn register_name(
        env: Env,
        name: String,
        owner: Address,
        resolver: Address,
        number_of_years: u64,
    ) -> bool {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        if Self::is_name_registered(env.clone(), name.clone()) {
            panic_with_error!(&env, Error::NameAlreadyRegistered);
        }

        let domain = Domain {
            owner,
            resolver,
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
