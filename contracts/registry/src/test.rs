#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    vec, Env,
};

const MAX_ASSET_AMOUNT: i128 = 100000;

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
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let contract_id = env.register(
        Registry,
        (&token_admin.address, vec![&env, com_tld.clone()]),
    );
    let client = RegistryClient::new(&env, &contract_id);

    let name = Bytes::from_slice(&env, "ttt".as_bytes());
    let owner = Address::generate(&env);
    let resolver = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET_AMOUNT);
    client.set_resolver(&resolver);

    let is_registered = client.is_name_registered(&name, &com_tld);
    assert_eq!(is_registered, false);

    client.register_name(&name, &com_tld, &owner, &1);

    let is_registered = client.is_name_registered(&name, &com_tld);
    assert_eq!(is_registered, true);

    assert_eq!(client.get_owner(&name, &com_tld), owner);
    assert_eq!(client.get_resolver(), resolver);
    assert_eq!(client.is_name_expired(&name, &com_tld), false);
    assert_eq!(
        token.balance(&owner),
        i128::from(MAX_ASSET_AMOUNT - i128::from(ASSET_AMOUNT_PER_YEAR * 1))
    );
    assert_eq!(
        token.balance(&contract_id),
        i128::from(ASSET_AMOUNT_PER_YEAR * 1)
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_name_invalid_because_uppercase() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let contract_id = env.register(
        Registry,
        (&token_admin.address, vec![&env, com_tld.clone()]),
    );
    let client = RegistryClient::new(&env, &contract_id);

    let name = Bytes::from_slice(&env, "DOMAIN".as_bytes());
    let owner = Address::generate(&env);
    let resolver = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET_AMOUNT);
    client.set_resolver(&resolver);
    client.register_name(&name, &com_tld, &owner, &1);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_name_invalid_because_not_ascii_alphabetic() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let contract_id = env.register(
        Registry,
        (&token_admin.address, vec![&env, com_tld.clone()]),
    );
    let client = RegistryClient::new(&env, &contract_id);

    let name = Bytes::from_slice(&env, "domain-".as_bytes());
    let owner = Address::generate(&env);
    let resolver = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET_AMOUNT);
    client.set_resolver(&resolver);
    client.register_name(&name, &com_tld, &owner, &1);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_name_invalid_because_too_short() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let contract_id = env.register(
        Registry,
        (&token_admin.address, vec![&env, com_tld.clone()]),
    );
    let client = RegistryClient::new(&env, &contract_id);

    let name = Bytes::from_slice(&env, "d0".as_bytes());
    let owner = Address::generate(&env);
    let resolver = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET_AMOUNT);
    client.set_resolver(&resolver);
    client.register_name(&name, &com_tld, &owner, &1);
}

#[test]
#[should_panic(expected = "Error(Contract, #3)")]
fn test_name_invalid_because_too_long() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let contract_id = env.register(
        Registry,
        (&token_admin.address, vec![&env, com_tld.clone()]),
    );
    let client = RegistryClient::new(&env, &contract_id);

    let name = Bytes::from_slice(&env, "thisdomainisverylongsoitshouldbeinvalid".as_bytes());
    let owner = Address::generate(&env);
    let resolver = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET_AMOUNT);
    client.set_resolver(&resolver);
    client.register_name(&name, &com_tld, &owner, &1);
}

#[test]
fn test_transfer_owner_ship() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let contract_id = env.register(
        Registry,
        (&token_admin.address, vec![&env, com_tld.clone()]),
    );
    let client = RegistryClient::new(&env, &contract_id);

    let name = Bytes::from_slice(&env, "test".as_bytes());
    let owner = Address::generate(&env);
    let resolver = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET_AMOUNT);
    client.set_resolver(&resolver);

    client.register_name(&name, &com_tld, &owner, &1);

    let new_owner = Address::generate(&env);

    client.transfer(&name, &com_tld, &new_owner);

    assert_eq!(client.get_owner(&name, &com_tld), new_owner);
}

#[test]
#[should_panic(expected = "Error(Contract, #7)")]
fn test_register_unsupported_tld() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let net_tld = Bytes::from_slice(&env, "net".as_bytes());
    let contract_id = env.register(
        Registry,
        (&token_admin.address, vec![&env, com_tld.clone()]),
    );
    let client = RegistryClient::new(&env, &contract_id);

    let name = Bytes::from_slice(&env, "test".as_bytes());
    let owner = Address::generate(&env);
    
    client.register_name(&name, &net_tld, &owner, &1)
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_get_owner_from_unregistered_name() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let contract_id = env.register(
        Registry,
        (&token_admin.address, vec![&env, com_tld.clone()]),
    );
    let client = RegistryClient::new(&env, &contract_id);
    let name = Bytes::from_slice(&env, "test".as_bytes());
    let owner = Address::generate(&env);

    assert_eq!(client.get_owner(&name, &com_tld), owner);
}

#[test]
fn test_name_should_be_expired() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let contract_id = env.register(
        Registry,
        (&token_admin.address, vec![&env, com_tld.clone()]),
    );
    let client = RegistryClient::new(&env, &contract_id);

    let name = Bytes::from_slice(&env, "test".as_bytes());
    let owner = Address::generate(&env);
    let resolver = Address::generate(&env);
    client.set_resolver(&resolver);

    token_admin.mint(&owner, &MAX_ASSET_AMOUNT);
    client.register_name(&name, &com_tld, &owner, &1);

    env.ledger().set_timestamp(1000000000000000);

    assert_eq!(client.is_name_registered(&name, &com_tld), true);
    assert_eq!(client.is_name_expired(&name, &com_tld), true);
}
