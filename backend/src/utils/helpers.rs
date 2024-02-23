use axum::http::StatusCode;
use axum::response::ErrorResponse;
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::db::user_queries;
use crate::models::user::User;
use crate::utils::constants::COOKIE_AUTH_SESSION;

// Helper function to check if the user is logged in and fetch the user from the database if they are
pub async fn check_user_session(
    cookies: CookieJar,
    pool: sqlx::PgPool,
) -> Result<User, ErrorResponse> {
    let session_cookie = cookies.get(COOKIE_AUTH_SESSION);

    // If the session cookie is not present, return an error
    let Some(session_cookie) = session_cookie else {
        return Err(ErrorResponse::from(StatusCode::UNAUTHORIZED));
    };

    // Parse the UUID from the session cookie
    let session_uuid = match Uuid::parse_str(session_cookie.value()) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err(ErrorResponse::from(StatusCode::BAD_REQUEST));
        }
    };

    // Fetch the user from the database
    let user = match user_queries::fetch_user_by_session_uuid(&pool, session_uuid).await {
        Ok(user) => user,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err(ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR));
        }
    };

    Ok(user)
}
