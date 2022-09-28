use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use lazy_static::lazy_static;
use std::env;
use uuid::Uuid;

lazy_static! {
    static ref MAILER: SmtpTransport = {
        let email_creds = Credentials::new(
            env::var("MAIL_USERNAME").expect("MAIL_USERNAME not set!"),
            env::var("MAIL_PASSWORD").expect("MAIL_PASSWORD not set!"),
        );
        SmtpTransport::starttls_relay(&env::var("MAIL_DOMAIN").expect("MAIL_DOMAIN not set!"))
            .unwrap()
            .credentials(email_creds)
            .build()
    };
}

pub fn send_verification_email(username: &str, email: &str, uuid: Uuid) -> Result<(), String> {
    let email = Message::builder()
        .to(format!("{} <{}>", username, email).parse().unwrap())
        .from(format!("Crystalium <{}@{}>", env::var("MAIL_USERNAME").unwrap(), env::var("DOMAIN").unwrap()).parse().unwrap())
        .subject("Email verification needed")
        .body(format!(
            "Hi {},\n
            Thanks for getting started with Crystalium!\n
            We need a little more information to complete your registration, including a confirmation of your email address.\n
            Click below to confirm your email address:\n
            https://{}/email_verification/{}\n
            If you have problems, please paste the above URL into your web browser.", 
            username,
            env::var("DOMAIN").expect("DOMAIN not set!"),
            uuid
        ))
        .unwrap();

    match MAILER.send(&email) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
