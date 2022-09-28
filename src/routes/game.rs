use crate::util::TttApiErr;
use crate::{util::SessionData, AppState};
use actix_session::Session;
use actix_web::{get, web, web::Payload, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use ttt_game_server::server::messages::GetGameAddress;
use ttt_game_server::ws::GameWebsocket as GameWs;
use uuid::Uuid;

#[get("/game/{game_id}")]
async fn enter_queue(
    data: web::Data<AppState>,
    req: HttpRequest,
    stream: Payload,
    session: Session,
    game_id: web::Path<Uuid>,
) -> Result<HttpResponse, TttApiErr> {
    let user = session.get_data()?;
    let db = &data.ttt_db;
    let res = db.check_user_in_active_game(*game_id, user.id).await?;
    if !res {
        return Err(TttApiErr::forbidden());
    }
    let game_srv = &data.game_server;
    let game_addr = game_srv.send(GetGameAddress(*game_id)).await;
    match game_addr {
        Ok(addr) => {
            if addr.is_none() {
                return Err(TttApiErr::forbidden());
            }
            let game_addr = addr.unwrap();
            let ws = GameWs::new(user.id, game_addr);
            let res = ws::start(ws, &req, stream);
            match res {
                Ok(res) => Ok(res),
                Err(_) => Err(TttApiErr::unhandled()),
            }
        }
        Err(_) => return Err(TttApiErr::forbidden()),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(enter_queue);
}
