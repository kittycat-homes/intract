use std::error::Error;

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};

use crate::config::CONFIG;

#[derive(Debug)]
pub struct HashAndSalt {
    pub hash: String,
    pub salt: String,
}

/// hash and salt a password, for initialization
pub fn hash_password_and_generate_salt(pass: &str) -> Result<HashAndSalt, Box<dyn Error>> {
    // default params for argon 2
    let salt = SaltString::generate(&mut OsRng).to_string();
    let hash = hash_password(pass, &salt)?;
    Ok(HashAndSalt { hash, salt })
}

/// used to hash a password when the salt is already known.
/// this is useful when for example handing out new session keys
pub fn hash_password(pass: &str, salt: &str) -> Result<String, Box<dyn Error>> {
    let pepper = CONFIG.server.argon2_pepper.clone().into_bytes();
    let argon2 = Argon2::new_with_secret(
        &pepper,
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        argon2::Params::default(),
    )?;

    let salt: SaltString = SaltString::from_b64(salt)?;
    let hash = argon2.hash_password(pass.as_bytes(), &salt)?;
    Ok(hash.serialize().to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_hashing() {
        let pass = String::from("supersecret");
        assert!(hash_password_and_generate_salt(&pass).is_ok());
    }

    #[test]
    fn check_hashes_unequal() {
        let pass = String::from("hunter2");

        let one = hash_password_and_generate_salt(&pass).unwrap();
        let two = hash_password_and_generate_salt(&pass).unwrap();

        println!("hashes: \n{:#?} {:#?}", one, two);
        assert!(one.hash != two.hash);
        assert!(one.salt != two.salt);
    }
}
