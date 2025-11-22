use tonic::Request;
use std::time::Duration;

pub mod pb {
    tonic::include_proto!("service");
}

use pb::link_with_mentor_client::LinkWithMentorClient;
use pb::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 LinkWithMentor gRPC Client - Comprehensive Testing\n");
    println!("=" .repeat(60));

    // Connect to gRPC server
    println!("\n📡 Connecting to gRPC server at http://127.0.0.1:3001...");
    let mut client = LinkWithMentorClient::connect("http://127.0.0.1:3001").await?;
    println!("✅ Connected!\n");

    // Test 1: Ping
    println!("=" .repeat(60));
    println!("Test 1: Ping Service");
    println!("=" .repeat(60));
    test_ping(&mut client).await?;

    // Test 2: Create User
    println!("\n" .repeat(1) + &"=".repeat(60));
    println!("Test 2: Create User");
    println!("=" .repeat(60));
    let user_id = test_create_user(&mut client).await?;

    // Test 3: Get User
    println!("\n" .repeat(1) + &"=".repeat(60));
    println!("Test 3: Get User");
    println!("=" .repeat(60));
    test_get_user(&mut client, user_id).await?;

    // Test 4: Create Session
    println!("\n" .repeat(1) + &"=".repeat(60));
    println!("Test 4: Create Session");
    println!("=" .repeat(60));
    test_create_session(&mut client, user_id).await?;

    // Test 5: Get User Sessions
    println!("\n" .repeat(1) + &"=".repeat(60));
    println!("Test 5: Get User Sessions");
    println!("=" .repeat(60));
    test_get_sessions(&mut client, user_id).await?;

    // Test 6: Register Device Token
    println!("\n" .repeat(1) + &"=".repeat(60));
    println!("Test 6: Register Device Token");
    println!("=" .repeat(60));
    test_register_device_token(&mut client, user_id).await?;

    // Test 7: Send Notification
    println!("\n" .repeat(1) + &"=".repeat(60));
    println!("Test 7: Send Notification");
    println!("=" .repeat(60));
    test_send_notification(&mut client, user_id).await?;

    // Test 8: Get Unread Notifications
    println!("\n" .repeat(1) + &"=".repeat(60));
    println!("Test 8: Get Unread Notifications");
    println!("=" .repeat(60));
    test_get_notifications(&mut client, user_id).await?;

    println!("\n" .repeat(1) + &"=".repeat(60));
    println!("✅ All gRPC tests completed successfully!");
    println!("=" .repeat(60));

    Ok(())
}

async fn test_ping(client: &mut LinkWithMentorClient<tonic::transport::Channel>) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(PingRequest {
        message: "Hello from comprehensive test client!".into(),
    });

    let response = client.ping(request).await?;
    println!("✅ Response: {}", response.into_inner().message);
    Ok(())
}

async fn test_create_user(client: &mut LinkWithMentorClient<tonic::transport::Channel>) -> Result<u64, Box<dyn std::error::Error>> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let request = Request::new(CreateUserRequest {
        firebase_uid: format!("test_user_{}", timestamp),
        email: format!("test{}@example.com", timestamp),
        display_name: Some("Test User".to_string()),
        photo_url: Some("https://example.com/photo.jpg".to_string()),
        role: Some("user".to_string()),
    });

    let response = client.create_user(request).await?;
    let user = response.into_inner();
    
    println!("✅ User created:");
    println!("   ID: {}", user.id);
    println!("   Email: {}", user.email);
    println!("   Display Name: {}", user.display_name.unwrap_or_default());
    println!("   Role: {}", user.role);

    Ok(user.id)
}

async fn test_get_user(client: &mut LinkWithMentorClient<tonic::transport::Channel>, user_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(GetUserRequest {
        identifier: Some(get_user_request::Identifier::UserId(user_id)),
    });

    let response = client.get_user(request).await?;
    let user = response.into_inner();
    
    println!("✅ User retrieved:");
    println!("   ID: {}", user.id);
    println!("   Firebase UID: {}", user.firebase_uid);
    println!("   Email: {}", user.email);

    Ok(())
}

async fn test_create_session(client: &mut LinkWithMentorClient<tonic::transport::Channel>, user_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(CreateSessionRequest {
        user_id,
        mentor_id: user_id, // Using same user as mentor for testing
        title: "Test Mentorship Session".to_string(),
        description: Some("This is a test session".to_string()),
        scheduled_at: "2025-12-01 10:00:00".to_string(),
        duration_minutes: Some(60),
        meeting_link: Some("https://meet.example.com/test".to_string()),
    });

    let response = client.create_session(request).await?;
    let session = response.into_inner();
    
    println!("✅ Session created:");
    println!("   ID: {}", session.id);
    println!("   Title: {}", session.title);
    println!("   Scheduled: {}", session.scheduled_at);
    println!("   Duration: {} minutes", session.duration_minutes);

    Ok(())
}

async fn test_get_sessions(client: &mut LinkWithMentorClient<tonic::transport::Channel>, user_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(GetUserSessionsRequest {
        user_id,
    });

    let response = client.get_user_sessions(request).await?;
    let sessions = response.into_inner().sessions;
    
    println!("✅ Retrieved {} session(s):", sessions.len());
    for (i, session) in sessions.iter().enumerate() {
        println!("   {}. {} - {}", i + 1, session.title, session.status);
    }

    Ok(())
}

async fn test_register_device_token(client: &mut LinkWithMentorClient<tonic::transport::Channel>, user_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(RegisterDeviceTokenRequest {
        user_id,
        token: format!("test_fcm_token_{}", user_id),
        device_type: "android".to_string(),
    });

    client.register_device_token(request).await?;
    println!("✅ Device token registered successfully");

    Ok(())
}

async fn test_send_notification(client: &mut LinkWithMentorClient<tonic::transport::Channel>, user_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(SendNotificationRequest {
        user_id,
        title: "Test Notification".to_string(),
        body: "This is a test notification from the gRPC client".to_string(),
        notification_type: "standard".to_string(),
        data: None,
    });

    let response = client.send_notification(request).await?;
    let notification = response.into_inner();
    
    println!("✅ Notification sent:");
    println!("   ID: {}", notification.id);
    println!("   Title: {}", notification.title);
    println!("   Type: {}", notification.notification_type);

    Ok(())
}

async fn test_get_notifications(client: &mut LinkWithMentorClient<tonic::transport::Channel>, user_id: u64) -> Result<(), Box<dyn std::error::Error>> {
    let request = Request::new(GetUnreadNotificationsRequest {
        user_id,
    });

    let response = client.get_unread_notifications(request).await?;
    let notifications = response.into_inner().notifications;
    
    println!("✅ Retrieved {} unread notification(s):", notifications.len());
    for (i, notif) in notifications.iter().enumerate() {
        println!("   {}. {} - {}", i + 1, notif.title, notif.body);
    }

    Ok(())
}
