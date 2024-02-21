use axum::extract::State;
use axum::http::{Response, StatusCode};
use axum::Json;
use axum::{
    body::Body,
    routing::{delete, get, post, put},
    Router,
};
use uuid::Uuid;

use crate::db::user_queries;
use crate::models::user::User;

pub fn users_routes(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/:uuid", get(get_user_by_uuid))
        .route("/create", post(create_user))
        .route("/update", put(update_user))
        .route("/delete/:uuid", delete(delete_user))
        .with_state(pool)
}

async fn get_user_by_uuid(
    State(pool): State<sqlx::PgPool>,
    params: axum::extract::Path<String>,
) -> Result<Json<User>, String> {
    // parse the uuid from the path and if its invalid return a 400
    let uuid = match Uuid::parse_str(&params) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err("Invalid UUID".to_string());
        }
    };

    let user = match user_queries::fetch_user_by_uuid(&pool, uuid).await {
        Ok(user) => user,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err("Failed to fetch user".to_string());
        }
    };

    Ok(Json(user))
}

async fn create_user(
    State(pool): State<sqlx::PgPool>,
    request: Json<User>,
) -> Result<Json<User>, String> {
    let request_body = request.0;
    let uuid = request_body.uuid;
    let username = request_body.username;
    let email = request_body.email;

    let user = match user_queries::create_user(
        &pool,
        uuid,
        username.clone().as_str(),
        email.clone().as_str(),
    )
    .await
    {
        Ok(user) => user,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err("Failed to create user".to_string());
        }
    };

    Ok(Json(user))
}

async fn update_user(
    State(pool): State<sqlx::PgPool>,
    request: Json<User>,
) -> Result<Json<User>, String> {
    let request_body = request.0;
    let uuid = request_body.uuid;
    let username = request_body.username;
    let email = request_body.email;

    let user = match user_queries::update_user(&pool, uuid, username.as_str(), email.as_str()).await {
        Ok(user) => user,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err("Failed to update user".to_string());
        }
    };

    Ok(Json(user))
}

async fn delete_user(
    State(pool): State<sqlx::PgPool>,
    params: axum::extract::Path<String>,
) -> Result<Response<Body>, String> {
    let uuid = match Uuid::parse_str(&params) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err("Invalid UUID".to_string());
        }
    };

    match user_queries::delete_user(&pool, uuid).await {
        Ok(_) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("User deleted"))
            .unwrap()),
        Err(err) => {
            eprintln!("Database error: {}", err);
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to delete user"))
                .unwrap())
        }
    }
}
