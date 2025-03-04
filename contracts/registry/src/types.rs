use soroban_sdk::{contracttype, Address, Bytes};

#[contracttype]
pub enum DataKey {
    // domain.tld
    Name(Bytes, Bytes),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Domain {
    pub owner: Address,
    pub resolver: Address,
    pub expiry: u64,
}
