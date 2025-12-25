use sqlx::{PgPool, FromRow};
use crate::{User, AppError};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        UserRepository { pool }
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1 AND is_deleted = FALSE")
            .bind(email)
            .fetch_optional(&self.pool)
            .await.map_err(AppError::DatabaseError)?;
        Ok(user)
    }

    pub async fn soft_delete(&self, user_id: i32) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET is_deleted = TRUE, deleted_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await.map_err(AppError::DatabaseError)?;
        Ok(())
    }

    // Additional CRUD methods can be implemented here.
}
