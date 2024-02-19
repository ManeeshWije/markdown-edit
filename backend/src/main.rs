mod db;
mod models;
mod routes;
use axum::Router;
use dotenv::dotenv;
use routes::users::users_routes;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // bind the server to the address
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    // connect to the database and run migrations
    let pool = db::connection::connect(env::var("DATABASE_URL").unwrap().as_str())
        .await
        .unwrap_or_else(|err| {
            eprintln!("Database error: {}", err);
            std::process::exit(1);
        });

    let app = Router::new().nest("/users", users_routes(pool.clone()));

    // start the server
    axum::serve(listener, app).await.unwrap_or_else(|err| {
        eprintln!("Server error: {}", err);
        std::process::exit(1);
    })
}
