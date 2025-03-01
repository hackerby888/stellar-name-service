#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Env, String,
};

const MAX_ASSET: i128 = 100000;

fn create_token_contract<'a>(
    e: &Env,
    admin: &Address,
) -> (token::Client<'a>, token::StellarAssetClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}

#[test]
fn test_basic_funtional() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (token, token_admin) = create_token_contract(&env, &admin);
    let contract_id = env.register(Registry, (&token_admin.address,));
    let client = RegistryClient::new(&env, &contract_id);

    let name = String::from_str(&env, "test");
    let owner = Address::generate(&env);
    let resolver = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET);
    client.set_resolver(&resolver);

    let is_registered = client.is_name_registered(&name);
    assert_eq!(is_registered, false);

    let is_ok = client.register_name(&name, &owner, &1);
    assert_eq!(is_ok, true);

    let is_registered = client.is_name_registered(&name);
    assert_eq!(is_registered, true);

    assert_eq!(client.get_owner(&name), owner);
    assert_eq!(client.get_resolver(&name), resolver);
    assert_eq!(client.is_name_expired(&name), false);
    assert_eq!(
        token.balance(&owner),
        i128::from(MAX_ASSET - i128::from(ASSET_PER_YEAR * 1))
    );
    assert_eq!(token.balance(&contract_id), i128::from(ASSET_PER_YEAR * 1));
}

#[test]
fn test_transfer_owner_ship() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let contract_id = env.register(Registry, (&token_admin.address,));
    let client = RegistryClient::new(&env, &contract_id);

    let name = String::from_str(&env, "test");
    let owner = Address::generate(&env);
    let resolver = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET);
    client.set_resolver(&resolver);

    client.register_name(&name, &owner, &1);

    let new_owner = Address::generate(&env);

    let is_ok = client.transfer(&name, &new_owner);
    assert_eq!(is_ok, true);

    assert_eq!(client.get_owner(&name), new_owner);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_get_owner_from_unregistered_name() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let contract_id = env.register(Registry, (&token_admin.address,));
    let client = RegistryClient::new(&env, &contract_id);
    let name = String::from_str(&env, "test");
    let owner = Address::generate(&env);

    assert_eq!(client.get_owner(&name), owner);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_get_resolver_from_unregistered_name() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let contract_id = env.register(Registry, (&token_admin.address,));
    let client = RegistryClient::new(&env, &contract_id);
    let name = String::from_str(&env, "test");

    let resolver = Address::generate(&env);
    assert_eq!(client.get_resolver(&name), resolver);
}

#[test]
fn test_name_should_be_expired() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let contract_id = env.register(Registry, (&token_admin.address,));
    let client = RegistryClient::new(&env, &contract_id);

    let name = String::from_str(&env, "test");
    let owner = Address::generate(&env);
    let resolver = Address::generate(&env);
    client.set_resolver(&resolver);

    token_admin.mint(&owner, &MAX_ASSET);
    client.register_name(&name, &owner, &1);

    env.ledger().set_timestamp(1000000000000000);

    assert_eq!(client.is_name_expired(&name), true);
}
