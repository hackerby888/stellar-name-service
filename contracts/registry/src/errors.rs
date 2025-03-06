use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NameAlreadyRegistered = 1,
    NameNotRegistered = 2,
    NameInvalid = 3,
    NameExpired = 4,
    NameHasNoRecord = 5,
    ResolverAlreadySet = 6,
    TLDNotSupported = 7,
    NoOffer = 8,
    RecordTypeInvalid = 9,
}
