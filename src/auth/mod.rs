use argon2::Argon2;
use password_hash::{SaltString, rand_core::OsRng};

use crate::error::TangoError;

pub fn hash_passwd(passwd: &str) -> Result<String, TangoError> {
    let saltstr = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let hashed = [0u8];

    // let passwd_hash = argon2.hash_password_into(passwd.as_bytes(), &saltstr , &mut hashed);
    Ok(String::from_utf8(hashed.to_vec()).unwrap())
} 
