use actix::fut::wrap_future;
use actix::{Actor, AsyncContext, Context, Handler};
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport,
    Tokio1Executor,
};
use log::{error, info, warn};
use std::{env, process::exit, sync::Arc};
use uuid::Uuid;

use super::{email_templates::*, messages::*};

#[derive(Debug, Clone)]
pub struct MailWorker {
    mailer: Arc<Option<AsyncSmtpTransport<Tokio1Executor>>>,
    username: String,
}

impl MailWorker {
    pub async fn new() -> Self {
        let stdout = env::var("STDOUT_MAIL").unwrap().parse::<bool>().unwrap();
        let server = env::var("MAIL_SERVER").unwrap_or("localhost".to_string());
        let username = env::var("MAIL_USERNAME").unwrap_or("user@localhost".to_string());
        let password = env::var("MAIL_PASSWORD").unwrap_or("password".to_string());
        let creds = Credentials::new(username.clone(), password);
        let mailer: Option<AsyncSmtpTransport<Tokio1Executor>>;
        if stdout {
            mailer = None;
        } else {
            let transport: AsyncSmtpTransport<Tokio1Executor> =
                AsyncSmtpTransport::<Tokio1Executor>::relay(&server)
                    .unwrap_or_else(|err| {
                        error!("Error connecting to mail server: {:?}", err);
                        exit(1);
                    })
                    .credentials(creds)
                    .build();
            let test = transport.test_connection().await;
            if test.is_err() {
                error!("Error connecting to mail server: {:?}", test.unwrap_err());
                exit(1);
            } else if !test.unwrap() {
                error!("Error connecting to mail server: Unhandled error");
                exit(1);
            }
            mailer = Some(transport);
        }
        let mailer = Arc::new(mailer);
        Self { mailer, username }
    }
}

impl Actor for MailWorker {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
        info!("Mail worker is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        info!("Mail worker is stopped");
    }
}

impl Handler<SendVerificationEmail> for MailWorker {
    type Result = ();
    fn handle(&mut self, msg: SendVerificationEmail, ctx: &mut Self::Context) -> Self::Result {
        let username = msg.username;
        let email = msg.email;
        let uuid = msg.uuid;
        let mailer = self.mailer.clone();
        let sender = self.username.clone();
        let fut = wrap_future::<_, Self>(async move {
            send_verification_email(mailer, sender, username, email, uuid).await
        });
        ctx.spawn(fut);
    }
}

pub async fn send_verification_email(
    mailer: Arc<Option<AsyncSmtpTransport<Tokio1Executor>>>,
    sender: String,
    username: String,
    email: String,
    uuid: Uuid,
) -> () {
    let mailer = &*mailer;
    match mailer {
        None => println!(
            "{}",
            verification_email_stdout(&username, &email, &sender, &uuid)
        ),
        Some(mailer) => {
            let msg = verification_email(&username, &email, &sender, &uuid);
            let res = mailer.send(msg).await;
            if let Err(err) = res {
                warn!("Error sending email: {:?}", err)
            }
        }
    }
}
