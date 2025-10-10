use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use tower_http::services::fs::ServeDir;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod html_serve;

#[derive(Serialize, Deserialize)]
struct HealthResponse {
    status: String,
    version: String,
}

async fn healthcheck() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

#[tokio::main]
async fn main() {
    // Thiết lập logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Closure kiểm tra và tạo các thư mục con trong static_path
    let ensure_dirs = |dirs: &[&PathBuf]| {
        for path in dirs {
            if !path.exists() {
                tracing::warn!("Thư mục không tồn tại: {:?}", path);
                std::fs::create_dir_all(&path).unwrap_or_else(|e| {
                    tracing::error!("Không thể tạo thư mục {:?}: {}", path, e);
                });
            } else {
                tracing::info!("Phục vụ static files từ: {:?}", path);
            }
        }
    };

    const STATIC_PATH: &str = "static";
    const APP_PATH: &str = "_app";
    const ASSETS_PATH: &str = "assets";

    // Đường dẫn đến các thư mục static và assets
    let static_path = PathBuf::from(STATIC_PATH);
    let assets_path = static_path.join(ASSETS_PATH);
    let app_path = static_path.join(APP_PATH);
    // Kiểm tra và tạo các thư mục con cần thiết
    ensure_dirs(&[&static_path, &assets_path, &app_path]);

    // Tạo router với endpoint health check, static assets, và fallback cho HTML
    let app = Router::new()
        .route("/api/health", get(healthcheck))
        // Phục vụ file tĩnh (js, css, images, etc.)
        .nest_service("/_app", ServeDir::new(app_path))
        .nest_service("/assets", ServeDir::new(assets_path))
        // Fallback cho các route còn lại: trả về file HTML phù hợp
        .fallback(html_serve::serve_html)
        .layer(TraceLayer::new_for_http());

    // Thiết lập địa chỉ server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Đang lắng nghe tại http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
