// #[macro_use]
// extern crate log;
// extern crate pretty_env_logger;

use dotenvy::dotenv;
//use log::info;
use pretty_env_logger;
//use sqlx::postgres::PgPoolOptions;

use axum::{
    Router,
    routing::{delete, get, post},
};

mod handlers;
mod models;
mod persistance;

use handlers::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
    //     .await
    //     .expect("Failed to connect to database");

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
