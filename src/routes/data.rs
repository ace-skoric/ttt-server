use actix_session::Session;
use actix_web::{get, web, HttpResponse};

use crate::AppState;

use crate::util::{SessionData, TttApiErr};

#[get("/data")]
async fn user_data(data: web::Data<AppState>, session: Session) -> Result<HttpResponse, TttApiErr> {
    let user = session.get_data()?;
    let db = &data.ttt_db;
    let data = db.get_user_data(user.id).await?;
    Ok(HttpResponse::Ok().json(data))
}

#[get("/elo")]
async fn get_elo(data: web::Data<AppState>, session: Session) -> Result<HttpResponse, TttApiErr> {
    let user = session.get_data()?;
    let db = &data.ttt_db;
    let elo = db.get_elo(user.id).await?;
    Ok(HttpResponse::Ok().json(elo))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(user_data);
    cfg.service(get_elo);
}
