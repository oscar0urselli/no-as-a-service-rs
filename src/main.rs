use std::{fs::File, io::Read, sync::{Arc}};

use rand::prelude::*;

use axum::{
    extract::State, routing::get, Json, Router, debug_handler
};

use serde::Serialize;
use tokio::sync::Mutex;


struct AppState {
    reasons: Vec<String>
} 

#[tokio::main]
async fn main() {
    let mut file: File = File::open("reasons.json").unwrap();
    let mut cnt = String::new();
    file.read_to_string(&mut cnt).unwrap();

    let shared_state = Arc::new(Mutex::new(AppState {
        reasons: serde_json::from_str(&cnt).unwrap()
    }));

    let app = Router::new()
        .route("/no", get(handler))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct ReasonRes {
    reason: String
}

#[debug_handler]
async fn handler(
    State(state): State<Arc<Mutex<AppState>>>
) -> Json<ReasonRes> {
    let mtx = state.lock().await;
    let upper_bound = mtx.reasons.len();
    let mut rng = rand::rng();

    Json(ReasonRes {
        reason: mtx.reasons.get(rng.random_range(..upper_bound)).unwrap().clone()
    })
}