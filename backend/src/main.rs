mod db;
mod models;
mod routes;
mod utils;
use axum::Router;
use dotenv::dotenv;
use http::{HeaderValue, Method};
use http::header::CONTENT_TYPE;
use routes::auth::google_auth_router;
use routes::documents::document_routes;
use routes::users::users_routes;
use std::env;
use std::time::Duration;
use tokio::time;
use tower_http::{cors::CorsLayer, services::ServeDir};

#[tokio::main]
async fn main() {
    dotenv().ok();
    // connect to the database and run migrations
    let pool = db::connection::connect(env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap_or_else(|err| {
            eprintln!("Database error: {}", err);
            std::process::exit(1);
        });

    // bind the server to the address and port
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    // spawn a task to delete expired sessions periodically
    tokio::spawn(delete_expired_sessions_periodically(pool.clone()));

    let cors_origin = env::var("CLIENT_URL")
        .unwrap()
        .as_str()
        .parse::<HeaderValue>()
        .unwrap();
    let cors_middleware = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT])
        .allow_origin(cors_origin)
        .allow_headers([CONTENT_TYPE])
        .allow_credentials(true);

    let auth_router = google_auth_router(pool.clone()).layer(cors_middleware.clone());
    let users_router = users_routes(pool.clone()).layer(cors_middleware.clone());
    let documents_router = document_routes(pool.clone()).layer(cors_middleware.clone());

    let dist_dir = if cfg!(debug_assertions) {
        "../frontend/dist/"
    } else {
        "./dist"
    };

    let app = Router::new()
        .nest_service("/", ServeDir::new(dist_dir))
        .nest("/auth", auth_router)
        .nest("/users", users_router)
        .nest("/documents", documents_router);

    // start the server
    axum::serve(listener, app).await.unwrap_or_else(|err| {
        eprintln!("Server error: {}", err);
        std::process::exit(1);
    })
}

async fn delete_expired_sessions_periodically(pool: sqlx::PgPool) {
    // Run indefinitely
    loop {
        // Wait for 1 hour before deleting expired sessions again
        time::sleep(Duration::from_secs(3600)).await;

        // Delete expired sessions
        if let Err(err) = db::user_queries::delete_expired_sessions(&pool).await {
            eprintln!("Error deleting expired sessions: {}", err);
        }
    }
}
