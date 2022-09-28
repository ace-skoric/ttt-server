use crate::util::TttApiErr;
use crate::{util::SessionData, AppState};
use actix_session::Session;
use actix_web::{get, web, web::Payload, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use ttt_db::TttDbErr::UserAlreadyQueued;
use ttt_matchmaking::ws::MatchmakingWebsocket as MmWs;

#[get("/matchmaking")]
async fn enter_queue(
    data: web::Data<AppState>,
    req: HttpRequest,
    stream: Payload,
    session: Session,
) -> Result<HttpResponse, TttApiErr> {
    let user = session.get_data()?;
    let db = &data.ttt_db;
    if db.check_if_queued(user.id).await? {
        return Err(UserAlreadyQueued.into());
    }
    let mm_worker = data.mm_worker.clone();
    let ws = MmWs::new(user.id, mm_worker);
    let res = ws::start(ws, &req, stream);
    match res {
        Ok(res) => Ok(res),
        Err(_) => Err(TttApiErr::unhandled()),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(enter_queue);
}
