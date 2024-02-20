use std::sync::Arc;

use axum::http::{Response, StatusCode};
use axum::Json;
use axum::{
    body::Body,
    routing::{delete, get, post, put},
    Router,
};
use uuid::Uuid;

use crate::db::queries;
use crate::models::user::User;

pub fn users_routes(pool: sqlx::PgPool) -> Router {
    let pool_arc = Arc::new(pool);

    let post_pool_arc = pool_arc.clone();
    let get_pool_arc = pool_arc.clone();
    let put_pool_arc = pool_arc.clone();
    let delete_pool_arc = pool_arc.clone();

    Router::new()
        .route(
            "/create",
            post(move |user: Json<User>| create_user(post_pool_arc, user)),
        )
        .route(
            "/:uuid",
            get(|params: axum::extract::Path<String>| get_user_by_uuid(get_pool_arc, params)),
        )
        .route(
            "/update",
            put(move |user: Json<User>| update_user(put_pool_arc, user)),
        )
        .route(
            "/delete/:uuid",
            delete(|params: axum::extract::Path<String>| delete_user(delete_pool_arc, params)),
        )
}

async fn get_user_by_uuid(
    pool: Arc<sqlx::PgPool>,
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

async fn create_user(
    pool: Arc<sqlx::PgPool>,
    request: Json<User>,
) -> impl axum::response::IntoResponse {
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

async fn update_user(
    pool: Arc<sqlx::PgPool>,
    request: Json<User>,
) -> impl axum::response::IntoResponse {
    let request_body = request.0;
    let uuid = request_body.uuid;
    let username = request_body.username;
    let email = request_body.email;

    let user = match queries::update_user(&pool, uuid, username.as_str(), email.as_str()).await {
        Ok(user) => user,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to update user"))
                .unwrap();
        }
    };

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&user).unwrap()))
        .unwrap()
}

async fn delete_user(
    pool: Arc<sqlx::PgPool>,
    params: axum::extract::Path<String>,
) -> impl axum::response::IntoResponse {
    let uuid = match Uuid::parse_str(&params) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Invalid UUID"))
                .unwrap()
        }
    };

    match queries::delete_user(&pool, uuid).await {
        Ok(_) => Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("User deleted"))
            .unwrap(),
        Err(err) => {
            eprintln!("Database error: {}", err);
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Failed to delete user"))
                .unwrap()
        }
    }
}
