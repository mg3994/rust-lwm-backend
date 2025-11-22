# API Documentation

## gRPC API Reference

Base URL: `localhost:3001`

---

## Health & Monitoring

### HealthCheck
Check system health status.

**Request**: `EmptyRequest`

**Response**: `HealthResponse`
```json
{
  "status": "healthy",
  "database": true,
  "firebase": true,
  "uptime_seconds": 3600
}
```

**Example**:
```bash
grpcurl -plaintext localhost:3001 service.LinkWithMentor/HealthCheck
```

---

### GetMetrics
Get system metrics and statistics.

**Request**: `EmptyRequest`

**Response**: `MetricsResponse`
```json
{
  "total_requests": 1000,
  "successful_requests": 950,
  "failed_requests": 50,
  "success_rate": 95.0,
  "total_users_created": 100,
  "total_sessions_created": 50,
  "total_notifications_sent": 200
}
```

**Example**:
```bash
grpcurl -plaintext localhost:3001 service.LinkWithMentor/GetMetrics
```

---

### Ping
Simple health check endpoint.

**Request**: `PingRequest`
```json
{
  "message": "Hello"
}
```

**Response**: `PingResponse`
```json
{
  "message": "Pong: Hello"
}
```

**Example**:
```bash
grpcurl -plaintext -d '{"message": "test"}' localhost:3001 service.LinkWithMentor/Ping
```

---

## User Management

### CreateUser
Create a new user with Firebase authentication.

**Request**: `CreateUserRequest`
```json
{
  "firebase_uid": "abc123",
  "email": "user@example.com",
  "display_name": "John Doe",
  "photo_url": "https://example.com/photo.jpg",
  "role": "user"
}
```

**Response**: `UserResponse`
```json
{
  "id": 1,
  "firebase_uid": "abc123",
  "email": "user@example.com",
  "display_name": "John Doe",
  "photo_url": "https://example.com/photo.jpg",
  "role": "user",
  "created_at": "2025-11-22 12:00:00"
}
```

**Rate Limit**: 100 requests per minute per user

**Example**:
```bash
grpcurl -plaintext -d '{
  "firebase_uid": "abc123",
  "email": "user@example.com",
  "display_name": "John Doe"
}' localhost:3001 service.LinkWithMentor/CreateUser
```

---

### GetUser
Retrieve user by ID or Firebase UID.

**Request**: `GetUserRequest`
```json
{
  "user_id": 1
}
// OR
{
  "firebase_uid": "abc123"
}
```

**Response**: `UserResponse`

**Example**:
```bash
# By user ID
grpcurl -plaintext -d '{"user_id": 1}' localhost:3001 service.LinkWithMentor/GetUser

# By Firebase UID
grpcurl -plaintext -d '{"firebase_uid": "abc123"}' localhost:3001 service.LinkWithMentor/GetUser
```

---

## Session Management

### CreateSession
Schedule a mentorship session.

**Request**: `CreateSessionRequest`
```json
{
  "user_id": 1,
  "mentor_id": 2,
  "title": "Career Guidance",
  "description": "Discuss career path",
  "scheduled_at": "2025-12-01 10:00:00",
  "duration_minutes": 60,
  "meeting_link": "https://meet.example.com/session"
}
```

**Response**: `SessionResponse`
```json
{
  "id": 1,
  "user_id": 1,
  "mentor_id": 2,
  "title": "Career Guidance",
  "description": "Discuss career path",
  "scheduled_at": "2025-12-01 10:00:00",
  "duration_minutes": 60,
  "status": "scheduled",
  "meeting_link": "https://meet.example.com/session"
}
```

**Example**:
```bash
grpcurl -plaintext -d '{
  "user_id": 1,
  "mentor_id": 2,
  "title": "Career Guidance",
  "scheduled_at": "2025-12-01 10:00:00"
}' localhost:3001 service.LinkWithMentor/CreateSession
```

---

### GetUserSessions
Get all sessions for a user (as participant or mentor).

**Request**: `GetUserSessionsRequest`
```json
{
  "user_id": 1
}
```

**Response**: `SessionListResponse`
```json
{
  "sessions": [
    {
      "id": 1,
      "user_id": 1,
      "mentor_id": 2,
      "title": "Career Guidance",
      "scheduled_at": "2025-12-01 10:00:00",
      "duration_minutes": 60,
      "status": "scheduled"
    }
  ]
}
```

**Example**:
```bash
grpcurl -plaintext -d '{"user_id": 1}' localhost:3001 service.LinkWithMentor/GetUserSessions
```

---

## Notifications

### SendNotification
Send a push notification to a user.

**Request**: `SendNotificationRequest`
```json
{
  "user_id": 1,
  "title": "New Message",
  "body": "You have a new message from your mentor",
  "notification_type": "standard",
  "data": "{\"extra\": \"info\"}"
}
```

**Notification Types**:
- `standard` - Basic notification
- `link` - Contains a URL
- `image` - Contains an image URL
- `chat` - Chat message notification
- `call` - Call notification

**Response**: `NotificationResponse`
```json
{
  "id": 1,
  "user_id": 1,
  "title": "New Message",
  "body": "You have a new message from your mentor",
  "notification_type": "standard",
  "data": "{\"extra\": \"info\"}",
  "is_read": false,
  "created_at": "2025-11-22 12:00:00"
}
```

**Example**:
```bash
grpcurl -plaintext -d '{
  "user_id": 1,
  "title": "New Message",
  "body": "You have a new message",
  "notification_type": "standard"
}' localhost:3001 service.LinkWithMentor/SendNotification
```

---

### GetUnreadNotifications
Get all unread notifications for a user.

**Request**: `GetUnreadNotificationsRequest`
```json
{
  "user_id": 1
}
```

**Response**: `NotificationListResponse`
```json
{
  "notifications": [
    {
      "id": 1,
      "user_id": 1,
      "title": "New Message",
      "body": "You have a new message",
      "notification_type": "standard",
      "is_read": false,
      "created_at": "2025-11-22 12:00:00"
    }
  ]
}
```

**Example**:
```bash
grpcurl -plaintext -d '{"user_id": 1}' localhost:3001 service.LinkWithMentor/GetUnreadNotifications
```

---

### MarkNotificationRead
Mark a notification as read.

**Request**: `MarkNotificationReadRequest`
```json
{
  "notification_id": 1
}
```

**Response**: `EmptyResponse`

**Example**:
```bash
grpcurl -plaintext -d '{"notification_id": 1}' localhost:3001 service.LinkWithMentor/MarkNotificationRead
```

---

## Device Management

### RegisterDeviceToken
Register a device token for push notifications.

**Request**: `RegisterDeviceTokenRequest`
```json
{
  "user_id": 1,
  "token": "fcm_token_here",
  "device_type": "android"
}
```

**Device Types**:
- `android`
- `ios`
- `web`

**Response**: `EmptyResponse`

**Example**:
```bash
grpcurl -plaintext -d '{
  "user_id": 1,
  "token": "fcm_token_here",
  "device_type": "android"
}' localhost:3001 service.LinkWithMentor/RegisterDeviceToken
```

---

## Error Codes

| Status Code | Description |
|-------------|-------------|
| `OK` | Success |
| `INVALID_ARGUMENT` | Invalid request parameters |
| `NOT_FOUND` | Resource not found |
| `RESOURCE_EXHAUSTED` | Rate limit exceeded |
| `INTERNAL` | Internal server error |
| `UNAUTHENTICATED` | Authentication required |
| `PERMISSION_DENIED` | Insufficient permissions |

---

## Rate Limiting

- **Global**: 100 requests per minute per user
- **Headers**: Rate limit info not currently exposed (future enhancement)
- **Retry**: Wait 60 seconds before retrying after rate limit

---

## Authentication

Currently, authentication is implemented but not enforced on all endpoints.

**Future**: All endpoints except `Ping` and `HealthCheck` will require Firebase JWT token in metadata:

```
authorization: Bearer <firebase_jwt_token>
```

---

## HTTP/3 API

Base URL: `localhost:3000`

### Raw QUIC Messaging

The HTTP/3 server currently supports raw QUIC bidirectional streams.

**Example** (using quinn client):
```rust
let (mut send, mut recv) = connection.open_bi().await?;
send.write_all(b"Hello").await?;
let response = recv.read_to_end(1024).await?;
```

**Future**: HTTP/3 will support REST-like endpoints over QUIC.

---

## Testing

### Using grpcurl

```bash
# List all services
grpcurl -plaintext localhost:3001 list

# Describe a service
grpcurl -plaintext localhost:3001 describe service.LinkWithMentor

# Call a method
grpcurl -plaintext -d '{"message": "test"}' localhost:3001 service.LinkWithMentor/Ping
```

### Using Test Client

```bash
cd client

# Quick test
cargo run

# Comprehensive gRPC tests
cargo run --example grpc_test

# Comprehensive HTTP/3 tests
cargo run --example http3_test
```

---

## Monitoring

### Metrics Endpoint

Call `GetMetrics` to get real-time statistics:
- Total requests
- Success rate
- Users created
- Sessions created
- Notifications sent

### Health Endpoint

Call `HealthCheck` to verify:
- Database connectivity
- Firebase status
- System uptime

### Logs

Structured logging with tracing:
```bash
# Set log level
export RUST_LOG=debug

# Run server
cargo run
```

---

## Best Practices

1. **Error Handling**: Always check response status
2. **Rate Limiting**: Implement exponential backoff
3. **Timeouts**: Set reasonable request timeouts
4. **Retries**: Retry on transient failures
5. **Monitoring**: Monitor metrics and health endpoints

---

*Last Updated: 2025-11-22*
*API Version: 1.0.0*
