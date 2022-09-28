use actix_session::Session;
use actix_web::{post, web, HttpResponse};

use crate::AppState;
use ttt_db::serializables::UserMessage;

use crate::util::send_verification_email;
use crate::util::SessionData;
use crate::util::TttApiErr;

#[post("/auth/signup")]
async fn sign_up(
    data: web::Data<AppState>,
    req: web::Json<UserMessage>,
) -> Result<HttpResponse, TttApiErr> {
    let db = &data.ttt_db;
    let user = req.into_inner();
    db.sign_up(user, send_verification_email).await?;
    Ok(HttpResponse::Created().json("User Created"))
}

#[post("/auth/signin")]
async fn sign_in(
    data: web::Data<AppState>,
    req: web::Json<UserMessage>,
    session: Session,
) -> Result<HttpResponse, TttApiErr> {
    let db = &data.ttt_db;
    let user = db.sign_in(req.into_inner()).await?;
    if user.is_banned {
        let ban_ends = match user.ban_ends {
            Some(datetime) => datetime.format("%d/%m/%Y %R %Z").to_string(),
            None => "the end of time".to_string(),
        };
        return Ok(HttpResponse::Forbidden().json(format!(
            "Your account has been suspended until {}",
            ban_ends
        )));
    }
    session.insert("id", &user.user_id)?;
    session.insert("username", &user.username)?;
    session.insert("admin", &user.is_admin)?;
    session.insert("guest", &user.guest)?;
    session.renew();
    Ok(HttpResponse::Created().json("Signed in successfuly"))
}

#[post("/auth/guest")]
async fn sign_in_as_guest(
    data: web::Data<AppState>,
    session: Session,
) -> Result<HttpResponse, TttApiErr> {
    let db = &data.ttt_db;
    let user = db.create_guest_user().await?;
    session.insert("id", &user.user_id)?;
    session.insert("username", &user.username)?;
    session.insert("admin", &user.is_admin)?;
    session.insert("guest", &user.guest)?;
    session.renew();
    Ok(HttpResponse::Created().json("Signed in successfuly"))
}

#[post("/auth")]
async fn authorize_session(session: Session) -> Result<HttpResponse, TttApiErr> {
    let user = &session.get_data()?;
    Ok(HttpResponse::Ok().json(format!("You are signed in as {}", user.username)))
}

#[post("/auth/signout")]
async fn signout(session: Session) -> Result<HttpResponse, TttApiErr> {
    let _ = &session.get_data()?;
    session.purge();
    Ok(HttpResponse::Ok().json("Logged out"))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(sign_up);
    cfg.service(sign_in);
    cfg.service(sign_in_as_guest);
    cfg.service(signout);
    cfg.service(authorize_session);
}
