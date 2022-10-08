use lazy_static::lazy_static;
use regex::Regex;
use std::env;

pub(crate) fn is_valid_email(email: &str) -> bool {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$)").unwrap();
    }
    RE.is_match(email)
}

pub(crate) fn is_valid_username(username: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[\-_A-Za-z0-9]+$").unwrap();
    }
    username.len() <= 50 && RE.is_match(username)
}

pub(crate) fn is_valid_password(password: &str) -> bool {
    password.len() >= 6 && !password.is_empty()
}

pub(crate) fn hash_password(password: &str) -> String {
    let password = password.as_bytes();
    let secret = env::var("PASSWORD_HASH_SECRET").expect("PASSWORD_HASH_SECRET not set!");
    let secret = secret.as_bytes();
    let salt = b"thisissomesalt";
    let config = argon2::Config {
        variant: argon2::Variant::Argon2id,
        version: argon2::Version::Version13,
        mem_cost: 4096,
        time_cost: 32,
        lanes: 2,
        thread_mode: argon2::ThreadMode::Parallel,
        secret,
        ad: &[],
        hash_length: 16,
    };

    let hash = argon2::hash_encoded(password, salt, &config).expect("Error hashing password");

    hash
}

pub(crate) fn verify_password(password: &str, hash: &str) -> bool {
    let password = password.as_bytes();
    let secret = env::var("PASSWORD_HASH_SECRET").expect("HASH_SECRET not set");
    let secret = secret.as_bytes();

    let res = argon2::verify_encoded_ext(hash, password, secret, &[]);

    match res {
        Ok(res) => res,
        Err(_) => false,
    }
}
