#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_get_resolved_name() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );

    let registry_id = env.register(registry::WASM, (&admin,));
    let resolver_id = env.register(Resolver, (&registry_id,));

    let resolver_client = ResolverClient::new(&env, &resolver_id);
    let registry_client = registry::Client::new(&env, &registry_id);

    let name = String::from_str(&env, "test");
    let owner = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );

    registry_client.register_name(&name, &owner, &resolver_id, &1);

    let address_to_be_resolved = Address::from_str(
        &env,
        "GA6O4HS4WYRLQJ7WNABBYJ2WMO6TT3NC6NTHJFTYRM6T3DOFYIT43LOJ",
    );

    resolver_client.set_resolve_data(&name, &address_to_be_resolved);

    assert_eq!(resolver_client.resolve_name(&name), address_to_be_resolved);
    assert_ne!(resolver_client.resolve_name(&name), owner);
}

#[test]
#[should_panic]
fn test_get_resolved_data_from_registered_but_not_set() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );

    let registry_id = env.register(registry::WASM, (&admin,));
    let resolver_id = env.register(Resolver, (&admin, &registry_id));

    let resolver_client = ResolverClient::new(&env, &resolver_id);
    let registry_client = registry::Client::new(&env, &registry_id);

    let name = String::from_str(&env, "test");
    let owner = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );

    registry_client.register_name(&name, &owner, &resolver_id, &1);
    resolver_client.resolve_name(&name);
}

#[test]
#[should_panic]
fn test_get_resolved_data_from_not_registered() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );

    let registry_id = env.register(registry::WASM, (&admin,));
    let resolver_id = env.register(Resolver, (&admin, &registry_id));

    let resolver_client = ResolverClient::new(&env, &resolver_id);

    let name = String::from_str(&env, "test");

    resolver_client.resolve_name(&name);
}

#[test]
#[should_panic]
fn test_set_resolved_data_from_not_registered() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );

    let registry_id = env.register(registry::WASM, (&admin,));
    let resolver_id = env.register(Resolver, (&admin, &registry_id));

    let resolver_client = ResolverClient::new(&env, &resolver_id);

    let name = String::from_str(&env, "test");

    let address_to_be_resolved = Address::from_str(
        &env,
        "GA6O4HS4WYRLQJ7WNABBYJ2WMO6TT3NC6NTHJFTYRM6T3DOFYIT43LOJ",
    );

    resolver_client.set_resolve_data(&name, &address_to_be_resolved);
}

#[test]
#[should_panic]
fn test_set_resolved_wrong_name_but_there_is_another_is_valid() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );

    let registry_id = env.register(registry::WASM, (&admin,));
    let resolver_id = env.register(Resolver, (&admin, &registry_id));

    let resolver_client = ResolverClient::new(&env, &resolver_id);
    let registry_client = registry::Client::new(&env, &registry_id);

    let name = String::from_str(&env, "test");
    let owner = Address::from_str(
        &env,
        "GDW7HH4TXOF5IOJOXOV2JLIOLRNOT6PVEXP47GIBUMHN6TIFMQ2FMROQ",
    );

    registry_client.register_name(&name, &owner, &resolver_id, &1);

    let address_to_be_resolved = Address::from_str(
        &env,
        "GA6O4HS4WYRLQJ7WNABBYJ2WMO6TT3NC6NTHJFTYRM6T3DOFYIT43LOJ",
    );

    resolver_client.set_resolve_data(&name, &address_to_be_resolved);

    let name2 = String::from_str(&env, "test2");

    resolver_client.resolve_name(&name2);
}
