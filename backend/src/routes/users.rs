use axum::body::Body;
use axum::extract::State;
use axum::http::{Response, StatusCode};
use axum::response::ErrorResponse;
use axum::Json;
use axum::{
    routing::{delete, get},
    Router,
};
use axum_extra::extract::cookie::CookieJar;

use crate::db::user_queries;
use crate::models::user::User;
use crate::utils::helpers::check_user_session;

pub fn users_routes(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/me", get(get_user_by_uuid))
        .route("/delete", delete(delete_user))
        .with_state(pool)
}

async fn get_user_by_uuid(
    cookies: CookieJar,
    State(pool): State<sqlx::PgPool>,
) -> Result<Json<User>, ErrorResponse> {
    // Check if the user is logged in and fetch the user from the database
    let user = match check_user_session(cookies, pool.clone()).await {
        Ok(user) => user,
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            return Err(ErrorResponse::from(StatusCode::UNAUTHORIZED));
        }
    };

    Ok(Json(user))
}

async fn delete_user(
    cookies: CookieJar,
    State(pool): State<sqlx::PgPool>,
) -> Result<Response<Body>, ErrorResponse> {
    // Check if the user is logged in
    let user = match check_user_session(cookies, pool.clone()).await {
        Ok(user) => user,
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            return Err(ErrorResponse::from(StatusCode::UNAUTHORIZED));
        }
    };

    // Delete the user from the database and return a success response
    user_queries::delete_user(&pool, user.uuid)
        .await
        .map_err(|err| {
            eprintln!("Database error: {}", err);
            ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR)
        })?;

    Ok(Response::new(Body::empty()))
}
