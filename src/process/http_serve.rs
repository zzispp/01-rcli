use anyhow::Result;
use axum::{extract::State, routing::get, Router};
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::fs;
use tracing::info;

struct HttpServeState {
    path: PathBuf,
}

pub async fn process_http_serve(path: PathBuf, port: u16) -> Result<()> {
    info!("Serving {:?} on port {}", path, port);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    info!("Listening {:?} on {}", path, addr);
    let state = HttpServeState { path };

    let router = Router::new()
        .route("/{*path}", get(index_handler))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await.unwrap();

    Ok(())
}

async fn index_handler(
    State(state): State<Arc<HttpServeState>>,
    axum::extract::Path(key): axum::extract::Path<String>,
) -> (axum::http::StatusCode, String) {
    let path = state.path.clone();
    info!("Serving {:?} on port {}", path, 8080);
    let file_path = Path::new(&path).join(key);
    info!("File path: {:?}", file_path);
    if file_path.exists() {
        match fs::read_to_string(file_path).await {
            Ok(content) => (axum::http::StatusCode::OK, content),
            Err(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "500 Internal Server Error".to_string(),
            ),
        }
    } else {
        (
            axum::http::StatusCode::NOT_FOUND,
            "404 Not Found".to_string(),
        )
    }
}
