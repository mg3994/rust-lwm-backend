use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u64,
    pub firebase_uid: String,
    pub email: String,
    pub display_name: Option<String>,
    pub photo_url: Option<String>,
    pub role: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub firebase_uid: String,
    pub email: String,
    pub display_name: Option<String>,
    pub photo_url: Option<String>,
    pub role: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: u64,
    pub user_id: u64,
    pub mentor_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub scheduled_at: chrono::NaiveDateTime,
    pub duration_minutes: i32,
    pub status: String,
    pub meeting_link: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSession {
    pub user_id: u64,
    pub mentor_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub scheduled_at: chrono::NaiveDateTime,
    pub duration_minutes: Option<i32>,
    pub meeting_link: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: u64,
    pub user_id: u64,
    pub title: String,
    pub body: String,
    pub notification_type: String,
    pub data: Option<String>, // JSON string
    pub is_read: bool,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNotification {
    pub user_id: u64,
    pub title: String,
    pub body: String,
    pub notification_type: String,
    pub data: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct DeviceToken {
    pub id: u64,
    pub user_id: u64,
    pub token: String,
    pub device_type: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDeviceToken {
    pub user_id: u64,
    pub token: String,
    pub device_type: String,
}
