use soroban_sdk::{contracttype, Address, Bytes};

#[contracttype]
pub enum DataKey {
    // domain.tld
    Name(Bytes, Bytes),
    Offer(Bytes, Bytes),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Domain {
    pub owner: Address,
    pub resolver: Address,
    pub expiry: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Offer {
    pub seller: Address,
    pub name: Bytes,
    pub tld: Bytes,
    pub price: u64,
}
