#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Events, Ledger},
    token, vec, Address, Bytes, Env, IntoVal, Val,
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

fn get_events_by_contract_id<'a>(
    e: &'a Env,
    contract_id: &Address,
) -> Vec<(Address, Vec<Val>, Val)> {
    let all_events: Vec<(Address, Vec<Val>, Val)> = e.events().all();
    let mut contract_events: Vec<(Address, Vec<Val>, Val)> = vec![&e];
    for event in all_events.iter() {
        if event.0 == contract_id.clone() {
            contract_events.push_back(event.clone());
        }
    }
    contract_events
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
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
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

    assert_eq!(
        get_events_by_contract_id(&env, &contract_id),
        vec![
            &env,
            (
                contract_id.clone(),
                (Symbol::new(&env, "register_name"),).into_val(&env),
                (owner.clone(), name.clone(), com_tld.clone(), 1u64).into_val(&env)
            )
        ]
    );

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
fn test_make_sell_offer() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (token, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let contract_id = env.register(
        Registry,
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
    );
    let client = RegistryClient::new(&env, &contract_id);

    let name = Bytes::from_slice(&env, "ttt".as_bytes());
    let owner = Address::generate(&env);
    let resolver = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET_AMOUNT);
    client.set_resolver(&resolver);

    client.register_name(&name, &com_tld, &owner, &1);

    client.make_sell_offer(&name, &com_tld, &10);

    assert_eq!(
        get_events_by_contract_id(&env, &contract_id),
        vec![
            &env,
            (
                contract_id.clone(),
                (Symbol::new(&env, "make_sell_offer"),).into_val(&env),
                (owner.clone(), name.clone(), com_tld.clone(), 10u64).into_val(&env),
            )
        ]
    );

    let offer: Offer = client.get_sell_offer(&name, &com_tld);
    assert_eq!(offer.seller, owner);

    let buyer = Address::generate(&env);
    token_admin.mint(&buyer, &MAX_ASSET_AMOUNT);

    client.buy_name(&name, &com_tld, &buyer);

    assert_eq!(
        get_events_by_contract_id(&env, &contract_id),
        vec![
            &env,
            (
                contract_id.clone(),
                (Symbol::new(&env, "buy_name"),).into_val(&env),
                (buyer.clone(), name.clone(), com_tld.clone(), 10u64).into_val(&env),
            )
        ]
    );

    assert_eq!(client.get_owner(&name, &com_tld), buyer);
    assert_eq!(token.balance(&owner), i128::from(MAX_ASSET_AMOUNT - 10));
}

#[test]
#[should_panic(expected = "Error(Contract, #8)")]
fn test_cancel_sell_offer() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let contract_id = env.register(
        Registry,
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
    );
    let client = RegistryClient::new(&env, &contract_id);

    let name = Bytes::from_slice(&env, "ttt".as_bytes());
    let owner = Address::generate(&env);
    let resolver = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET_AMOUNT);
    client.set_resolver(&resolver);

    client.register_name(&name, &com_tld, &owner, &1);

    client.make_sell_offer(&name, &com_tld, &10);

    client.cancel_sell_offer(&name, &com_tld);

    assert_eq!(
        get_events_by_contract_id(&env, &contract_id),
        vec![
            &env,
            (
                contract_id.clone(),
                (Symbol::new(&env, "cancel_sell_offer"),).into_val(&env),
                (owner.clone(), name.clone(), com_tld.clone()).into_val(&env),
            )
        ]
    );

    client.get_sell_offer(&name, &com_tld);
}

#[test]
fn test_check_sub_domain() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let contract_id = env.register(
        Registry,
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
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

    let sub_name = Bytes::from_slice(&env, "me.ttt".as_bytes());

    assert_eq!(client.is_name_registered(&sub_name, &com_tld), true);
    assert_eq!(client.get_owner(&sub_name, &com_tld), owner);
    assert_eq!(client.is_name_expired(&sub_name, &com_tld), false);
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
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
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
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
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
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
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
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
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
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
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
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
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
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
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
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
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

#[test]
fn test_register_name_that_expired() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let contract_id = env.register(
        Registry,
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
    );
    let client = RegistryClient::new(&env, &contract_id);

    let name = Bytes::from_slice(&env, "test".as_bytes());
    let owner = Address::generate(&env);
    let resolver = Address::generate(&env);
    client.set_resolver(&resolver);

    token_admin.mint(&owner, &MAX_ASSET_AMOUNT);
    client.register_name(&name, &com_tld, &owner, &1);

    env.ledger().set_timestamp(1000000000000000);

    client.register_name(&name, &com_tld, &owner, &1);
}
