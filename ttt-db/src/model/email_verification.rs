use sea_orm::{entity::*, query::*, ActiveValue::Set};

use chrono::prelude::*;
use uuid::Uuid;

use crate::entity::{email_verification, users};
use crate::ttt_db::{TttDbConn, TttDbErr};

impl TttDbConn {
    pub async fn verify_email(
        &self,
        uuid: Uuid,
        send_verification_email: fn(&str, &str, Uuid) -> Result<(), String>,
    ) -> Result<(), TttDbErr> {
        let db = &self.db;
        let tx = db.begin().await?;
        let q = email_verification::Entity::find_by_id(uuid)
            .one(&tx)
            .await?;
        match q {
            None => Err(TttDbErr::EmailVerifyNotFound),
            Some(q) => {
                let user = users::Entity::find()
                    .filter(users::Column::Email.eq(q.email.clone()))
                    .one(&tx)
                    .await?
                    .unwrap();
                match Local::now() > q.expires_at {
                    true => {
                        q.delete(&tx).await?;
                        let new_uuid = Uuid::new_v4();
                        email_verification::ActiveModel {
                            email: Set(user.email.clone()),
                            id: Set(new_uuid),
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
                    false => {
                        q.delete(&tx).await?;
                        let mut user: users::ActiveModel = user.into();
                        user.email_verified = Set(true);
                        user.update(&tx).await?;
                        Ok(())
                    }
                }
            }
        }
    }
}
