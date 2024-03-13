use axum::extract::State;
use axum::http::StatusCode;
use axum::response::ErrorResponse;
use axum::Json;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use axum_extra::extract::cookie::CookieJar;
use uuid::Uuid;

use crate::db::document_queries;
use crate::models::document::Document;
use crate::utils::helpers::check_user_session;

pub fn document_routes(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/:uuid", get(get_document_by_uuid))
        .route("/all", get(get_all_documents_by_user_uuid))
        .route("/create", post(create_document))
        .route("/update/:uuid", put(update_document))
        .route("/delete/:uuid", delete(delete_document))
        .with_state(pool)
}

async fn get_document_by_uuid(
    cookies: CookieJar,
    State(pool): State<sqlx::PgPool>,
    params: axum::extract::Path<String>,
) -> Result<Json<Document>, ErrorResponse> {
    // Parse the UUID from the request parameters
    let uuid = match Uuid::parse_str(&params) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err(ErrorResponse::from(StatusCode::BAD_REQUEST));
        }
    };

    // Check if the user is logged in
    let user_uuid = match check_user_session(cookies, pool.clone()).await {
        Ok(user) => user.uuid,
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            return Err(ErrorResponse::from(StatusCode::UNAUTHORIZED));
        }
    };

    // Fetch the document from the database
    let document = match document_queries::fetch_document_by_uuid(&pool, uuid, user_uuid).await {
        Ok(document) => document,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err(ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR));
        }
    };

    Ok(Json(document))
}

async fn get_all_documents_by_user_uuid(
    cookies: CookieJar,
    State(pool): State<sqlx::PgPool>,
) -> Result<Json<Vec<Document>>, ErrorResponse> {
    // Check if the user is logged in
    let user_uuid = match check_user_session(cookies, pool.clone()).await {
        Ok(user) => user.uuid,
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            return Err(ErrorResponse::from(StatusCode::UNAUTHORIZED));
        }
    };

    // Fetch all documents from the database
    let documents = match document_queries::fetch_all_documents_for_user(&pool, user_uuid).await {
        Ok(documents) => documents,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err(ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR));
        }
    };

    Ok(Json(documents))
}

async fn create_document(
    cookies: CookieJar,
    State(pool): State<sqlx::PgPool>,
    request: Json<Document>,
) -> Result<Json<Document>, ErrorResponse> {
    // Check if the user is logged in
    let user_uuid = match check_user_session(cookies, pool.clone()).await {
        Ok(user) => user.uuid,
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            return Err(ErrorResponse::from(StatusCode::UNAUTHORIZED));
        }
    };

    // Parse the request body
    let request_body = request.0;
    let title = request_body.title;
    let content = request_body.content;
    let uuid = request_body.uuid;

    // Create the document in the database
    let document = match document_queries::create_document(
        &pool,
        uuid.clone().unwrap_or(Uuid::new_v4()),
        user_uuid,
        title.clone().expect("title is required").as_str(),
        content.clone().expect("content is required").as_str(),
    )
    .await
    {
        Ok(document) => document,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err(ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR));
        }
    };

    Ok(Json(document))
}

async fn update_document(
    cookies: CookieJar,
    State(pool): State<sqlx::PgPool>,
    params: axum::extract::Path<String>,
    request: Json<Document>,
) -> Result<Json<Document>, ErrorResponse> {
    // Check if the user is logged in
    let user_uuid = match check_user_session(cookies, pool.clone()).await {
        Ok(user) => user.uuid,
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            return Err(ErrorResponse::from(StatusCode::UNAUTHORIZED));
        }
    };

    // Parse the UUID from the request parameters
    let uuid = match Uuid::parse_str(&params) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err(ErrorResponse::from(StatusCode::BAD_REQUEST));
        }
    };

    // Parse the request body
    let request_body = request.0;
    let title = request_body.title;
    let content = request_body.content;
    let updated_at = request_body.updated_at;

    // Update the document in the database
    let document = match document_queries::update_document(
        &pool,
        uuid,
        user_uuid,
        title.clone().expect("title is required").as_str(),
        content.clone().expect("content is required").as_str(),
        updated_at.clone().expect("updated_at is required").as_str(),
    )
    .await
    {
        Ok(document) => document,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err(ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR));
        }
    };

    Ok(Json(document))
}

async fn delete_document(
    cookies: CookieJar,
    State(pool): State<sqlx::PgPool>,
    params: axum::extract::Path<String>,
) -> Result<Json<Document>, ErrorResponse> {
    // Parse the UUID from the request parameters
    let uuid = match Uuid::parse_str(&params) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err(ErrorResponse::from(StatusCode::BAD_REQUEST));
        }
    };

    // Check if the user is logged in
    let user_uuid = match check_user_session(cookies, pool.clone()).await {
        Ok(user) => user.uuid,
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            return Err(ErrorResponse::from(StatusCode::UNAUTHORIZED));
        }
    };

    // Delete the document from the database
    let document = match document_queries::delete_document(&pool, uuid, user_uuid).await {
        Ok(document) => document,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err(ErrorResponse::from(StatusCode::INTERNAL_SERVER_ERROR));
        }
    };

    Ok(Json(document))
}
