use axum::http::{Response, StatusCode};
use axum::Json;
use axum::{
    body::Body,
    routing::{get, post},
    Router,
};
use uuid::Uuid;

use crate::db::queries;
use crate::models::user::User;

pub fn users_routes(pool: sqlx::PgPool) -> Router {
    let pool_clone = pool.clone();
    Router::new()
        .route(
            "/create",
            post(move |user: Json<User>| create_user(pool.clone(), user)),
        )
        .route(
            "/:uuid",
            get(|params: axum::extract::Path<String>| get_user_by_uuid(pool_clone, params)),
        )
}

async fn get_user_by_uuid(
    pool: sqlx::PgPool,
    params: axum::extract::Path<String>,
) -> impl axum::response::IntoResponse {
    // parse the uuid from the path and if its invalid return a 400
    let uuid = match Uuid::parse_str(&params) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Invalid UUID"))
                .unwrap()
        }
    };

    let user = match queries::fetch_user_by_uuid(&pool, uuid).await {
        Ok(user) => user,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to fetch user"))
                .unwrap();
        }
    };

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&user).unwrap()))
        .unwrap()
}

async fn create_user(pool: sqlx::PgPool, request: Json<User>) -> impl axum::response::IntoResponse {
    let request_body = request.0;
    let uuid = request_body.uuid;
    let username = request_body.username;
    let email = request_body.email;

    let user = match queries::create_user(
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
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to create user"))
                .unwrap();
        }
    };

    Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&user).unwrap()))
        .unwrap()
}
