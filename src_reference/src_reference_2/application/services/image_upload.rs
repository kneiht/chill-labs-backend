use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageResponse {
    pub url: String,
}

pub struct LocalImageUploadService {
    upload_dir: String,
}

impl LocalImageUploadService {
    pub fn new(upload_dir: String) -> Self {
        Self { upload_dir }
    }
}

#[async_trait]
impl ImageUploadService for LocalImageUploadService {
    async fn upload(&self, image: Vec<u8>) -> Result<ImageResponse> {
        // Validate file size (5MB)
        let max_size = 5 * 1024 * 1024;
        if image.len() > max_size {
            anyhow::bail!("File too large. Maximum size is 5MB.");
        }

        // For simplicity, assume PNG
        let ext = "png";
        let filename = format!("{}.{}", Uuid::now_v7(), ext);
        let file_path = Path::new(&self.upload_dir).join(&filename);

        // Ensure directory exists
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Save file
        fs::write(&file_path, image).await?;

        // Return URL
        let url = format!("/uploads/{}", filename);
        Ok(ImageResponse { url })
    }
}

#[async_trait]
pub trait ImageUploadService {
    async fn upload(&self, image: Vec<u8>) -> Result<ImageResponse>;
}
