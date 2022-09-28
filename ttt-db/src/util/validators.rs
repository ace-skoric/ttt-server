use argonautica::{Hasher, Verifier};
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
    let mut hasher = Hasher::default();
    hasher
        .with_password(password)
        .with_secret_key(env::var("HASH_SECRET").expect("Hash secret not set"))
        // .with_salt("somesalt")
        .hash()
        .unwrap()
}

pub(crate) fn verify_password(password: &str, hash: &str) -> bool {
    let mut verifier = Verifier::default();
    verifier
        .with_hash(hash)
        .with_password(password)
        .with_secret_key(env::var("HASH_SECRET").expect("Hash secret not set"))
        // .with_salt("somesalt")
        .verify()
        .unwrap()
}
