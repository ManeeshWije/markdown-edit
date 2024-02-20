use crate::models::user::User;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn fetch_user_by_uuid(pool: &PgPool, uuid: Uuid) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "
        SELECT * FROM users
        WHERE uuid = $1
        ",
        uuid
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn create_user(
    pool: &PgPool,
    uuid: Uuid,
    username: &str,
    email: &str,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query!(
        "
        INSERT INTO users (uuid, username, email, created_at, updated_at)
        VALUES ($1, $2, $3, DEFAULT, DEFAULT)
        RETURNING *
        ",
        uuid,
        username,
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(User {
        uuid: user.uuid,
        username: user.username,
        email: user.email,
        created_at: Some(user.created_at.unwrap().to_string()),
        updated_at: Some(user.updated_at.unwrap().to_string()),
    })
}

pub async fn update_user(
    pool: &PgPool,
    uuid: Uuid,
    username: &str,
    email: &str,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query!(
        "
        UPDATE users
        SET username = $2, email = $3, updated_at = DEFAULT
        WHERE uuid = $1
        RETURNING *
        ",
        uuid,
        username,
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(User {
        uuid: user.uuid,
        username: user.username,
        email: user.email,
        created_at: Some(user.created_at.unwrap().to_string()),
        updated_at: Some(user.updated_at.unwrap().to_string()),
    })
}

pub async fn delete_user(pool: &PgPool, uuid: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
        DELETE FROM users
        WHERE uuid = $1
        ",
        uuid
    )
    .execute(pool)
    .await?;

    sqlx::query!(
        "
        DELETE FROM documents
        WHERE user_uuid = $1
        ",
        uuid
    )
    .execute(pool)
    .await?;

    Ok(())
}
