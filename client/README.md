# LinkWithMentor Test Client

A comprehensive CLI client to test both HTTP/3 (QUIC) and gRPC services of the LinkWithMentor backend.

## Features

- ✅ **HTTP/3 (QUIC) Testing**: Tests raw QUIC connection and bidirectional streams
- ✅ **gRPC Service Testing**: Tests gRPC Ping service
- ✅ **Self-signed Certificate Support**: Accepts self-signed SSL certificates for local testing
- 🚧 **Firebase Notification Testing**: Coming soon

## Usage

### Prerequisites
Make sure the backend server is running:
```bash
cd ..
cargo run
```

The server should be listening on:
- HTTP/3 (QUIC): `localhost:3000`
- gRPC: `localhost:3001`

### Run Tests
```bash
cargo run
```

You should see output like:
```
🚀 LinkWithMentor Client - Testing Tool

==================================================

📡 Testing HTTP/3 (QUIC) connection...
  → Connecting to 127.0.0.1:3000...
  → Connected! Opening bidirectional stream...
  → Sent: Hello from HTTP/3 client!
  → Received: Hello from LinkWithMentor HTTP/3 (Raw QUIC)
✅ HTTP/3 test passed!

📡 Testing gRPC connection...
  → Connecting to http://127.0.0.1:3001...
  → Sending Ping request...
  → Response: Pong: Hello from gRPC client!
✅ gRPC test passed!

==================================================
✅ All tests completed!
```

## What It Tests

### HTTP/3 (QUIC)
- Connection establishment with self-signed certificates
- Bidirectional stream creation
- Message sending and receiving
- Proper connection closure

### gRPC
- Service connection
- Ping RPC method
- Message serialization/deserialization

## Advanced Testing

The client includes comprehensive test examples in the `examples/` directory:

### Run gRPC Comprehensive Tests
```bash
cargo run --example grpc_test
```

This will test all 9 gRPC methods:
- CreateUser
- GetUser
- CreateSession
- GetUserSessions
- RegisterDeviceToken
- SendNotification
- GetUnreadNotifications
- MarkNotificationRead
- Ping

### Run HTTP/3 Comprehensive Tests
```bash
cargo run --example http3_test
```

This will test:
- Basic QUIC connection
- Multiple concurrent streams
- Large message transfer (10KB)

## Example Output

### gRPC Test Output:
```
🚀 LinkWithMentor gRPC Client - Comprehensive Testing

============================================================
Test 1: Ping Service
============================================================
✅ Response: Pong: Hello from comprehensive test client!

============================================================
Test 2: Create User
============================================================
✅ User created:
   ID: 1
   Email: test1732265234@example.com
   Display Name: Test User
   Role: user

... (more tests)

============================================================
✅ All gRPC tests completed successfully!
============================================================
```

### HTTP/3 Test Output:
```
🚀 LinkWithMentor HTTP/3 Client - Comprehensive Testing

============================================================
Test 1: Basic Connection
============================================================
  → Connecting to 127.0.0.1:3000...
  → Connected! Opening stream...
  → Sent: Hello from HTTP/3 test client!
  → Received: Hello from LinkWithMentor HTTP/3 (Raw QUIC)
✅ Basic connection test passed

... (more tests)

============================================================
✅ All HTTP/3 tests completed successfully!
============================================================
```
