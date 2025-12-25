use sqlx::query;

async fn get_posts_for_users(pool: &sqlx::PgPool, user_ids: &[i32]) -> Result<Vec<Post>, sqlx::Error> {
    let posts = query!("SELECT * FROM posts WHERE user_id = ANY($1)", user_ids)
        .fetch_all(pool)
        .await?;
    Ok(posts)
}