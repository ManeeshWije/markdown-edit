use sqlx::PgPool;
use std::time::Duration;
use uuid::Uuid;

use crate::models::user::User;
use crate::models::user_session::UserSession;

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
        SELECT u.uuid, u.username, u.email, u.created_at, u.updated_at
        FROM users AS u
        LEFT JOIN UserSessions AS s ON u.uuid = s.user_uuid
        WHERE s.uuid = $1 AND s.expires_at > $2
        ",
        session_uuid,
        chrono::offset::Utc::now()
            .naive_utc()
            .timestamp()
            .to_string()
    )
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn fetch_user_session_by_user_uuid(
    pool: &PgPool,
    user_uuid: Uuid,
) -> Result<UserSession, sqlx::Error> {
    let user_session = sqlx::query_as!(
        UserSession,
        "
        SELECT * FROM UserSessions
        WHERE user_uuid = $1
        ",
        user_uuid
    )
    .fetch_one(pool)
    .await?;

    Ok(user_session)
}

pub async fn create_user(
    pool: &PgPool,
    uuid: Uuid,
    username: &str,
    email: &str,
) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
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

    Ok(user)
}

pub async fn create_user_session(
    pool: &PgPool,
    user_uuid: Uuid,
    session_duration: Duration,
) -> Result<UserSession, sqlx::Error> {
    let uuid = Uuid::new_v4();
    let created_at_timestamp = chrono::offset::Utc::now().naive_utc().timestamp();
    let expires_at_timestamp = created_at_timestamp + session_duration.as_secs() as i64;

    sqlx::query_as!(
        UserSession,
        "
        INSERT INTO UserSessions (uuid, user_uuid, created_at, expires_at)
        VALUES ($1, $2, $3, $4)
        ",
        uuid,
        user_uuid,
        created_at_timestamp.to_string(),
        expires_at_timestamp.to_string(),
    )
    .execute(pool)
    .await?;

    let user_session = sqlx::query_as!(
        UserSession,
        "
        SELECT * FROM UserSessions
        WHERE uuid = $1
        ",
        uuid
    )
    .fetch_one(pool)
    .await?;

    Ok(user_session)
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

pub async fn delete_expired_sessions(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
        DELETE FROM UserSessions
        WHERE expires_at < $1
        ",
        chrono::offset::Utc::now()
            .naive_utc()
            .timestamp()
            .to_string()
    )
    .execute(pool)
    .await?;

    Ok(())
}
