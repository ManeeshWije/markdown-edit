use std::time::Duration;

use crate::models::user::User;
use crate::models::user_session::UserSession;
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

pub async fn fetch_user_by_email(pool: &PgPool, email: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "
        SELECT * FROM users
        WHERE email = $1
        ",
        email
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn fetch_user_by_session_uuid(
    pool: &PgPool,
    session_uuid: Uuid,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "
        SELECT users.* FROM users
        JOIN UserSessions ON users.uuid = UserSessions.user_uuid
        WHERE UserSessions.uuid = $1
        ",
        session_uuid
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

pub async fn create_user_session(
    pool: &PgPool,
    user_uuid: Uuid,
    session_duration: Duration,
) -> Result<UserSession, sqlx::Error> {
    let uuid = Uuid::new_v4();
    let created_at_timestamp = chrono::offset::Utc::now().naive_utc().timestamp();
    let expires_at_timestamp = created_at_timestamp + session_duration.as_secs() as i64;

    sqlx::query!(
        r#"
            INSERT INTO UserSessions (uuid, user_uuid, created_at, expires_at)
            VALUES ($1, $2, $3, $4)
        "#,
        uuid,
        user_uuid,
        created_at_timestamp.to_string(),
        expires_at_timestamp.to_string(),
    )
    .execute(pool)
    .await?;

    let user_session = sqlx::query_as!(
        UserSession,
        r#"
            SELECT 
                uuid as "uuid: uuid::Uuid",
                user_uuid as "user_uuid: uuid::Uuid",
                created_at as "created_at: _",
                expires_at as "expires_at: _" 
            FROM UserSessions
            WHERE uuid = $1
        "#,
        uuid
    )
    .fetch_one(pool)
    .await?;

    Ok(UserSession {
        uuid: user_session.uuid,
        user_uuid: user_session.user_uuid,
        created_at: Some(user_session.created_at.unwrap().to_string()),
        expires_at: Some(user_session.expires_at.unwrap().to_string()),
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

pub async fn delete_user_session(pool: &PgPool, session_uuid: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
        DELETE FROM UserSessions
        WHERE uuid = $1
        ",
        session_uuid
    )
    .execute(pool)
    .await?;

    Ok(())
}
