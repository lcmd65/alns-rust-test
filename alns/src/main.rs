use crate::engine::alns::Alns;
use crate::utils::json;
use crate::input::input::InputData;
use std::net::SocketAddr;

mod engine;
mod input;
mod staff;
mod coverage;
mod shift;
mod solution;
mod utils;
mod constraint;
mod violation;
mod executor;
mod test;

use axum::{
    routing::get,
    Router,
    Json,
    response::IntoResponse,
};

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

async fn run_alns(Json(input_data): Json<InputData>) -> impl IntoResponse {
    println!("[validate_input_data]");
    let mut alns = Alns::new(&input_data);
    println!("[alns start]");
    alns.run_iteration();
    println!("[end]");

    Json(alns.solution)
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/run-alns-rust", get(run_alns));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}