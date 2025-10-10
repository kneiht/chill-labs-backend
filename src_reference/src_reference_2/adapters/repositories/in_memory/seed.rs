use crate::entities::{Image, Post, Role, User};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub async fn seed_posts() -> Vec<Post> {
    vec![
        Post::hydrate(crate::entities::HydratePostDto {
            id: Uuid::parse_str("01997199-4f31-7718-a766-687e926dd0b6").unwrap(),
            title: Some("Post 1".to_string()),
            content: Some("Content of post 1".to_string()),
            image_id: None,
            created_at: DateTime::parse_from_rfc3339("2025-09-17T10:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339("2025-09-17T10:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        })
        .await
        .unwrap(),
        Post::hydrate(crate::entities::HydratePostDto {
            id: Uuid::parse_str("01997199-4f31-7341-b70f-64e96841cd7b").unwrap(),
            title: Some("Post 2".to_string()),
            content: Some("Content of post 2".to_string()),
            image_id: None,
            created_at: DateTime::parse_from_rfc3339("2025-09-17T11:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339("2025-09-17T11:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        })
        .await
        .unwrap(),
        Post::hydrate(crate::entities::HydratePostDto {
            id: Uuid::parse_str("01997199-4f31-79a9-9464-31f5e79905cf").unwrap(),
            title: Some("Post 3".to_string()),
            content: Some("Content of post 3".to_string()),
            image_id: None,
            created_at: DateTime::parse_from_rfc3339("2025-09-17T12:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339("2025-09-17T12:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        })
        .await
        .unwrap(),
    ]
}

pub async fn seed_users() -> Vec<User> {
    let hashed_password = bcrypt::hash("123123", 10).unwrap();
    vec![
        User::hydrate(crate::entities::HydrateUserDto {
            id: Uuid::parse_str("01997199-4f31-7718-a766-687e926dd0c7").unwrap(),
            name: Some("admin".to_string()),
            email: "admin@gmail.com".to_string(),
            hashed_password: hashed_password.clone(),
            role: Role::ADMIN,
            created_at: DateTime::parse_from_rfc3339("2025-09-17T10:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339("2025-09-17T10:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        })
        .await
        .unwrap(),
        User::hydrate(crate::entities::HydrateUserDto {
            id: Uuid::parse_str("01997199-4f31-7718-a766-687e926dd0c8").unwrap(),
            name: Some("user1".to_string()),
            email: "user1@gmail.com".to_string(),
            hashed_password: hashed_password.clone(),
            role: Role::USER,
            created_at: DateTime::parse_from_rfc3339("2025-09-17T10:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339("2025-09-17T10:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        })
        .await
        .unwrap(),
        User::hydrate(crate::entities::HydrateUserDto {
            id: Uuid::parse_str("01997199-4f31-7718-a766-687e926dd0c9").unwrap(),
            name: Some("user2".to_string()),
            email: "user2@gmail.com".to_string(),
            hashed_password: hashed_password.clone(),
            role: Role::USER,
            created_at: DateTime::parse_from_rfc3339("2025-09-17T10:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339("2025-09-17T10:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        })
        .await
        .unwrap(),
    ]
}

pub async fn seed_images() -> Vec<Image> {
    vec![
        Image::hydrate(crate::entities::HydrateImageDto {
            id: Uuid::parse_str("01997199-4f31-7718-a766-687e926dd0d1").unwrap(),
            url: "https://example.com/image1.jpg".to_string(),
            created_at: DateTime::parse_from_rfc3339("2025-09-17T10:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339("2025-09-17T10:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        })
        .await
        .unwrap(),
        Image::hydrate(crate::entities::HydrateImageDto {
            id: Uuid::parse_str("01997199-4f31-7718-a766-687e926dd0d2").unwrap(),
            url: "https://example.com/image2.jpg".to_string(),
            created_at: DateTime::parse_from_rfc3339("2025-09-17T11:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339("2025-09-17T11:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        })
        .await
        .unwrap(),
        Image::hydrate(crate::entities::HydrateImageDto {
            id: Uuid::parse_str("01997199-4f31-7718-a766-687e926dd0d3").unwrap(),
            url: "https://example.com/image3.jpg".to_string(),
            created_at: DateTime::parse_from_rfc3339("2025-09-17T12:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339("2025-09-17T12:00:00.000Z")
                .unwrap()
                .with_timezone(&Utc),
        })
        .await
        .unwrap(),
    ]
}
