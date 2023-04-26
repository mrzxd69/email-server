use sea_orm::{Database,DatabaseConnection, EntityTrait, ColumnTrait, QueryFilter, Set};
use crate::models::{user, user::Entity as User};
use crate::cfg::config::DATABASE_CONNECTION;
use anyhow::{anyhow, Result};

pub async fn register_user(email: String) -> Result<()> {
    let db: DatabaseConnection = Database::connect(DATABASE_CONNECTION).await?;

    let user = User::find()
        .filter(user::Column::Email.contains(&email))
        .all(&db)
        .await?;


    if user.is_empty() {
        let user = user::ActiveModel {
            email: Set(email)
        };

        User::insert(user).exec(&db).await?;
    } else {
        return Err(anyhow!("Данный email уже зарегистрирован"));
    }
    Ok(())
}