# LinkWithMentor Test Client

A simple CLI client to test the LinkWithMentor backend services.

## Features

- gRPC service testing
- HTTP/3 endpoint testing (future)
- Firebase notification testing (future)

## Usage

```bash
# Run the client
cargo run

# Run specific tests
cargo run -- --grpc
cargo run -- --http3
```

## Testing

Make sure the backend server is running on:
- gRPC: `localhost:3001`
- HTTP/3: `localhost:3000`

Then run:
```bash
cargo run
```
