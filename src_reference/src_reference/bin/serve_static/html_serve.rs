use axum::{
    body::Body,
    extract::OriginalUri,
    http::{Response, StatusCode},
    response::IntoResponse,
};

use std::path::Path as FsPath;
use tokio::fs;

pub async fn serve_html(uri: OriginalUri) -> impl IntoResponse {
    let static_dir = FsPath::new("static");
    let path = uri.path();

    // Xử lý path: "/" => "index.html", "/login" => "login.html", "/example" => "example.html" hoặc "example/index.html"
    let mut candidates = vec![];

    if path == "/" {
        candidates.push(static_dir.join("index.html"));
    } else {
        let clean_path = path.trim_start_matches('/');

        // /abc => static/abc.html
        candidates.push(static_dir.join(format!("{}.html", clean_path)));
        // /abc => static/abc/index.html
        candidates.push(static_dir.join(clean_path).join("index.html"));
    }

    for candidate in candidates {
        if candidate.exists() {
            if let Ok(contents) = fs::read(&candidate).await {
                return Response::builder()
                    .status(StatusCode::OK)
                    .header("Content-Type", "text/html; charset=utf-8")
                    .body(Body::from(contents))
                    .unwrap();
            }
        }
    }

    // Không tìm thấy file phù hợp
    let error_404_path = static_dir.join("404.html");
    if error_404_path.exists() {
        if let Ok(contents) = fs::read(&error_404_path).await {
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("Content-Type", "text/html; charset=utf-8")
                .body(Body::from(contents))
                .unwrap();
        }
    }

    // Không tìm thấy file phù hợp
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("404 Not Found"))
        .unwrap()
}
