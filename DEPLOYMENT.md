# Deployment Guide

## Docker Deployment

### 1. Create Dockerfile

```dockerfile
# Build stage
FROM rust:1.75 as builder

WORKDIR /app
COPY . .

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/backend /app/backend
COPY --from=builder /app/schema.sql /app/schema.sql

# Expose ports
EXPOSE 3000 3001

# Run the application
CMD ["./backend"]
```

### 2. Create docker-compose.yml

```yaml
version: '3.8'

services:
  mysql:
    image: mysql:8.0
    environment:
      MYSQL_ROOT_PASSWORD: ${DB_PASSWORD}
      MYSQL_DATABASE: ${DB_NAME}
    volumes:
      - mysql_data:/var/lib/mysql
      - ./schema.sql:/docker-entrypoint-initdb.d/schema.sql
    ports:
      - "3306:3306"
    healthcheck:
      test: ["CMD", "mysqladmin", "ping", "-h", "localhost"]
      interval: 10s
      timeout: 5s
      retries: 5

  backend:
    build: .
    depends_on:
      mysql:
        condition: service_healthy
    environment:
      - DB_HOST=mysql
      - DB_PORT=3306
      - DB_NAME=${DB_NAME}
      - DB_USERNAME=${DB_USERNAME}
      - DB_PASSWORD=${DB_PASSWORD}
      - HOST=0.0.0.0
      - PORT=3000
      - FIREBASE_PROJECT_ID=${FIREBASE_PROJECT_ID}
      - FIREBASE_API_KEY=${FIREBASE_API_KEY}
    volumes:
      - ./firebase-service-account.json:/app/firebase-service-account.json
    ports:
      - "3000:3000"
      - "3001:3001"
    restart: unless-stopped

volumes:
  mysql_data:
```

### 3. Deploy

```bash
# Build and start
docker-compose up -d

# View logs
docker-compose logs -f backend

# Stop
docker-compose down
```

## Cloud Deployment

### AWS ECS

1. **Build and push Docker image**:
```bash
aws ecr get-login-password --region us-east-1 | docker login --username AWS --password-stdin <account-id>.dkr.ecr.us-east-1.amazonaws.com

docker build -t lwm-backend .
docker tag lwm-backend:latest <account-id>.dkr.ecr.us-east-1.amazonaws.com/lwm-backend:latest
docker push <account-id>.dkr.ecr.us-east-1.amazonaws.com/lwm-backend:latest
```

2. **Create ECS Task Definition** with:
   - Container: lwm-backend
   - Port mappings: 3000, 3001
   - Environment variables from .env
   - Secrets: Firebase credentials from AWS Secrets Manager

3. **Create ECS Service** with:
   - Load balancer for HTTP/3 and gRPC
   - Auto-scaling based on CPU/memory
   - Health checks

### Google Cloud Run

```bash
# Build and deploy
gcloud builds submit --tag gcr.io/PROJECT_ID/lwm-backend
gcloud run deploy lwm-backend \
  --image gcr.io/PROJECT_ID/lwm-backend \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --set-env-vars DB_HOST=...,DB_NAME=... \
  --port 3000
```

### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: lwm-backend
spec:
  replicas: 3
  selector:
    matchLabels:
      app: lwm-backend
  template:
    metadata:
      labels:
        app: lwm-backend
    spec:
      containers:
      - name: backend
        image: lwm-backend:latest
        ports:
        - containerPort: 3000
        - containerPort: 3001
        env:
        - name: DB_HOST
          valueFrom:
            secretKeyRef:
              name: db-credentials
              key: host
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
---
apiVersion: v1
kind: Service
metadata:
  name: lwm-backend-service
spec:
  selector:
    app: lwm-backend
  ports:
  - name: http3
    port: 3000
    targetPort: 3000
  - name: grpc
    port: 3001
    targetPort: 3001
  type: LoadBalancer
```

## Production Checklist

### Security
- [ ] Use environment variables for all secrets
- [ ] Enable HTTPS/TLS in production
- [ ] Implement proper authentication
- [ ] Set up firewall rules
- [ ] Use secrets management (AWS Secrets Manager, etc.)
- [ ] Regular security updates

### Performance
- [ ] Enable connection pooling (already done)
- [ ] Set up caching (Redis)
- [ ] Configure rate limiting (already done)
- [ ] Optimize database queries
- [ ] Enable gzip compression

### Monitoring
- [ ] Set up logging aggregation (ELK, CloudWatch)
- [ ] Configure metrics (Prometheus)
- [ ] Set up alerts (PagerDuty, Slack)
- [ ] Health check endpoints
- [ ] Performance monitoring (New Relic, Datadog)

### Reliability
- [ ] Database backups
- [ ] Disaster recovery plan
- [ ] Load balancing
- [ ] Auto-scaling
- [ ] Circuit breakers

### CI/CD
- [ ] Automated testing
- [ ] Automated deployment
- [ ] Rollback strategy
- [ ] Blue-green deployment
- [ ] Canary releases

## Environment Variables

Production `.env`:
```env
# Database
DB_HOST=production-db.example.com
DB_PORT=3306
DB_NAME=lwm_prod
DB_USERNAME=lwm_user
DB_PASSWORD=<strong-password>

# Server
HOST=0.0.0.0
PORT=3000

# Firebase
FIREBASE_PROJECT_ID=your-prod-project
FIREBASE_API_KEY=<api-key>
FIREBASE_AUTH_DOMAIN=your-prod-project.firebaseapp.com
FIREBASE_STORAGE_BUCKET=your-prod-project.appspot.com
FIREBASE_MESSAGING_SENDER_ID=<sender-id>
FIREBASE_APP_ID=<app-id>

# Logging
RUST_LOG=info,backend=debug
```

## Monitoring

### Health Check Endpoint

Add to `src/server.rs`:
```rust
// GET /health
async fn health_check() -> &'static str {
    "OK"
}
```

### Metrics

Use Prometheus metrics:
```bash
cargo add prometheus
```

## Scaling

### Horizontal Scaling
- Run multiple instances behind a load balancer
- Use sticky sessions for WebSocket connections
- Share session state via Redis

### Vertical Scaling
- Increase CPU/memory allocation
- Optimize database queries
- Use connection pooling

## Backup Strategy

### Database Backups
```bash
# Daily backup
0 2 * * * mysqldump -u root -p rotiride > /backups/rotiride_$(date +\%Y\%m\%d).sql

# Weekly full backup
0 3 * * 0 mysqldump -u root -p --all-databases > /backups/full_$(date +\%Y\%m\%d).sql
```

### Application State
- Store uploaded files in S3/Cloud Storage
- Keep Firebase credentials in secrets manager
- Version control all configuration
