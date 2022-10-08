use actix_session::Session;
use actix_web::{get, post, web, HttpResponse};
use ttt_db::TttDbErr;
use ttt_mailer::SendVerificationEmail;
use uuid::Uuid;

use crate::util::SessionData;
use crate::util::TttApiErr;
use crate::AppState;

#[post("/verify/uuid/{uuid}")]
async fn verify(
    data: web::Data<AppState>,
    uuid: web::Path<String>,
) -> Result<HttpResponse, TttApiErr> {
    let uuid = match Uuid::parse_str(&uuid) {
        Ok(uuid) => uuid,
        Err(_) => return Err(TttApiErr::from(TttDbErr::EmailVerifyNotFound)),
    };
    let db = &data.ttt_db;
    let mailer = data.mail_worker.clone();
    let f = move |username: String, email: String, uuid: Uuid| {
        mailer.do_send(SendVerificationEmail::new(username, email, uuid))
    };
    db.verify_email(uuid, f).await?;
    Ok(HttpResponse::Created().json("Welcome to Crystalium"))
}

#[post("/verify/resend")]
async fn resend_verification_email(
    data: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, TttApiErr> {
    let user = session.get_data()?;
    let db = &data.ttt_db;
    let mailer = data.mail_worker.clone();
    let f = move |username: String, email: String, uuid: Uuid| {
        mailer.do_send(SendVerificationEmail::new(username, email, uuid))
    };
    db.resend_verification_email(user.id, f).await?;
    Ok(HttpResponse::Created().json("Email sent"))
}

#[get("/verify/status")]
async fn get_verification_status(
    data: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, TttApiErr> {
    let user = session.get_data()?;
    let db = &data.ttt_db;
    let u = db.find_user_by_id(user.id).await?;
    Ok(HttpResponse::Ok().json(u.email_verified))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(verify);
    cfg.service(resend_verification_email);
    cfg.service(get_verification_status);
}
