use crate::models::User;
use async_trait::async_trait;
use sqlx::PgPool;

#[async_trait]
pub trait UserDb {
    async fn create_user(&self, email: &str, password_hash: &str) -> Result<User, sqlx::Error>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error>;
}

#[async_trait]
impl UserDb for PgPool {
    async fn create_user(&self, email: &str, password_hash: &str) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password_hash)
            VALUES ($1, $2)
            RETURNING id, email, password_hash, created_at
            "#,
            email,
            password_hash
        )
        .fetch_one(self)
        .await?;

        Ok(user)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, email, password_hash, created_at
            FROM users
            WHERE email = $1
            "#,
            email
        )
        .fetch_optional(self)
        .await?;

        Ok(user)
    }
}