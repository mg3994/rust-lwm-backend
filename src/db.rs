use crate::config::Config;
use sqlx::{MySql, Pool};

pub type DbPool = Pool<MySql>;

pub async fn init(config: &Config) -> Result<DbPool, Box<dyn std::error::Error>> {
    println!("Initializing MySQL connection to {}...", config.db_name);
    
    // Construct connection string
    let url = if config.db_pass.is_empty() {
        format!(
            "mysql://{}@{}:{}/{}", 
            config.db_user, 
            config.db_host, 
            config.db_port, 
            config.db_name
        )
    } else {
        format!(
            "mysql://{}:{}@{}:{}/{}", 
            config.db_user, 
            config.db_pass, 
            config.db_host, 
            config.db_port, 
            config.db_name
        )
    };

    println!("Connecting to MySQL at {}:{}/{}", config.db_host, config.db_port, config.db_name);
    
    let pool = sqlx::MySqlPool::connect(&url).await?;
    
    println!("Successfully connected to MySQL!");

    Ok(pool)
}

// User CRUD operations
pub async fn create_user(pool: &DbPool, user: &crate::models::CreateUser) -> Result<u64, Box<dyn std::error::Error>> {
    let result = sqlx::query!(
        r#"
        INSERT INTO users (firebase_uid, email, display_name, photo_url, role)
        VALUES (?, ?, ?, ?, COALESCE(?, 'user'))
        "#,
        user.firebase_uid,
        user.email,
        user.display_name,
        user.photo_url,
        user.role
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_id())
}

pub async fn get_user_by_firebase_uid(pool: &DbPool, firebase_uid: &str) -> Result<Option<crate::models::User>, Box<dyn std::error::Error>> {
    let user = sqlx::query_as!(
        crate::models::User,
        r#"SELECT id, firebase_uid, email, display_name, photo_url, role, created_at, updated_at FROM users WHERE firebase_uid = ?"#,
        firebase_uid
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_id(pool: &DbPool, user_id: u64) -> Result<Option<crate::models::User>, Box<dyn std::error::Error>> {
    let user = sqlx::query_as!(
        crate::models::User,
        r#"SELECT id, firebase_uid, email, display_name, photo_url, role, created_at, updated_at FROM users WHERE id = ?"#,
        user_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

// Session CRUD operations
pub async fn create_session(pool: &DbPool, session: &crate::models::CreateSession) -> Result<u64, Box<dyn std::error::Error>> {
    let result = sqlx::query!(
        r#"
        INSERT INTO sessions (user_id, mentor_id, title, description, scheduled_at, duration_minutes, meeting_link)
        VALUES (?, ?, ?, ?, ?, COALESCE(?, 60), ?)
        "#,
        session.user_id,
        session.mentor_id,
        session.title,
        session.description,
        session.scheduled_at,
        session.duration_minutes,
        session.meeting_link
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_id())
}

pub async fn get_sessions_by_user(pool: &DbPool, user_id: u64) -> Result<Vec<crate::models::Session>, Box<dyn std::error::Error>> {
    let sessions = sqlx::query_as!(
        crate::models::Session,
        r#"SELECT id, user_id, mentor_id, title, description, scheduled_at, duration_minutes, status, meeting_link, created_at, updated_at 
        FROM sessions WHERE user_id = ? OR mentor_id = ? ORDER BY scheduled_at DESC"#,
        user_id,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(sessions)
}

// Notification CRUD operations
pub async fn create_notification(pool: &DbPool, notification: &crate::models::CreateNotification) -> Result<u64, Box<dyn std::error::Error>> {
    let result = sqlx::query!(
        r#"
        INSERT INTO notifications (user_id, title, body, notification_type, data)
        VALUES (?, ?, ?, ?, ?)
        "#,
        notification.user_id,
        notification.title,
        notification.body,
        notification.notification_type,
        notification.data
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_id())
}

pub async fn get_unread_notifications(pool: &DbPool, user_id: u64) -> Result<Vec<crate::models::Notification>, Box<dyn std::error::Error>> {
    let notifications = sqlx::query_as!(
        crate::models::Notification,
        r#"SELECT id, user_id, title, body, notification_type, data, is_read as "is_read: bool", created_at 
        FROM notifications WHERE user_id = ? AND is_read = FALSE ORDER BY created_at DESC"#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(notifications)
}

pub async fn mark_notification_read(pool: &DbPool, notification_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query!(
        r#"UPDATE notifications SET is_read = TRUE WHERE id = ?"#,
        notification_id
    )
    .execute(pool)
    .await?;

    Ok(())
}

// Device Token CRUD operations
pub async fn upsert_device_token(pool: &DbPool, token: &crate::models::CreateDeviceToken) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query!(
        r#"
        INSERT INTO device_tokens (user_id, token, device_type)
        VALUES (?, ?, ?)
        ON DUPLICATE KEY UPDATE updated_at = CURRENT_TIMESTAMP
        "#,
        token.user_id,
        token.token,
        token.device_type
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_user_device_tokens(pool: &DbPool, user_id: u64) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let tokens = sqlx::query_scalar!(
        r#"SELECT token FROM device_tokens WHERE user_id = ?"#,
        user_id
    )
    .fetch_all(pool)
    .await?;

    Ok(tokens)
}
