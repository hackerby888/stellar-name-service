use crate::{errors::Error, TLDS};
use soroban_sdk::{panic_with_error, Bytes, Env, Vec};

const DOT_IN_BYTE: u8 = 46;

pub trait BytesValidator {
    fn is_has_dot(&self) -> bool;
    fn validate_name(&self, env: &Env, allow_subdomain: bool);
    fn get_root_name(&self, env: &Env) -> Bytes;
    fn validate_tld(&self, env: &Env);
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

    fn is_has_dot(&self) -> bool {
        for byte in self.iter() {
            if byte == DOT_IN_BYTE {
                return true;
            }
        }

        false
    }

    fn get_root_name(&self, env: &Env) -> Bytes {
        if !self.is_has_dot() {
            return self.clone();
        }

        let mut root_name: Bytes = Bytes::new(env);
        let mut is_after_dot: bool = false;
        for byte in self.iter() {
            if byte == 46 {
                is_after_dot = true;
                continue;
            }
            if is_after_dot {
                root_name.push_back(byte);
            }
        }

        root_name
    }

    fn validate_tld(&self, env: &Env) {
        let tlds: Vec<Bytes> = env.storage().instance().get(&TLDS).unwrap();
        if !tlds.contains(self) {
            panic_with_error!(env, Error::TLDNotSupported);
        }
    }
}