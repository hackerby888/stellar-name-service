#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Ledger, Address, Env, String};

#[test]
fn test_basic_funtional() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );
    let contract_id = env.register(Registry, (&admin,));
    let client = RegistryClient::new(&env, &contract_id);

    let name = String::from_str(&env, "test");
    let owner = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );
    let resolver = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );

    let is_registered = client.is_name_registered(&name);
    assert_eq!(is_registered, false);

    let is_ok = client.register_name(&name, &owner, &resolver, &1);
    assert_eq!(is_ok, true);

    let is_registered = client.is_name_registered(&name);
    assert_eq!(is_registered, true);

    assert_eq!(client.get_owner(&name), owner);
    assert_eq!(client.get_resolver(&name), resolver);
    assert_eq!(client.is_name_expired(&name), false);
}

#[test]
fn test_transfer_owner_ship() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );
    let contract_id = env.register(Registry, (&admin,));
    let client = RegistryClient::new(&env, &contract_id);

    let name = String::from_str(&env, "test");
    let owner = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );
    let resolver = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );

    client.register_name(&name, &owner, &resolver, &1);

    let new_owner = Address::from_str(
        &env,
        "GA6QUE7FLHV6D2LSYTOBJ6CBA7OSDD2E3U6FPJX7DPBSJBP5YLQNUSDE",
    );
    
    let is_ok = client.transfer(&name, &new_owner);
    assert_eq!(is_ok, true);

    assert_eq!(client.get_owner(&name), new_owner);
}


#[test]
#[should_panic]
fn test_get_owner_from_unregistered_name() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );
    let contract_id = env.register(Registry, (&admin,));
    let client = RegistryClient::new(&env, &contract_id);
    let name = String::from_str(&env, "test");
    let owner = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );

    assert_eq!(client.get_owner(&name), owner);
}

#[test]
#[should_panic]
fn test_get_resolver_from_unregistered_name() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );
    let contract_id = env.register(Registry, (&admin,));
    let client = RegistryClient::new(&env, &contract_id);
    let name = String::from_str(&env, "test");

    let resolver = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );
    assert_eq!(client.get_resolver(&name), resolver);
}

#[test]
fn test_name_should_be_expired() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );
    let contract_id = env.register(Registry, (&admin,));
    let client = RegistryClient::new(&env, &contract_id);

    let name = String::from_str(&env, "test");
    let owner = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );
    let resolver = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );

    client.register_name(&name, &owner, &resolver, &1);

    env.ledger().set_timestamp(1000000000000000);

    assert_eq!(client.is_name_expired(&name), true);
}
