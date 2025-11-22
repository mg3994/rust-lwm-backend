use tonic::{Request, Status};
use crate::firebase::FirebaseClient;

/// Middleware for Firebase token authentication
pub struct AuthMiddleware {
    firebase: FirebaseClient,
}

impl AuthMiddleware {
    pub fn new(firebase: FirebaseClient) -> Self {
        Self { firebase }
    }

    /// Verify Firebase ID token from request metadata
    pub async fn verify_token(&self, token: &str) -> Result<String, Status> {
        if self.firebase.verify_token(token).await {
            // In production, decode the token to get the user ID
            // For now, return a placeholder
            Ok("verified_user_id".to_string())
        } else {
            Err(Status::unauthenticated("Invalid token"))
        }
    }

    /// Extract token from gRPC request metadata
    pub fn extract_token<T>(request: &Request<T>) -> Result<String, Status> {
        let metadata = request.metadata();
        
        metadata
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .map(|s| s.to_string())
            .ok_or_else(|| Status::unauthenticated("Missing authorization token"))
    }
}

/// Interceptor for gRPC authentication
pub fn auth_interceptor(
    mut req: Request<()>,
) -> Result<Request<()>, Status> {
    // Extract and validate token
    let token = req
        .metadata()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or_else(|| Status::unauthenticated("Missing authorization header"))?;

    if !token.starts_with("Bearer ") {
        return Err(Status::unauthenticated("Invalid authorization format"));
    }

    // Token validation would happen here in production
    // For now, just check if it exists
    println!("Auth interceptor: Token present");

    Ok(req)
}

/// Helper to verify user has required role
pub fn check_role(user_role: &str, required_role: &str) -> Result<(), Status> {
    match (user_role, required_role) {
        ("admin", _) => Ok(()), // Admin can do anything
        (role, req) if role == req => Ok(()),
        _ => Err(Status::permission_denied("Insufficient permissions")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_role() {
        assert!(check_role("admin", "user").is_ok());
        assert!(check_role("user", "user").is_ok());
        assert!(check_role("user", "admin").is_err());
    }
}
