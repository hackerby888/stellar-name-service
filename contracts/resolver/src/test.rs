#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token, vec, Address, Env,
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

fn address_to_bytes(address: &Address) -> Bytes {
    let mut address_bytes_slice = [0u8; 56];
    address
        .to_string()
        .copy_into_slice(&mut address_bytes_slice);
    Bytes::from_slice(&address.env(), &address_bytes_slice)
}

#[test]
#[should_panic(expected = "Error(Contract, #9)")]
fn test_set_resolved_record_type_invalid() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let registry_id = env.register(
        registry::WASM,
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
    );
    let resolver_id = env.register(Resolver, (&admin, &registry_id));

    let resolver_client = ResolverClient::new(&env, &resolver_id);
    let registry_client = registry::Client::new(&env, &registry_id);

    let name = Bytes::from_slice(&env, "test".as_bytes());
    let owner = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET);
    registry_client.set_resolver(&resolver_id);
    registry_client.register_name(&name, &com_tld, &owner, &1);

    let address_to_be_resolved = Address::generate(&env);

    resolver_client.set_record(
        &name,
        &com_tld,
        &Bytes::from_slice(&env, "abi".as_bytes()),
        &address_to_bytes(&address_to_be_resolved),
    );

    assert_eq!(
        resolver_client.resolve_name(&name, &com_tld),
        Record::Name(address_to_be_resolved)
    );
}

#[test]
fn test_get_resolved_record_type_name() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let registry_id = env.register(
        registry::WASM,
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
    );
    let resolver_id = env.register(Resolver, (&admin, &registry_id));

    let resolver_client = ResolverClient::new(&env, &resolver_id);
    let registry_client = registry::Client::new(&env, &registry_id);

    let name = Bytes::from_slice(&env, "test".as_bytes());
    let owner = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET);
    registry_client.set_resolver(&resolver_id);
    registry_client.register_name(&name, &com_tld, &owner, &1);

    let address_to_be_resolved = Address::generate(&env);

    resolver_client.set_record(
        &name,
        &com_tld,
        &Bytes::from_slice(&env, "name".as_bytes()),
        &address_to_bytes(&address_to_be_resolved),
    );

    assert_eq!(
        resolver_client.resolve_name(&name, &com_tld),
        Record::Name(address_to_be_resolved)
    );
    assert_ne!(
        resolver_client.resolve_name(&name, &com_tld),
        Record::Name(owner)
    );
}

#[test]
fn test_get_resolved_record_type_ipfs() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let registry_id = env.register(
        registry::WASM,
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
    );
    let resolver_id = env.register(Resolver, (&admin, &registry_id));

    let resolver_client = ResolverClient::new(&env, &resolver_id);
    let registry_client = registry::Client::new(&env, &registry_id);

    let name = Bytes::from_slice(&env, "test".as_bytes());
    let owner = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET);
    registry_client.set_resolver(&resolver_id);
    registry_client.register_name(&name, &com_tld, &owner, &1);

    let my_hash = Bytes::from_slice(&env, "this is my hash".as_bytes());

    resolver_client.set_record(
        &name,
        &com_tld,
        &Bytes::from_slice(&env, "ipfs".as_bytes()),
        &my_hash,
    );

    assert_eq!(
        resolver_client.resolve_name(&name, &com_tld),
        Record::Ipfs(my_hash)
    );
}

#[test]
fn test_get_resolved_record_type_text() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let registry_id = env.register(
        registry::WASM,
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
    );
    let resolver_id = env.register(Resolver, (&admin, &registry_id));

    let resolver_client = ResolverClient::new(&env, &resolver_id);
    let registry_client = registry::Client::new(&env, &registry_id);

    let name = Bytes::from_slice(&env, "test".as_bytes());
    let owner = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET);
    registry_client.set_resolver(&resolver_id);
    registry_client.register_name(&name, &com_tld, &owner, &1);

    let my_text = Bytes::from_slice(&env, "this is my text".as_bytes());

    resolver_client.set_record(
        &name,
        &com_tld,
        &Bytes::from_slice(&env, "text".as_bytes()),
        &my_text,
    );

    assert_eq!(
        resolver_client.resolve_name(&name, &com_tld),
        Record::Text(my_text)
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_get_record_from_registered_but_not_set() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let registry_id = env.register(
        registry::WASM,
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
    );
    let resolver_id = env.register(Resolver, (&admin, &registry_id));

    let resolver_client = ResolverClient::new(&env, &resolver_id);
    let registry_client = registry::Client::new(&env, &registry_id);

    let name = Bytes::from_slice(&env, "test".as_bytes());
    let owner = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET);
    registry_client.set_resolver(&resolver_id);
    registry_client.register_name(&name, &com_tld, &owner, &1);
    resolver_client.resolve_name(&name, &com_tld);
}

#[test]
#[should_panic(expected = "Error(Contract, #5)")]
fn test_get_resolved_data_from_not_registered() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let registry_id = env.register(
        registry::WASM,
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
    );
    let resolver_id = env.register(Resolver, (&admin, &registry_id));

    let resolver_client = ResolverClient::new(&env, &resolver_id);

    let name = Bytes::from_slice(&env, "test".as_bytes());

    resolver_client.resolve_name(&name, &com_tld);
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_set_resolved_data_from_not_registered() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let registry_id = env.register(
        registry::WASM,
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
    );
    let resolver_id = env.register(Resolver, (&admin, &registry_id));

    let resolver_client = ResolverClient::new(&env, &resolver_id);

    let name = Bytes::from_slice(&env, "test".as_bytes());

    let address_to_be_resolved = Address::generate(&env);

    resolver_client.set_record(
        &name,
        &com_tld,
        &Bytes::from_slice(&env, "name".as_bytes()),
        &address_to_bytes(&address_to_be_resolved),
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #2)")]
fn test_set_resolved_wrong_name_but_there_is_another_is_valid() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let registry_id = env.register(
        registry::WASM,
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
    );
    let resolver_id = env.register(Resolver, (&admin, &registry_id));

    let resolver_client = ResolverClient::new(&env, &resolver_id);
    let registry_client = registry::Client::new(&env, &registry_id);

    let name = Bytes::from_slice(&env, "test".as_bytes());
    let owner = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET);
    registry_client.set_resolver(&resolver_id);
    registry_client.register_name(&name, &com_tld, &owner, &1);

    let address_to_be_resolved = Address::generate(&env);

    resolver_client.set_record(
        &name,
        &com_tld,
        &Bytes::from_slice(&env, "name".as_bytes()),
        &address_to_bytes(&address_to_be_resolved),
    );

    let name2 = Bytes::from_slice(&env, "nametwo".as_bytes());

    resolver_client.set_record(
        &name2,
        &com_tld,
        &Bytes::from_slice(&env, "name".as_bytes()),
        &address_to_bytes(&address_to_be_resolved),
    );
}

#[test]
#[should_panic(expected = "Error(Contract, #4)")]
fn test_name_expired() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let (_, token_admin) = create_token_contract(&env, &admin);
    let com_tld = Bytes::from_slice(&env, "com".as_bytes());
    let registry_id = env.register(
        registry::WASM,
        (&admin, &token_admin.address, vec![&env, com_tld.clone()]),
    );
    let resolver_id = env.register(Resolver, (&admin, &registry_id));

    let resolver_client = ResolverClient::new(&env, &resolver_id);
    let registry_client = registry::Client::new(&env, &registry_id);

    let name = Bytes::from_slice(&env, "test".as_bytes());
    let owner = Address::generate(&env);

    token_admin.mint(&owner, &MAX_ASSET);
    registry_client.set_resolver(&resolver_id);
    registry_client.register_name(&name, &com_tld, &owner, &1);
    let address_to_be_resolved = Address::generate(&env);

    let name_sub = Bytes::from_slice(&env, "feiyu.test".as_bytes());
    resolver_client.set_record(
        &name_sub,
        &com_tld,
        &Bytes::from_slice(&env, "name".as_bytes()),
        &address_to_bytes(&address_to_be_resolved),
    );

    assert_eq!(
        resolver_client.is_name_has_record(&name_sub, &com_tld),
        true
    );
    env.ledger().set_timestamp(1000000000000000);
    resolver_client.resolve_name(&name_sub, &com_tld);
}
