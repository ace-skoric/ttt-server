use actix_web::cookie::Key;
use dotenv::dotenv;
use lazy_static::lazy_static;
use log::info;
use std::env;

lazy_static! {
    pub static ref DOMAIN: String =
        env::var("DOMAIN").expect("DOMAIN environment variable not set!");
    pub static ref HOST: String = env::var("HOST").expect("HOST environment variable not set!");
    pub static ref PORT: u16 = env::var("PORT")
        .expect("PORT environment variable not set!")
        .parse::<u16>()
        .expect("Invalid port");
    pub static ref MAIL_SERVER: String =
        env::var("MAIL_SERVER").expect("MAIL_SERVER environment variable not set!");
    pub static ref MAIL_USERNAME: String =
        env::var("MAIL_USERNAME").expect("MAIL_USERNAME environment variable not set!");
    pub static ref MAIL_PASSWORD: String =
        env::var("MAIL_USERNAME").expect("MAIL_USERNAME environment variable not set!");
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set!");
    pub static ref REDIS_URL: String =
        env::var("REDIS_URL").expect("REDIS_URL environment variable not set!");
    pub static ref REDIS_SECRET: Key = Key::from(
        env::var("REDIS_SECRET")
            .expect("REDIS_SECRET environment variable not set!")
            .as_bytes()
    );
    pub static ref HASH_SECRET: String =
        env::var("HASH_SECRET").expect("HASH_SECRET environment variable not set!");
    pub static ref RUST_LOG: String =
        env::var("RUST_LOG").expect("RUST_LOG environment variable not set!");
}

pub fn init_env() {
    dotenv().ok();
    let _ = DOMAIN;
    let _ = HOST;
    let _ = PORT;
    let _ = MAIL_SERVER;
    let _ = MAIL_USERNAME;
    let _ = MAIL_PASSWORD;
    let _ = DATABASE_URL;
    let _ = REDIS_URL;
    let _ = REDIS_SECRET;
    let _ = HASH_SECRET;
    let _ = RUST_LOG;
    info!("Environment variables initialized successfuly");
}
