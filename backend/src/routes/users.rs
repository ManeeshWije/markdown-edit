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
use uuid::Uuid;

use crate::db::user_queries;
use crate::models::user::User;
use crate::utils::helpers::check_user_session;

pub fn users_routes(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/:uuid", get(get_user_by_uuid))
        .route("/delete/:uuid", delete(delete_user))
        .with_state(pool)
}

async fn get_user_by_uuid(
    cookies: CookieJar,
    State(pool): State<sqlx::PgPool>,
    params: axum::extract::Path<String>,
) -> Result<Json<User>, ErrorResponse> {
    // Parse the UUID from the request parameters
    let _uuid = match Uuid::parse_str(&params) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err(ErrorResponse::from(StatusCode::BAD_REQUEST));
        }
    };

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
    params: axum::extract::Path<String>,
) -> Result<Response<Body>, ErrorResponse> {
    // Parse the UUID from the request parameters
    let uuid = match Uuid::parse_str(&params) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err(ErrorResponse::from(StatusCode::BAD_REQUEST));
        }
    };

    // Check if the user is logged in
    let _user = match check_user_session(cookies, pool.clone()).await {
        Ok(user) => user,
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            return Err(ErrorResponse::from(StatusCode::UNAUTHORIZED));
        }
    };

    // Delete the user from the database and return a success response
    user_queries::delete_user(&pool, uuid)
        .await
        .map_err(|err| {
            eprintln!("Database error: {}", err);
            ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR)
        })?;

    Ok(Response::new(Body::empty()))
}
