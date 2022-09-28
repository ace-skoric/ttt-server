use actix_session::Session;
use actix_web::{get, web, HttpResponse};

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
    Ok(HttpResponse::NotImplemented().json(user))
}

#[get("/user")]
async fn get_session_data(session: Session) -> Result<HttpResponse, TttApiErr> {
    let user = session.get_data()?;
    Ok(HttpResponse::Ok().json(user))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_by_id);
    cfg.service(get_session_data);
}
