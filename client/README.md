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
