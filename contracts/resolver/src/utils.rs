use crate::{errors::Error, registry::DataKey};
use soroban_sdk::{panic_with_error, Bytes, Env};

const DOT_IN_BYTE: u8 = 46;

pub trait BytesValidator {
    fn validate_name(&self, env: &Env, allow_subdomain: bool);
}

impl BytesValidator for Bytes {
    fn validate_name(&self, env: &Env, allow_subdomain: bool) {
        if self.len() < 3 {
            panic_with_error!(env, Error::NameInvalid);
        }
        if self.len() > 24 {
            panic_with_error!(env, Error::NameInvalid);
        }

        if !allow_subdomain {
            for byte in self.iter() {
                if !(byte.is_ascii_alphabetic() && byte.is_ascii_lowercase()) {
                    panic_with_error!(&env, &Error::NameInvalid);
                }
            }
        } else {
            if self.first_unchecked() == DOT_IN_BYTE || self.last_unchecked() == DOT_IN_BYTE {
                panic_with_error!(env, Error::NameInvalid);
            }

            let mut previous_char_is_dot = false;
            let mut dots_count = 0;
            for byte in self.iter() {
                if byte == DOT_IN_BYTE {
                    if previous_char_is_dot {
                        panic_with_error!(env, Error::NameInvalid);
                    }
                    if dots_count >= 1 {
                        panic_with_error!(env, Error::NameInvalid);
                    }
                    previous_char_is_dot = true;
                    dots_count += 1;
                } else {
                    previous_char_is_dot = false;

                    if !(byte.is_ascii_alphabetic() && byte.is_ascii_lowercase()) {
                        panic_with_error!(&env, &Error::NameInvalid);
                    }
                }
            }
        }
    }

}

pub trait Base {
    fn extend_me(&self);
    fn delete_name(&self, name: &Bytes, tld: &Bytes);
}

impl Base for Env {
    fn extend_me(&self) {
        self.storage().instance().extend_ttl(17280, 17280 * 30);
    }

    fn delete_name(&self, name: &Bytes, tld: &Bytes) {
        self.storage()
            .instance()
            .remove(&DataKey::Name(name.clone(), tld.clone()));
    }
}