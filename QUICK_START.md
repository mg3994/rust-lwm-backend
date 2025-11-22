# Quick Start Guide

## Prerequisites

1. **MySQL Server** running at `C:\dev\mysql`
   - Username: `root`
   - No password
   - Database: `rotiride`

2. **Rust** installed (latest stable)

3. **Environment Setup**:
   - Copy `.env.example` to `.env` (if exists)
   - Add `firebase-service-account.json` with your Firebase credentials

## Step-by-Step Setup

### 1. Database Setup

```powershell
# Start MySQL
C:\dev\mysql\toggle_mysql.bat

# Create database
C:\dev\mysql\bin\mysql.exe -u root -e "CREATE DATABASE IF NOT EXISTS rotiride CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;"

# Apply schema
Get-Content schema.sql | C:\dev\mysql\bin\mysql.exe -u root rotiride
```

### 2. Configure Environment

Edit `.env`:
```env
# Firebase Configuration
FIREBASE_PROJECT_ID=your-project-id
FIREBASE_API_KEY=your-api-key
# ... (other Firebase settings)

# Database Configuration
DB_HOST=127.0.0.1
DB_PORT=3306
DB_NAME=rotiride
DB_USERNAME=root
DB_PASSWORD=

# Server Configuration
HOST=127.0.0.1
PORT=3000
```

### 3. Start the Server

```powershell
cd backend
cargo run
```

You should see:
```
Starting server on 127.0.0.1:3000
Generating self-signed certificates...
Certificates generated successfully.
Initializing MySQL connection to rotiride...
Successfully connected to MySQL!
Initializing Firebase Client...
HTTP/3 server listening on 127.0.0.1:3000
gRPC server listening on 127.0.0.1:3001
```

### 4. Test with Client

**Option A: Quick Test (Both Protocols)**
```powershell
cd client
cargo run
```

**Option B: Comprehensive gRPC Tests**
```powershell
cd client
cargo run --example grpc_test
```

**Option C: Comprehensive HTTP/3 Tests**
```powershell
cd client
cargo run --example http3_test
```

## Common Issues

### Build Fails with File Locking Error
**Solution**: See `client/BUILD_STATUS.md` for troubleshooting

### MySQL Connection Failed
**Solution**: 
```powershell
# Check MySQL status
C:\dev\mysql\mysql_status.bat

# Start MySQL if not running
C:\dev\mysql\toggle_mysql.bat
```

### Port Already in Use
**Solution**: Change `PORT` in `.env` to a different port (e.g., 3002)

## Testing Individual Services

### Test gRPC with grpcurl

```bash
# Install grpcurl
# https://github.com/fullstorydev/grpcurl

# List services
grpcurl -plaintext localhost:3001 list

# Test Ping
grpcurl -plaintext -d '{"message": "test"}' localhost:3001 service.LinkWithMentor/Ping

# Create User
grpcurl -plaintext -d '{
  "firebase_uid": "test123",
  "email": "test@example.com",
  "display_name": "Test User"
}' localhost:3001 service.LinkWithMentor/CreateUser
```

### Test HTTP/3 with Custom Client

See `client/examples/http3_test.rs` for a complete example.

## Next Steps

1. ✅ Server is running
2. ✅ Database is connected
3. ✅ Tests are passing

Now you can:
- Integrate with your Flutter/React app
- Add more gRPC methods
- Implement authentication
- Deploy to production

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    Client Applications                   │
│              (Flutter, React, Mobile, etc.)              │
└────────────────┬────────────────────┬───────────────────┘
                 │                    │
         HTTP/3 (QUIC)              gRPC
         Port 3000                Port 3001
                 │                    │
                 ▼                    ▼
┌─────────────────────────────────────────────────────────┐
│                  Rust Backend Server                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │   HTTP/3     │  │     gRPC     │  │   Firebase   │  │
│  │   Server     │  │    Server    │  │     FCM      │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│                          │                               │
│                          ▼                               │
│                  ┌──────────────┐                        │
│                  │  AppState    │                        │
│                  │  (Arc)       │                        │
│                  └──────────────┘                        │
│                          │                               │
│                          ▼                               │
│                  ┌──────────────┐                        │
│                  │  MySQL Pool  │                        │
│                  │  (sqlx)      │                        │
│                  └──────────────┘                        │
└─────────────────────────┬───────────────────────────────┘
                          │
                          ▼
                  ┌──────────────┐
                  │    MySQL     │
                  │   Database   │
                  │  (rotiride)  │
                  └──────────────┘
```

## Support

For issues or questions:
1. Check `IMPLEMENTATION_SUMMARY.md` for feature list
2. See `client/BUILD_STATUS.md` for build issues
3. Review `client/TESTING.md` for testing guide
