#![no_std]
mod errors;
mod types;
mod utils;
use crate::errors::*;
use crate::utils::*;
use soroban_sdk::{
    contract, contractimpl, panic_with_error, symbol_short, token, Address, Bytes, Env, Symbol, Vec,
};
use types::*;

const ONE_YEAR_IN_SECONDS: u64 = 365 * 24 * 60 * 60;
const RESOLVER: Symbol = symbol_short!("resolver");
const ASSET: Symbol = symbol_short!("asset");
const TLDS: Symbol = symbol_short!("tlds");
const ASSET_AMOUNT_PER_YEAR: u64 = 20;

#[contract]
pub struct Registry;

#[contractimpl]
impl Registry {
    pub fn __constructor(env: Env, asset: Address, tlds: Vec<Bytes>) {
        env.extend_me();
        env.storage().instance().set(&ASSET, &asset);
        env.storage().instance().set(&TLDS, &tlds);
    }

    pub fn register_name(env: Env, name: Bytes, tld: Bytes, owner: Address, number_of_years: u64) {
        env.extend_me();
        owner.require_auth();
        name.validate_name(&env, false);
        tld.validate_tld(&env);
        if Self::is_name_registered(env.clone(), name.clone(), tld.clone()) {
            if !Self::is_name_expired(env.clone(), name.clone(), tld.clone()) {
                panic_with_error!(&env, Error::NameAlreadyRegistered);
            }
        }
        token::Client::new(&env, &env.storage().instance().get(&ASSET).unwrap()).transfer(
            &owner,
            &env.current_contract_address(),
            &(number_of_years * ASSET_AMOUNT_PER_YEAR).into(),
        );
        let domain: Domain = Domain {
            owner: owner.clone(),
            resolver: env.storage().instance().get(&RESOLVER).unwrap(),
            expiry: env.ledger().timestamp() + (number_of_years * ONE_YEAR_IN_SECONDS),
        };
        env.storage()
            .instance()
            .set(&DataKey::Name(name.clone(), tld.clone()), &domain);

        env.events().publish(
            (Symbol::new(&env, "register_name"),),
            (owner, name, tld, number_of_years),
        );
    }

    pub fn is_name_expired(env: Env, name: Bytes, tld: Bytes) -> bool {
        env.extend_me();
        let name: Bytes = name.get_root_name(&env);
        if !Self::is_name_registered(env.clone(), name.clone(), tld.clone()) {
            panic_with_error!(&env, Error::NameNotRegistered);
        }
        let domain: Domain = env
            .storage()
            .instance()
            .get(&DataKey::Name(name.clone(), tld.clone()))
            .unwrap();
        if domain.expiry < env.ledger().timestamp() {
            return true;
        }
        return false;
    }

    pub fn is_name_registered(env: Env, name: Bytes, tld: Bytes) -> bool {
        env.extend_me();
        let name: Bytes = name.get_root_name(&env);
        env.storage()
            .instance()
            .has(&DataKey::Name(name.clone(), tld.clone()))
    }

    pub fn get_name(env: Env, name: Bytes, tld: Bytes) -> Domain {
        env.extend_me();
        let name: Bytes = name.get_root_name(&env);
        if !Self::is_name_registered(env.clone(), name.clone(), tld.clone()) {
            panic_with_error!(&env, Error::NameNotRegistered);
        }
        if Self::is_name_expired(env.clone(), name.clone(), tld.clone()) {
            panic_with_error!(&env, Error::NameExpired);
        }
        env.storage()
            .instance()
            .get(&DataKey::Name(name.clone(), tld.clone()))
            .unwrap()
    }

    pub fn get_resolver(env: Env) -> Address {
        env.extend_me();
        env.storage().instance().get(&RESOLVER).unwrap()
    }

    pub fn set_resolver(env: Env, resolver: Address) {
        env.extend_me();
        if !env.storage().instance().has(&RESOLVER) {
            env.storage().instance().set(&RESOLVER, &resolver);
        } else {
            panic_with_error!(&env, Error::ResolverAlreadySet);
        }
    }

    pub fn get_owner(env: Env, name: Bytes, tld: Bytes) -> Address {
        env.extend_me();
        let name: Bytes = name.get_root_name(&env);
        let domain: Domain = Self::get_name(env.clone(), name.clone(), tld.clone());
        return domain.owner;
    }

    pub fn transfer(env: Env, name: Bytes, tld: Bytes, new_owner: Address) {
        env.extend_me();
        let name: Bytes = name.get_root_name(&env);
        let mut domain: Domain = Self::get_name(env.clone(), name.clone(), tld.clone());

        domain.owner.require_auth();
        domain.owner = new_owner;
        env.storage()
            .instance()
            .set(&DataKey::Name(name.clone(), tld.clone()), &domain);
    }

    pub fn make_sell_offer(env: Env, name: Bytes, tld: Bytes, price: u64) {
        env.extend_me();
        let name: Bytes = name.get_root_name(&env);
        let domain: Domain = Self::get_name(env.clone(), name.clone(), tld.clone());
        domain.owner.require_auth();
        let offer: Offer = Offer {
            seller: domain.owner.clone(),
            name: name.clone(),
            tld: tld.clone(),
            price,
        };
        env.storage()
            .instance()
            .set(&DataKey::Offer(name.clone(), tld.clone()), &offer);

        env.events().publish(
            (Symbol::new(&env, "make_sell_offer"),),
            (domain.owner, name, tld, price),
        );
    }

    pub fn cancel_sell_offer(env: Env, name: Bytes, tld: Bytes) {
        env.extend_me();
        let name: Bytes = name.get_root_name(&env);
        let offer: Offer = Self::get_sell_offer(env.clone(), name.clone(), tld.clone());
        offer.seller.require_auth();
        env.storage()
            .instance()
            .remove(&DataKey::Offer(name.clone(), tld.clone()));

        env.events().publish(
            (Symbol::new(&env, "cancel_sell_offer"),),
            (offer.seller, name, tld),
        );
    }

    pub fn get_sell_offer(env: Env, name: Bytes, tld: Bytes) -> Offer {
        env.extend_me();
        let name: Bytes = name.get_root_name(&env);
        env.storage()
            .instance()
            .get(&DataKey::Offer(name.clone(), tld.clone()))
            .unwrap_or_else(|| {
                panic_with_error!(&env, Error::NoOffer);
            })
    }

    pub fn buy_name(env: Env, name: Bytes, tld: Bytes, buyer: Address) {
        env.extend_me();
        buyer.require_auth();
        let name: Bytes = name.get_root_name(&env);
        let offer: Offer = Self::get_sell_offer(env.clone(), name.clone(), tld.clone());
        token::Client::new(&env, &env.storage().instance().get(&ASSET).unwrap()).transfer(
            &buyer,
            &offer.seller,
            &offer.price.into(),
        );
     
        // Transfer the domain to the buyer
        let mut domain: Domain = Self::get_name(env.clone(), name.clone(), tld.clone());
        domain.owner = buyer.clone();
        env.storage()
            .instance()
            .set(&DataKey::Name(name.clone(), tld.clone()), &domain);

        env.storage()
            .instance()
            .remove(&DataKey::Offer(name.clone(), tld.clone()));

        env.events().publish(
            (Symbol::new(&env, "buy_name"),),
            (buyer, name, tld, offer.price),
        );
    }
}
mod test;
