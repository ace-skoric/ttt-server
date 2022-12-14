use actix_session::Session;
use actix_web::{get, post, web, HttpResponse};
use ttt_db::serializables::UserMessage;
use ttt_mailer::SendVerificationEmail;
use uuid::Uuid;

use crate::util::{SessionData, TttApiErr};
use crate::AppState;

#[get("/user/{user_id}")]
async fn find_by_id(
    data: web::Data<AppState>,
    user_id: web::Path<i64>,
    session: Session,
) -> Result<HttpResponse, TttApiErr> {
    let user = session.get_data()?;
    if !user.admin {
        return Err(TttApiErr::forbidden());
    }
    let db = &data.ttt_db;
    let user = db.find_user_by_id(*user_id).await?;
    Ok(HttpResponse::Ok().json(user))
}

#[get("/user")]
async fn get_session_data(session: Session) -> Result<HttpResponse, TttApiErr> {
    let user = session.get_data()?;
    Ok(HttpResponse::Ok().json(user))
}

#[post("/user/claim")]
async fn claim_guest_account(
    session: Session,
    data: web::Data<AppState>,
    req: web::Json<UserMessage>,
) -> Result<HttpResponse, TttApiErr> {
    let user = session.get_data()?;
    if !user.guest {
        return Ok(HttpResponse::BadRequest().json("This is not guest account."));
    }
    let db = &data.ttt_db;
    let mailer = data.mail_worker.clone();
    let f = move |username: String, email: String, uuid: Uuid| {
        mailer.do_send(SendVerificationEmail::new(username, email, uuid))
    };
    let user = db.claim_guest_account(user.id, req.into_inner(), f).await?;
    session.insert("id", &user.user_id)?;
    session.insert("username", &user.username)?;
    session.insert("admin", &user.is_admin)?;
    session.insert("guest", &user.guest)?;
    Ok(HttpResponse::Created().json("Account successfuly claimed."))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_by_id);
    cfg.service(get_session_data);
    cfg.service(claim_guest_account);
}
