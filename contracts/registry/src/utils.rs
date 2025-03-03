use crate::{errors::Error, TLDS};
use soroban_sdk::{panic_with_error, Bytes, Env, Vec};

pub fn validate_name(env: &Env, name: &Bytes) {
    if name.len() < 3 {
        panic_with_error!(env, Error::NameInvalid);
    }
    if name.len() > 24 {
        panic_with_error!(env, Error::NameInvalid);
    }
    if !name
        .iter()
        .all(|c| c.is_ascii_alphabetic() && c.is_ascii_lowercase())
    {
        panic_with_error!(env, Error::NameInvalid);
    }
}

pub fn validate_tld(env: &Env, tld: &Bytes) {
    let tlds: Vec<Bytes> = env.storage().instance().get(&TLDS).unwrap();
    if !tlds.contains(tld) {
        panic_with_error!(env, Error::TLDNotSupported);
    }
}
