use sea_orm::{entity::*, query::*, ActiveValue::Set};

use chrono::prelude::*;
use uuid::Uuid;

use crate::entity::{email_verification, users};
use crate::ttt_db::{TttDbConn, TttDbErr};

impl TttDbConn {
    pub async fn verify_email<F>(
        &self,
        uuid: Uuid,
        send_verification_email: F,
    ) -> Result<(), TttDbErr>
    where
        F: FnOnce(String, String, Uuid) -> (),
    {
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
                        tx.commit().await?;
                        send_verification_email(user.username, user.email, uuid);
                        Ok(())
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
