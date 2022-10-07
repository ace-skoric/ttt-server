use std::env;

use uuid::Uuid;

pub(crate) fn verification_email(
    username: &str,
    email: &str,
    sender: &str,
    uuid: &Uuid,
) -> lettre::Message {
    lettre::Message::builder()
        .to(format!("{} <{}>", username, email).parse().unwrap())
        .from(format!("Tic Tac Toe <{}>", sender).parse().unwrap())
        .subject("Email verification")
        .body(verification_email_raw(username, &uuid))
        .unwrap()
}

pub(crate) fn verification_email_raw(username: &str, uuid: &Uuid) -> String {
    format!(
        "Hi {},\n
        Thanks for getting started with our Tic Tac Toe!\n
        Click below to confirm your email address:\n
        https://{}/email_verification/{}\n
        If you have problems, please paste the above URL into your web browser.",
        username,
        env::var("DOMAIN").expect("DOMAIN not set!"),
        uuid
    )
}

pub(crate) fn verification_email_stdout(
    username: &str,
    email: &str,
    sender: &str,
    uuid: &Uuid,
) -> String {
    format!(
        "\nFrom: Tic Tac Toe <{}>\n
    To: {} <{}>\n
    Subject: Email verification\n
    {}
    ",
        sender,
        username,
        email,
        verification_email_raw(username, uuid)
    )
}
