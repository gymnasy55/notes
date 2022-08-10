use std::num::NonZeroU32;

use log::error;
use ring::rand::SecureRandom;
use ring::{digest, error, pbkdf2, rand};

use crate::helpers::vector::vec_to_array;

pub const CREDENTIAL_SIZE: usize = digest::SHA256_OUTPUT_LEN;
const N_ITER: Option<NonZeroU32> = NonZeroU32::new(10_000);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HashSalt {
    pub hash: String,
    pub salt: String,
}

pub fn encrypt(password: String) -> HashSalt {
    encrypt_with_salt(password, generate_salt().unwrap())
}

pub fn encrypt_with_salt(password: String, salt: [u8; CREDENTIAL_SIZE]) -> HashSalt {
    let mut hash = [0u8; CREDENTIAL_SIZE];

    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        N_ITER.unwrap(),
        &salt,
        password.as_bytes(),
        &mut hash,
    );

    HashSalt {
        salt: base64::encode(salt),
        hash: base64::encode(hash),
    }
}

#[allow(dead_code)]
pub fn verify(password: String, base64_salt: String, base64_hash: String) -> bool {
    pbkdf2::verify(
        pbkdf2::PBKDF2_HMAC_SHA256,
        N_ITER.unwrap(),
        &base64_to_sha256_array(base64_salt),
        password.as_bytes(),
        &base64_to_sha256_array(base64_hash),
    )
    .is_ok()
}

#[allow(dead_code)]
fn base64_to_sha256_array(base64: String) -> [u8; CREDENTIAL_SIZE] {
    match base64::decode(base64) {
        Ok(v) => vec_to_array::<_, CREDENTIAL_SIZE>(v),
        Err(e) => {
            error!("{}", e);
            [0u8; CREDENTIAL_SIZE]
        }
    }
}

fn generate_salt() -> Result<[u8; CREDENTIAL_SIZE], error::Unspecified> {
    let mut salt: [u8; CREDENTIAL_SIZE] = [0u8; CREDENTIAL_SIZE];

    match rand::SystemRandom::new().fill(&mut salt) {
        Ok(()) => Ok(salt),
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::password::{encrypt_with_salt, verify, HashSalt, CREDENTIAL_SIZE};

    const SALT: [u8; CREDENTIAL_SIZE] = [
        249, 186, 110, 86, 188, 130, 204, 148, 153, 54, 33, 213, 158, 143, 216, 89, 22, 38, 101,
        72, 216, 197, 242, 126, 39, 71, 91, 108, 87, 220, 228, 53,
    ];
    const HASH: &str = "shZHGkXULBZvk4Yvo0JUHtgSjUMPm1hKlRObZDBOfdQ=";
    const BASE64_SALT: &str = "+bpuVryCzJSZNiHVno/YWRYmZUjYxfJ+J0dbbFfc5DU=";

    #[test]
    fn encrypt_with_salt_should_return_correct() {
        let password = String::from("test");

        let expected = HashSalt {
            hash: String::from(HASH),
            salt: String::from(BASE64_SALT),
        };
        let actual = encrypt_with_salt(password, SALT);

        assert_eq!(actual, expected)
    }

    #[test]
    fn encrypt_with_salt_should_return_wrong() {
        let password = String::from("tset");

        let expected = HashSalt {
            hash: String::from(HASH),
            salt: String::from(BASE64_SALT),
        };
        let actual = encrypt_with_salt(password, SALT);

        assert_ne!(actual, expected)
    }

    #[test]
    fn verify_should_return_true() {
        let password = String::from("test");
        let hash_salt = HashSalt {
            hash: String::from(HASH),
            salt: String::from(BASE64_SALT),
        };

        let expected = true;
        let actual = verify(password, hash_salt.salt, hash_salt.hash);

        assert_eq!(actual, expected)
    }

    #[test]
    fn verify_should_return_false() {
        let password = String::from("tset");
        let hash_salt = HashSalt {
            hash: String::from(HASH),
            salt: String::from(BASE64_SALT),
        };

        let expected = false;
        let actual = verify(password, hash_salt.salt, hash_salt.hash);

        assert_eq!(actual, expected)
    }
}
