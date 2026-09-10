use axum::{extract::{Path, State}, http::StatusCode, routing::{get, put}, Router};
use std::{collections::HashMap, sync::{Arc, RwLock}};

type Store = Arc<RwLock<HashMap<String, Vec<u8>>>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let store = Store::default();
    let app = Router::new()
        .route("/health", get(|| async { "vaz-node:ok" }))
        .route("/v1/shards/{hash}", put(put_shard).get(get_shard))
        .with_state(store);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8787").await?;
    println!("VAZ node alpha on {}", listener.local_addr()?);
    axum::serve(listener, app).await?; Ok(())
}

async fn put_shard(State(store): State<Store>, Path(hash): Path<String>, body: axum::body::Bytes) -> StatusCode {
    if blake3::hash(&body).to_hex().to_string() != hash { return StatusCode::BAD_REQUEST; }
    store.write().unwrap().insert(hash, body.to_vec()); StatusCode::CREATED
}

async fn get_shard(State(store): State<Store>, Path(hash): Path<String>) -> Result<Vec<u8>, StatusCode> {
    store.read().unwrap().get(&hash).cloned().ok_or(StatusCode::NOT_FOUND)
}