use crate::ttt_db::{TttDbConn, TttDbErr};
use crate::util::validators::*;

use convert_case::{Case, Casing};
use petname::Petnames;
use sea_orm::ActiveValue::Set;
use sea_orm::{entity::*, QueryFilter, TransactionTrait};
use uuid::Uuid;

use crate::entity::email_verification::ActiveModel as EmailVerificationActiveModel;
use crate::entity::users::Entity as User;
use crate::entity::users::Model as UserModel;
use crate::entity::{user_stats, users};
use crate::serializables::UserMessage;

impl TttDbConn {
    pub async fn find_user_by_id(&self, user_id: i64) -> Result<UserModel, TttDbErr> {
        let db = &self.db;
        let user = User::find_by_id(user_id).one(db).await?;
        match user {
            Some(user) => Ok(user),
            None => Err(TttDbErr::UserNotFound),
        }
    }
    pub async fn sign_up(
        &self,
        mut user: UserMessage,
        send_verification_email: fn(&str, &str, Uuid) -> Result<(), String>,
    ) -> Result<(), TttDbErr> {
        user.email = user.email.trim().into();
        user.username = user.username.trim().into();
        user.password = user.password.trim().into();
        if user.email.len() == 0 || user.username.len() == 0 || user.password.len() == 0 {
            return Err(TttDbErr::Generic(
                "Email, password and username fields cannot be empty.".into(),
            ));
        }
        if !is_valid_email(&user.email) {
            return Err(TttDbErr::Generic("Invalid email format.".into()));
        }
        if !is_valid_username(&user.username) {
            return Err(TttDbErr::Generic("Username must consist of alphanumeric characters and cannot contain special characters.".into()));
        }
        if !is_valid_password(&user.password) {
            return Err(TttDbErr::Generic(
                "Passowrd must be at least 6 characters long.".into(),
            ));
        }
        let db = &self.db;
        let tx = db.begin().await?;
        let res = users::ActiveModel {
            email: Set(user.email.clone()),
            password: Set(hash_password(&user.password)),
            username: Set(user.username.clone()),
            ..Default::default()
        }
        .insert(&tx)
        .await;
        if let Err(err) = res {
            match err {
                sea_orm::DbErr::Query(s) => {
                    if s.contains("users_unique_username") {
                        return Err(TttDbErr::UsernameConfilct);
                    } else if s.contains("users_unique_email") {
                        return Err(TttDbErr::EmailConflict);
                    } else {
                        return Err(TttDbErr::DbErr(sea_orm::DbErr::Query(s)));
                    }
                }
                _ => return Err(TttDbErr::DbErr(err)),
            }
        }
        user_stats::ActiveModel {
            user_id: Set(res.unwrap().user_id),
            ..Default::default()
        }
        .insert(&tx)
        .await?;
        let uuid = Uuid::new_v4();
        EmailVerificationActiveModel {
            email: Set(user.email.clone()),
            id: Set(uuid),
            ..Default::default()
        }
        .insert(&tx)
        .await?;
        match send_verification_email(&user.username, &user.email, uuid) {
            Ok(_) => Ok(tx.commit().await?),
            Err(e) => {
                tx.rollback().await?;
                println!("Could not send verification email: {:?}", e);
                Err(TttDbErr::Generic(
                    "Could not send verification email.".into(),
                ))
            }
        }
    }
    pub async fn create_guest_user(&self) -> Result<UserModel, TttDbErr> {
        let username = Petnames::default().generate_one(3, "_");
        let email = username.clone() + "@askoric.me";
        let username = username.to_case(Case::Pascal);
        let password = "Password".to_string();
        let mut user = UserMessage {
            username,
            email,
            password,
        };
        user.email = user.email.trim().into();
        user.username = user.username.trim().into();
        user.password = user.password.trim().into();
        if !is_valid_email(&user.email) {
            return Err(TttDbErr::Generic("Invalid email format.".into()));
        }
        if !is_valid_username(&user.username) {
            return Err(TttDbErr::Generic("Username must consist of alphanumeric characters and cannot contain special characters.".into()));
        }
        if !is_valid_password(&user.password) {
            return Err(TttDbErr::Generic(
                "Passowrd must be at least 6 characters long.".into(),
            ));
        }
        let db = &self.db;
        let tx = db.begin().await?;
        let res = users::ActiveModel {
            email: Set(user.email.clone()),
            password: Set(hash_password(&user.password)),
            username: Set(user.username.clone()),
            guest: Set(true),
            ..Default::default()
        }
        .insert(&tx)
        .await;
        if let Err(err) = res {
            match err {
                sea_orm::DbErr::Query(s) => {
                    if s.contains("users_unique_username") {
                        return Err(TttDbErr::UsernameConfilct);
                    } else if s.contains("users_unique_email") {
                        return Err(TttDbErr::EmailConflict);
                    } else {
                        return Err(TttDbErr::DbErr(sea_orm::DbErr::Query(s)));
                    }
                }
                _ => return Err(TttDbErr::DbErr(err)),
            }
        }
        let res = res.unwrap();
        user_stats::ActiveModel {
            user_id: Set(res.user_id),
            ..Default::default()
        }
        .insert(&tx)
        .await?;
        tx.commit().await?;
        Ok(res)
    }
    pub async fn sign_in(&self, user: UserMessage) -> Result<UserModel, TttDbErr> {
        let db = &self.db;
        let password = user.password;
        let db_user = User::find()
            .filter(users::Column::Email.eq(user.email))
            .one(db)
            .await?;
        if let Some(db_user) = db_user {
            match verify_password(&password, &db_user.password) {
                true => Ok(db_user),
                false => Err(TttDbErr::InvalidPassword),
            }
        } else {
            Err(TttDbErr::UserNotFound)
        }
    }
}
