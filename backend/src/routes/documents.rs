use crate::db::document_queries;
use crate::db::user_queries;
use crate::models::document::Document;
use axum::extract::State;
use axum::Json;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use uuid::Uuid;

pub fn document_routes(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route("/:uuid", get(get_document_by_uuid))
        .route("/all", get(get_all_documents_by_user_uuid))
        .route("/create", post(create_document))
        .route("/update", put(update_document))
        .route("/delete/:uuid", delete(delete_document))
        .with_state(pool)
}

async fn get_document_by_uuid(
    State(pool): State<sqlx::PgPool>,
    params: axum::extract::Path<String>,
) -> Result<Json<Document>, String> {
    let uuid = match Uuid::parse_str(&params) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err("Invalid UUID".to_string());
        }
    };

    let user_uuid = match user_queries::fetch_user_by_uuid(&pool, uuid).await {
        Ok(user) => user.uuid,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err("Failed to fetch user".to_string());
        }
    };

    let document = match document_queries::fetch_document_by_uuid(&pool, uuid, user_uuid).await {
        Ok(document) => document,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err("Failed to fetch document".to_string());
        }
    };

    Ok(Json(document))
}

async fn get_all_documents_by_user_uuid(
    State(pool): State<sqlx::PgPool>,
    params: axum::extract::Path<String>,
) -> Result<Json<Vec<Document>>, String> {
    let uuid = match Uuid::parse_str(&params) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err("Invalid UUID".to_string());
        }
    };

    let documents = match document_queries::fetch_all_documents_for_user(&pool, uuid).await {
        Ok(documents) => documents,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err("Failed to fetch all documents".to_string());
        }
    };

    Ok(Json(documents))
}

async fn create_document(
    State(pool): State<sqlx::PgPool>,
    request: Json<Document>,
) -> Result<Json<Document>, String> {
    let request_body = request.0;
    let uuid = request_body.uuid;
    let title = request_body.title;
    let content = request_body.content;
    let user_uuid = request_body.user_uuid;

    let document = match document_queries::create_document(
        &pool,
        uuid,
        user_uuid,
        title.clone().as_str(),
        content.clone().as_str(),
    )
    .await
    {
        Ok(document) => document,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err("Failed to create document".to_string());
        }
    };

    Ok(Json(document))
}

async fn update_document(
    State(pool): State<sqlx::PgPool>,
    request: Json<Document>,
) -> Result<Json<Document>, String> {
    let request_body = request.0;
    let uuid = request_body.uuid;
    let title = request_body.title;
    let content = request_body.content;
    let user_uuid = request_body.user_uuid;

    let document = match document_queries::update_document(
        &pool,
        uuid,
        user_uuid,
        title.clone().as_str(),
        content.clone().as_str(),
    )
    .await
    {
        Ok(document) => document,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err("Failed to update document".to_string());
        }
    };

    Ok(Json(document))
}

async fn delete_document(
    State(pool): State<sqlx::PgPool>,
    params: axum::extract::Path<String>,
) -> Result<Json<Document>, String> {
    let uuid = match Uuid::parse_str(&params) {
        Ok(uuid) => uuid,
        Err(_) => {
            return Err("Invalid UUID".to_string());
        }
    };

    let user_uuid = match user_queries::fetch_user_by_uuid(&pool, uuid).await {
        Ok(user) => user.uuid,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err("Failed to fetch user".to_string());
        }
    };

    let document = match document_queries::delete_document(&pool, uuid, user_uuid).await {
        Ok(document) => document,
        Err(err) => {
            eprintln!("Database error: {}", err);
            return Err("Failed to delete document".to_string());
        }
    };

    Ok(Json(document))
}
