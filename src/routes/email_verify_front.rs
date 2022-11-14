use std::process::exit;

use actix_web::{get, web, HttpResponse};
use log::error;
use serde_json::json;
use tinytemplate::TinyTemplate;
use ttt_db::TttDbErr;
use ttt_mailer::SendVerificationEmail;
use uuid::Uuid;

use crate::util::TttApiErr;
use crate::AppState;

static TEMPLATE: &'static str = r#"
<!DOCTYPE html><html><head><meta charset="UTF-8"><link rel="preconnect" href="https://fonts.googleapis.com"><link rel="preconnect" href="https://fonts.gstatic.com" crossorigin><link href="https://fonts.googleapis.com/css2?family=Roboto:wght@400&display=swap" rel="stylesheet"><title>Email verification</title></head><body style="background-color:#1e1e2e;margin:0;height:100vh;display:flex;justify-content:center;align-items:center"><p style="font-family:Roboto,sans-serif;color:#cdd6f4">{msg}</p></body></html>
"#;

#[get("/email_verification/{uuid}")]
async fn verify(
    data: web::Data<AppState>,
    uuid: web::Path<String>,
) -> Result<HttpResponse, TttApiErr> {
    let tt = {
        let mut tt = TinyTemplate::new();
        tt.add_template("email_verify", TEMPLATE)
            .unwrap_or_else(|_| {
                error!("Error parsing email verification template!");
                exit(1);
            });
        tt
    };
    let uuid = match Uuid::parse_str(&uuid) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Ok(HttpResponse::Ok().content_type("text/html").body(
                tt.render(
                    "email_verify",
                    &json!({"msg": "Email verification not found"}),
                )
                .unwrap(),
            ))
        }
    };
    let db = &data.ttt_db;
    let mailer = data.mail_worker.clone();
    let f = move |username: String, email: String, uuid: Uuid| {
        mailer.do_send(SendVerificationEmail::new(username, email, uuid))
    };
    let res = db.verify_email(uuid, f).await;
    match res {
        Ok(_) => Ok(HttpResponse::Ok().content_type("text/html").body(
            tt.render(
                "email_verify",
                &json!({"msg": "Email verified. You can now close this page."}),
            )
            .unwrap(),
        )),
        Err(err) => match err {
            TttDbErr::EmailVerifyNotFound => Ok(HttpResponse::Ok().content_type("text/html").body(
                tt.render(
                    "email_verify",
                    &json!({"msg": "Email verification not found."}),
                )
                .unwrap(),
            )),
            TttDbErr::EmailVerifyExpired => Ok(HttpResponse::Ok().content_type("text/html").body(
                tt.render(
                    "email_verify",
                    &json!({"msg": "Link expired. We have sent you a new link. Check your email."}),
                )
                .unwrap(),
            )),
            _ => Ok(HttpResponse::Ok().content_type("text/html").body(
                tt.render("email_verify", &json!({"msg": "Unhandled error occured."}))
                    .unwrap(),
            )),
        },
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(verify);
}
