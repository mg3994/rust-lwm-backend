# Test Scripts

## Start Backend Server
```bash
cd ..
cargo run
```

## Run Client Tests
```bash
cargo run
```

## Manual gRPC Test with grpcurl
```bash
# Install grpcurl first: https://github.com/fullstorydev/grpcurl

# List services
grpcurl -plaintext localhost:3001 list

# Call Ping
grpcurl -plaintext -d '{"message": "test"}' localhost:3001 service.LinkWithMentor/Ping
```
