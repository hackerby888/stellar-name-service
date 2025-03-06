use soroban_sdk::{contracttype, Address, Bytes};


#[contracttype]
pub enum RecordKeys {
    Name(Bytes, Bytes),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Record {
    NullRecord,
    Name(Address),
    Ipfs(Bytes),
    Text(Bytes),
}