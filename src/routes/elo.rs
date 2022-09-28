use actix_session::Session;
use actix_web::{get, web, HttpResponse};

use crate::util::{SessionData, TttApiErr};
use crate::AppState;

#[get("/elo")]
async fn get_elo(data: web::Data<AppState>, session: Session) -> Result<HttpResponse, TttApiErr> {
    let user = session.get_data()?;
    let db = &data.ttt_db;
    let elo = db.get_elo(user.id).await?;
    Ok(HttpResponse::Ok().json(elo))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_elo);
}
