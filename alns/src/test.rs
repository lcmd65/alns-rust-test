use crate::engine::alns::Alns;
use crate::utils::json;
use crate::input::input::InputData;
use std::net::SocketAddr;

use axum::{
    routing::get,
    Router,
    Json,
    response::IntoResponse,
};

fn main() {
    let input_data = json::read_input_data_from_file("src/resource/dump/data_dummy.json")
        .expect("Failed to read input data from JSON file");

    println!("[validate_input_data]");
    let mut alns = Alns::new(&input_data);
    println!("[alns start]");
    alns.run_iteration();
    println!("[end]");
}


async fn run_alns(Json(input_data): Json<InputData>) -> impl IntoResponse {
    println!("[validate_input_data]");
    let mut alns = Alns::new(&input_data);
    println!("[alns start]");
    alns.run_iteration();
    println!("[end]");

    Json("ALNS run completed")
}

#[tokio::main]
async fn alns() {
    let app = Router::new().route("/run-alns-rust", get(run_alns));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}