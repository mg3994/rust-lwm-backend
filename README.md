# LinkWithMentor Backend - Project Structure

## Overview
This repository contains the LinkWithMentor backend server and test client.

```
backend/
├── src/              # Server source code
│   ├── main.rs       # Entry point, AppState
│   ├── config.rs     # Environment configuration
│   ├── db.rs         # Database layer (sqlx + MySQL)
│   ├── models.rs     # Data models
│   ├── server.rs     # HTTP/3 server (QUIC)
│   ├── grpc.rs       # gRPC server
│   ├── firebase.rs   # Firebase Auth & FCM
│   └── cert.rs       # SSL certificate generation
├── client/           # Test client workspace
│   ├── src/
│   │   └── main.rs   # Client testing tool
│   ├── build.rs      # Proto compilation
│   ├── README.md     # Client documentation
│   └── TESTING.md    # Testing guide
├── proto/            # Protocol Buffers definitions
│   └── service.proto # gRPC service definitions
├── schema.sql        # MySQL database schema
├── .env              # Environment variables (not in git)
└── firebase-service-account.json  # Firebase credentials (not in git)
```

## Quick Start

### 1. Setup Environment
```bash
# Copy and configure .env
cp .env.example .env

# Add your Firebase service account JSON
# Place it as firebase-service-account.json
```

### 2. Setup Database
```bash
# Create database
mysql -u root -e "CREATE DATABASE rotiride CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"

# Apply schema
mysql -u root rotiride < schema.sql
```

### 3. Run Server
```bash
cargo run
```

The server will start:
- HTTP/3 (QUIC): `localhost:3000`
- gRPC: `localhost:3001`

### 4. Test with Client
```bash
cd client
cargo run
```

## Features

✅ **Implemented:**
- HTTP/3 server with QUIC transport
- gRPC server with Ping service
- MySQL database with sqlx
- Firebase Authentication (JWT)
- Firebase Cloud Messaging (FCM)
- Hybrid notifications (link, image, chat, call)
- SSL certificate auto-generation
- Shared application state
- Complete CRUD operations
- Test client workspace

🚧 **In Progress:**
- Authentication middleware
- Additional gRPC services
- HTTP/3 route handlers

## Database Schema

- **users**: User accounts with Firebase integration
- **sessions**: Mentorship sessions
- **notifications**: Push notification tracking
- **device_tokens**: FCM device tokens

## Technologies

- **Runtime**: Tokio (async)
- **HTTP/3**: tokio-quiche
- **gRPC**: tonic + prost
- **Database**: MySQL + sqlx
- **Auth**: Firebase Admin SDK
- **Notifications**: Firebase Cloud Messaging

## Environment Variables

See `.env` file for configuration:
- Firebase credentials
- Database connection
- Server host/port

## Testing

See `client/TESTING.md` for detailed testing instructions.
