use actix_session::Session;
use actix_web::{get, post, web, HttpResponse};
use uuid::Uuid;

use crate::util::send_verification_email;
use crate::util::SessionData;
use crate::util::TttApiErr;
use crate::AppState;

#[post("/verify/{uuid}")]
async fn verify(
    data: web::Data<AppState>,
    uuid: web::Path<Uuid>,
) -> Result<HttpResponse, TttApiErr> {
    let db = &data.ttt_db;
    db.verify_email(*uuid, send_verification_email).await?;
    Ok(HttpResponse::Created().json("Welcome to Crystalium"))
    //     Err(e) => {
    //         match e {
    //             DbErr::Custom(e) => {
    //                 match e.as_str() {
    //                     "None found" => Ok(HttpResponse::NotFound().json("Resource not found")),
    //                     "Validation link expired" => Ok(HttpResponse::Gone().json("Validation link expired")),
    //                     _ => Ok(HttpResponse::InternalServerError().body("Unhandled error occured"))
    //                 }
    //             },
    //             _ => Ok(HttpResponse::InternalServerError().body("Unhandled error occured"))
    //         }
    //     }
    // }
}

#[post("/verify/resend")]
async fn resend_verification_email(_data: web::Data<AppState>) -> Result<HttpResponse, TttApiErr> {
    Ok(HttpResponse::NotImplemented().json("Function not implemented"))
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
