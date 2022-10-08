use log::warn;
use std::fmt::Display;

use actix_session::SessionInsertError;
use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use ttt_db::TttDbErr::{self, *};

#[derive(Debug, Clone)]
pub struct TttApiErr {
    status_code: StatusCode,
    body: String,
}

impl Display for TttApiErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "error {}:\t{}", self.status_code, self.body)
    }
}

impl TttApiErr {
    pub fn forbidden() -> Self {
        Self {
            status_code: StatusCode::FORBIDDEN,
            body: "Forbidden".into(),
        }
    }
    pub fn unhandled() -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            body: "Unhandled error occured.".into(),
        }
    }
}

impl ResponseError for TttApiErr {
    fn status_code(&self) -> StatusCode {
        self.status_code.clone()
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self.body.clone())
    }
}

impl From<TttDbErr> for TttApiErr {
    fn from(err: TttDbErr) -> Self {
        let body = err.to_string();
        let status_code = match err {
            UsernameConfilct => StatusCode::CONFLICT,
            EmailConflict => StatusCode::CONFLICT,
            UserNotFound => StatusCode::NOT_FOUND,
            InvalidPassword => StatusCode::BAD_REQUEST,
            EmailVerifyNotFound => StatusCode::NOT_FOUND,
            EmailVerifyExpired => StatusCode::GONE,
            UserAlreadyQueued => StatusCode::CONFLICT,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        Self { status_code, body }
    }
}

impl From<serde_json::Error> for TttApiErr {
    fn from(err: serde_json::Error) -> Self {
        warn!("{:?}", err);
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            body: "Internal server error".into(),
        }
    }
}

impl From<SessionInsertError> for TttApiErr {
    fn from(err: SessionInsertError) -> Self {
        warn!("{:?}", err);
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            body: "Internal server error".into(),
        }
    }
}
