use actix_web::cookie::Key;
use dotenv::dotenv;
use lazy_static::lazy_static;
use log::{error, info, warn};
use std::{env, process::exit};

lazy_static! {
    pub static ref HOST: String = env::var("HOST").unwrap_or("localhost".to_string());
    pub static ref PORT: u16 = env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse::<u16>()
        .unwrap_or_else(|_| {
            error!("Error parsing PORT env variable!");
            exit(1);
        });
    pub static ref DOMAIN: String =
        env::var("DOMAIN").unwrap_or(format!("{}:{}", *HOST, *PORT).to_string());
    pub static ref STDOUT_MAIL: bool = env::var("STDOUT_MAIL")
        .unwrap_or("1".to_string())
        .parse::<bool>()
        .unwrap_or_else(|_| {
            error!("Error parsing STDOUT_MAIL env variable!");
            exit(1);
        });
    pub static ref MAIL_SERVER: String = if *STDOUT_MAIL {
        env::var("MAIL_SERVER").unwrap_or("localhost".to_string())
    } else {
        env::var("MAIL_SERVER").unwrap_or_else(|_| {
            error!("MAIL_SERVER environment variable not set!");
            exit(1);
        })
    };
    pub static ref MAIL_USERNAME: String = if *STDOUT_MAIL {
        env::var("MAIL_USERNAME").unwrap_or("ttt@localhost".to_string())
    } else {
        env::var("MAIL_USERNAME").unwrap_or_else(|_| {
            error!("MAIL_USERNAME environment variable not set!");
            exit(1);
        })
    };
    pub static ref MAIL_PASSWORD: String = if *STDOUT_MAIL {
        env::var("MAIL_PASSWORD").unwrap_or("password".to_string())
    } else {
        env::var("MAIL_PASSWORD").unwrap_or_else(|_| {
            error!("MAIL_PASSWORD environment variable not set!");
            exit(1);
        })
    };
    pub static ref DATABASE_URL: String =
        env::var("DATABASE_URL").unwrap_or_else(|_| {
            warn!("DATABASE_URL not set! Setting to default (postgresql://postgres:password@db:5432/tictactoe)");
            "postgresql://postgres:password@db:5432/tictactoe".to_string()
        });
    pub static ref REDIS_URL: String =
        env::var("REDIS_URL").unwrap_or_else(|_| {
            warn!("REDIS_URL not set! Setting to default (redis://rdb:6379)");
            "redis://rdb:6379".to_string()
        });
    pub static ref SESSION_SECRET: Key = {
        let key: String = env::var("SESSION_SECRET").unwrap_or_else(|_| {
            warn!("SESSION_SECRET environment variable not set!");
            warn!("Generating random session key...");
            "".to_string()
        });
        if key.is_empty() {
            Key::generate()
        } else if key.as_bytes().len() < 64 {
            warn!("SESSION_SECRET must be at least 512 bytes long!");
            warn!("Generating random session key...");
            Key::generate()
        } else {
            Key::from(key.as_bytes())
        }
    };
    pub static ref PASSWORD_HASH_SECRET: String =
        env::var("PASSWORD_HASH_SECRET").unwrap_or_else(|_| {
            error!("PASSWORD_HASH_SECRET environment variable not set!");
            exit(1);
        });
}

pub fn init_env() {
    dotenv().ok();
    env_logger::init();
    let _x = &*HOST;
    let _x = &*PORT;
    let _x = &*DOMAIN;
    let _x = &*STDOUT_MAIL;
    let _x = &*MAIL_SERVER;
    let _x = &*MAIL_USERNAME;
    let _x = &*MAIL_PASSWORD;
    let _x = &*DATABASE_URL;
    let _x = &*REDIS_URL;
    let _x = &*SESSION_SECRET;
    let _x = &*PASSWORD_HASH_SECRET;
    info!("Environment variables initialized successfuly!");
}
