use sqlx::query;

async fn get_user_by_id(pool: &sqlx::PgPool, user_id: i32) -> Result<User, sqlx::Error> {
    let user = query!("SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(pool)
        .await?;
    Ok(user)
}