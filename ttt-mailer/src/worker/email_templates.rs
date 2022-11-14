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
    let pt1 = format!("Hi {},\n", username);
    let pt2 = "Thanks for getting started with our Tic Tac Toe!\n";
    let pt3 = "Click below to confirm your email address:\n";
    let pt4 = format!(
        "http://{}/email_verification/{}\n",
        env::var("Domain").expect("DOMAIN  not set!"),
        uuid
    );
    let pt5 = "If you have problems, please paste the above URL into your web browser.";
    pt1 + pt2 + pt3 + &pt4 + pt5
}

pub(crate) fn verification_email_stdout(
    username: &str,
    email: &str,
    sender: &str,
    uuid: &Uuid,
) -> String {
    format!(
        "
    From: Tic Tac Toe <{}>
    To: {} <{}>
    Subject: Email verification\n
        {}
    ",
        sender,
        username,
        email,
        verification_email_raw(username, uuid)
    )
}
