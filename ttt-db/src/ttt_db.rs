use redis::RedisError;
use sea_orm::{DatabaseConnection, DbErr};

#[derive(Debug, Clone)]
pub struct TttDbConn {
    pub(crate) db: DatabaseConnection,
    pub(crate) rdb: redis::Client,
}

impl TttDbConn {
    pub async fn new(db_conn_str: &str, redis_conn_str: &str) -> Self {
        Self {
            db: sea_orm::Database::connect(db_conn_str)
                .await
                .expect("Cannot connect to database"),
            rdb: redis::Client::open(redis_conn_str).expect("Error connecting to redis backend"),
        }
    }
    pub fn get_rdb(&self) -> redis::Client {
        self.rdb.clone()
    }
}

#[derive(Debug)]
pub enum TttDbErr {
    UsernameConfilct,
    EmailConflict,
    EmailVerifyNotFound,
    UserNotFound,
    InvalidPassword,
    Unhandled,
    UserAlreadyQueued,
    Generic(String),
    DbErr(sea_orm::DbErr),
}

impl ToString for TttDbErr {
    fn to_string(&self) -> String {
        match self {
            Self::UsernameConfilct => "Username already taken.".into(),
            Self::EmailConflict => "User already registered with this email.".into(),
            Self::UserNotFound => "User not found.".into(),
            Self::InvalidPassword => "Invalid password.".into(),
            Self::EmailVerifyNotFound => "Email verification not found.".into(),
            Self::Unhandled => "Unhandled error occured.".into(),
            Self::UserAlreadyQueued => "User is already in the matchmaking queue.".into(),
            Self::Generic(s) => s.to_string(),
            Self::DbErr(err) => err.to_string(),
        }
    }
}

impl Into<String> for TttDbErr {
    fn into(self) -> String {
        self.to_string()
    }
}

impl From<DbErr> for TttDbErr {
    fn from(err: DbErr) -> Self {
        TttDbErr::DbErr(err)
    }
}

impl From<RedisError> for TttDbErr {
    fn from(err: RedisError) -> Self {
        TttDbErr::Generic(err.category().to_string())
    }
}
