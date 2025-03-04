use soroban_sdk::{contracttype, Bytes};


#[contracttype]
pub enum DataKey {
    Name(Bytes, Bytes),
}