use axum::http::{Response, StatusCode};
use axum::{
    body::Body,
    routing::{get, post},
    Router,
};
use uuid::Uuid;

use crate::models::user::User;

pub fn users_routes() -> Router {
    Router::new()
        .route("/:uuid", get(get_user_by_id))
        .route("/create", post(create_user))
}

// return json response with user data
async fn get_user_by_id(params: axum::extract::Path<String>) -> impl axum::response::IntoResponse {
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

    // TODO: fetch the user from the database
    let user = User {
        uuid: uuid.to_string(),
        username: "test_get".to_string(),
        email: "test_get@email.com".to_string(),
    };

    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&user).unwrap()))
        .unwrap()
}

// create a new user and return the user data in a json response
async fn create_user() -> impl axum::response::IntoResponse {
    // TODO: create a new user in the database
    let user = User {
        uuid: Uuid::new_v4().to_string(),
        username: "test_create".to_string(),
        email: "test_create@email.com".to_string(),
    };

    Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&user).unwrap()))
        .unwrap()
}
