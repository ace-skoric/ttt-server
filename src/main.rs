extern crate log;

use std::sync::Arc;

use actix::{Actor, Addr};
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{web, App, HttpServer};
use ttt_db::TttDbConn;

mod routes;
mod util;
use ttt_game_server::server::GameServer;
use ttt_mailer::MailWorker;
use ttt_matchmaking::MatchmakingWorker;
use util::env;

use routes::*;

#[derive(Debug, Clone)]
struct AppState {
    ttt_db: TttDbConn,
    mm_worker: Addr<MatchmakingWorker>,
    game_server: Addr<GameServer>,
    mail_worker: Addr<MailWorker>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::init_env();

    // Connect to Postgres backend
    let db_url = &*env::DATABASE_URL;
    let redis_url = &*env::REDIS_URL;
    let ttt_db = TttDbConn::new(&db_url, &redis_url).await;
    let ttt_db_arc = Arc::new(ttt_db.clone());

    let mail_worker = MailWorker::new().await;
    let mail_worker = mail_worker.start();

    let game_server = GameServer::new(ttt_db_arc.clone());
    let game_server = game_server.start();

    let mm_worker = MatchmakingWorker::new(ttt_db_arc, game_server.clone());
    let mm_worker = mm_worker.start();

    let state = AppState {
        ttt_db,
        mm_worker,
        game_server,
        mail_worker,
    };

    // Add redis session store
    let store = RedisSessionStore::new(redis_url)
        .await
        .expect("Error connecting to redis backend");

    // Build server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(store.clone(), env::SESSION_SECRET.clone())
                    .cookie_name("ttt-session".into())
                    .cookie_secure(false)
                    .build(),
            )
            .app_data(web::Data::new(state.clone()))
            .service(
                web::scope("/api/v1")
                    .configure(auth::init_routes)
                    .configure(user::init_routes)
                    .configure(email_verify::init_routes)
                    .configure(matchmaking::init_routes)
                    .configure(data::init_routes)
                    .configure(elo::init_routes)
                    .configure(game::init_routes),
            )
            .configure(email_verify_front::init_routes)
    });
    let host = &*env::HOST.as_str();
    let port = *env::PORT;
    let server = server.bind((host, port))?;

    // Run mm_worker
    // Run server
    server.run().await
}
