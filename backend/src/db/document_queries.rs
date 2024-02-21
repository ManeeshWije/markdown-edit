use crate::models::document::Document;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn fetch_document_by_uuid(
    pool: &PgPool,
    uuid: Uuid,
    user_uuid: Uuid,
) -> Result<Document, sqlx::Error> {
    let document = sqlx::query!(
        "
        SELECT d.uuid, d.user_uuid, d.title, d.content, d.created_at, d.updated_at
        FROM documents AS d
        LEFT JOIN users AS u ON d.user_uuid = u.uuid
        WHERE d.uuid = $1 AND d.user_uuid = $2
        ",
        uuid,
        user_uuid
    )
    .fetch_one(pool)
    .await?;

    Ok(Document {
        uuid: document.uuid,
        user_uuid: document.user_uuid.unwrap(),
        title: document.title,
        content: document.content,
        created_at: Some(document.created_at.unwrap().to_string()),
        updated_at: Some(document.updated_at.unwrap().to_string()),
    })
}

pub async fn fetch_all_documents_for_user(
    pool: &PgPool,
    user_uuid: Uuid,
) -> Result<Vec<Document>, sqlx::Error> {
    let documents = sqlx::query!(
        "
        SELECT d.uuid, d.user_uuid, d.title, d.content, d.created_at, d.updated_at
        FROM documents AS d
        LEFT JOIN users AS u ON d.user_uuid = u.uuid
        WHERE d.user_uuid = $1
        ",
        user_uuid
    )
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();
    for document in documents {
        result.push(Document {
            uuid: document.uuid,
            user_uuid: document.user_uuid.unwrap(),
            title: document.title,
            content: document.content,
            created_at: Some(document.created_at.unwrap().to_string()),
            updated_at: Some(document.updated_at.unwrap().to_string()),
        });
    }

    Ok(result)
}

pub async fn create_document(
    pool: &PgPool,
    uuid: Uuid,
    user_uuid: Uuid,
    title: &str,
    content: &str,
) -> Result<Document, sqlx::Error> {
    let document = sqlx::query!(
        "
        INSERT INTO documents (uuid, user_uuid, title, content, created_at, updated_at)
        VALUES ($1, $2, $3, $4, DEFAULT, DEFAULT)
        RETURNING *
        ",
        uuid,
        user_uuid,
        title,
        content
    )
    .fetch_one(pool)
    .await?;

    Ok(Document {
        uuid: document.uuid,
        user_uuid: document.user_uuid.unwrap(),
        title: document.title,
        content: document.content,
        created_at: Some(document.created_at.unwrap().to_string()),
        updated_at: Some(document.updated_at.unwrap().to_string()),
    })
}

pub async fn update_document(
    pool: &PgPool,
    uuid: Uuid,
    user_uuid: Uuid,
    title: &str,
    content: &str,
) -> Result<Document, sqlx::Error> {
    let document = sqlx::query!(
        "
        UPDATE documents
        SET title = $1, content = $2, updated_at = DEFAULT
        WHERE uuid = $3 AND user_uuid = $4
        RETURNING *
        ",
        title,
        content,
        uuid,
        user_uuid
    )
    .fetch_one(pool)
    .await?;

    Ok(Document {
        uuid: document.uuid,
        user_uuid: document.user_uuid.unwrap(),
        title: document.title,
        content: document.content,
        created_at: Some(document.created_at.unwrap().to_string()),
        updated_at: Some(document.updated_at.unwrap().to_string()),
    })
}

pub async fn delete_document(
    pool: &PgPool,
    uuid: Uuid,
    user_uuid: Uuid,
) -> Result<Document, sqlx::Error> {
    let document = sqlx::query!(
        "
        DELETE FROM documents
        WHERE uuid = $1 AND user_uuid = $2
        RETURNING *
        ",
        uuid,
        user_uuid
    )
    .fetch_one(pool)
    .await?;

    Ok(Document {
        uuid: document.uuid,
        user_uuid: document.user_uuid.unwrap(),
        title: document.title,
        content: document.content,
        created_at: Some(document.created_at.unwrap().to_string()),
        updated_at: Some(document.updated_at.unwrap().to_string()),
    })
}
