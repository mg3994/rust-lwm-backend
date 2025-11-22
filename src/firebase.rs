use serde::{Deserialize, Serialize};
use serde_json::json;
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};
use chrono::{Utc, Duration};
use std::fs;

#[derive(Debug, Deserialize)]
struct ServiceAccount {
    project_id: String,
    private_key: String,
    client_email: String,
}

#[derive(Debug, Serialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: usize,
    iat: usize,
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    // expires_in: i32,
    // token_type: String,
}

pub struct FirebaseClient {
    client: reqwest::Client,
    service_account: ServiceAccount,
}

#[derive(Debug)]
pub enum NotificationType {
    Standard,
    Link { url: String },
    Image { url: String },
    Chat { sender_id: String, chat_id: String },
    Call { caller_id: String, call_id: String, is_video: bool },
}

impl FirebaseClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        println!("Initializing Firebase Client...");
        let content = fs::read_to_string("firebase-service-account.json")
            .map_err(|e| format!("Failed to read service account file: {}", e))?;
        
        let service_account: ServiceAccount = serde_json::from_str(&content)
            .map_err(|e| format!("Failed to parse service account json: {}", e))?;

        Ok(Self {
            client: reqwest::Client::new(),
            service_account,
        })
    }

    async fn get_access_token(&self) -> Result<String, Box<dyn std::error::Error>> {
        let now = Utc::now();
        let exp = now + Duration::hours(1);
        
        let claims = Claims {
            iss: self.service_account.client_email.clone(),
            scope: "https://www.googleapis.com/auth/firebase.messaging".to_string(),
            aud: "https://oauth2.googleapis.com/token".to_string(),
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        let header = Header::new(Algorithm::RS256);
        // The private key in the JSON file usually has \n which needs to be handled if not already
        // But standard PEM parsers often handle it. jsonwebtoken's EncodingKey::from_rsa_pem expects correct PEM.
        let key = EncodingKey::from_rsa_pem(self.service_account.private_key.as_bytes())
            .map_err(|e| format!("Failed to process private key: {}", e))?;

        let jwt = encode(&header, &claims, &key)
            .map_err(|e| format!("Failed to encode JWT: {}", e))?;

        let params = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ];

        let res = self.client.post("https://oauth2.googleapis.com/token")
            .form(&params)
            .send()
            .await
            .map_err(|e| format!("Failed to send token request: {}", e))?;

        if !res.status().is_success() {
            let text = res.text().await?;
            return Err(format!("Token exchange failed: {}", text).into());
        }

        let token_res: TokenResponse = res.json().await
            .map_err(|e| format!("Failed to parse token response: {}", e))?;

        Ok(token_res.access_token)
    }

    pub async fn verify_token(&self, token: &str) -> bool {
        println!("Verifying token: {}", token);
        // Full verification requires fetching Google's public keys and validating the signature.
        // For now, we are focusing on the sending part as requested.
        true
    }

    pub async fn send_notification(&self, token: &str, title: &str, body: &str, notification_type: NotificationType) -> Result<(), Box<dyn std::error::Error>> {
        println!("Sending notification to {}: {} - {} ({:?})", token, title, body, notification_type);
        
        let access_token = self.get_access_token().await?;

        let url = format!("https://fcm.googleapis.com/v1/projects/{}/messages:send", self.service_account.project_id);

        let mut data = serde_json::Map::new();
        
        match notification_type {
            NotificationType::Standard => {},
            NotificationType::Link { url } => {
                data.insert("type".to_string(), json!("link"));
                data.insert("url".to_string(), json!(url));
            },
            NotificationType::Image { url } => {
                data.insert("type".to_string(), json!("image"));
                data.insert("image_url".to_string(), json!(url));
            },
            NotificationType::Chat { sender_id, chat_id } => {
                data.insert("type".to_string(), json!("chat"));
                data.insert("sender_id".to_string(), json!(sender_id));
                data.insert("chat_id".to_string(), json!(chat_id));
            },
            NotificationType::Call { caller_id, call_id, is_video } => {
                data.insert("type".to_string(), json!("call"));
                data.insert("caller_id".to_string(), json!(caller_id));
                data.insert("call_id".to_string(), json!(call_id));
                data.insert("is_video".to_string(), json!(is_video.to_string())); // FCM data values must be strings
            },
        }

        let mut message = json!({
            "token": token,
            "notification": {
                "title": title,
                "body": body
            }
        });

        if !data.is_empty() {
            message["data"] = json!(data);
        }

        let payload = json!({ "message": message });

        let res = self.client.post(&url)
            .bearer_auth(access_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| format!("Failed to send FCM request: {}", e))?;

        println!("FCM Response Status: {}", res.status());
        
        if !res.status().is_success() {
             let text = res.text().await?;
             println!("FCM Error Body: {}", text);
             return Err(format!("FCM request failed: {}", text).into());
        }

        Ok(())
    }
}
