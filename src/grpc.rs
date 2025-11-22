use tonic::{transport::Server, Request, Response, Status};
use std::sync::Arc;
use crate::AppState;

pub mod pb {
    tonic::include_proto!("service");
}

use pb::link_with_mentor_server::{LinkWithMentor, LinkWithMentorServer};
use pb::*;

#[derive(Debug)]
pub struct MyLinkWithMentor {
    state: Arc<AppState>,
}

#[tonic::async_trait]
impl LinkWithMentor for MyLinkWithMentor {
    async fn ping(
        &self,
        request: Request<PingRequest>,
    ) -> Result<Response<PingResponse>, Status> {
        self.state.metrics.increment_requests();
        
        tracing::debug!("Got a ping request: {:?}", request);

        let reply = PingResponse {
            message: format!("Pong: {}", request.into_inner().message),
        };

        self.state.metrics.increment_successful();
        Ok(Response::new(reply))
    }

    async fn health_check(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        let health = crate::health::check_health(
            self.state.clone(),
            self.state.start_time
        ).await;

        Ok(Response::new(HealthResponse {
            status: health.status,
            database: health.database,
            firebase: health.firebase,
            uptime_seconds: health.uptime_seconds,
        }))
    }

    async fn get_metrics(
        &self,
        _request: Request<EmptyRequest>,
    ) -> Result<Response<MetricsResponse>, Status> {
        let snapshot = self.state.metrics.get_snapshot();

        Ok(Response::new(MetricsResponse {
            total_requests: snapshot.total_requests,
            successful_requests: snapshot.successful_requests,
            failed_requests: snapshot.failed_requests,
            success_rate: snapshot.success_rate(),
            total_users_created: snapshot.total_users_created,
            total_sessions_created: snapshot.total_sessions_created,
            total_notifications_sent: snapshot.total_notifications_sent,
        }))
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        self.state.metrics.increment_requests();
        
        let req = request.into_inner();
        
        // Rate limiting check
        if !self.state.rate_limiter.check_rate_limit(&req.firebase_uid) {
            tracing::warn!("Rate limit exceeded for user: {}", req.firebase_uid);
            self.state.metrics.increment_failed();
            return Err(Status::resource_exhausted("Rate limit exceeded. Please try again later."));
        }
        
        tracing::info!("Creating user: {}", req.email);
        
        let user = crate::models::CreateUser {
            firebase_uid: req.firebase_uid,
            email: req.email,
            display_name: req.display_name,
            photo_url: req.photo_url,
            role: req.role,
        };

        let user_id = crate::db::create_user(&self.state.db, &user)
            .await
            .map_err(|e| {
                tracing::error!("Failed to create user: {}", e);
                self.state.metrics.increment_failed();
                Status::internal(format!("Failed to create user: {}", e))
            })?;

        let created_user = crate::db::get_user_by_id(&self.state.db, user_id)
            .await
            .map_err(|e| {
                self.state.metrics.increment_failed();
                Status::internal(format!("Failed to get user: {}", e))
            })?
            .ok_or_else(|| {
                self.state.metrics.increment_failed();
                Status::not_found("User not found after creation")
            })?;

        tracing::info!("User created successfully: ID {}", created_user.id);
        self.state.metrics.increment_successful();
        self.state.metrics.increment_users_created();

        Ok(Response::new(UserResponse {
            id: created_user.id,
            firebase_uid: created_user.firebase_uid,
            email: created_user.email,
            display_name: created_user.display_name,
            photo_url: created_user.photo_url,
            role: created_user.role,
            created_at: created_user.created_at.to_string(),
        }))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<UserResponse>, Status> {
        let req = request.into_inner();

        let user = match req.identifier {
            Some(get_user_request::Identifier::UserId(id)) => {
                crate::db::get_user_by_id(&self.state.db, id).await
            }
            Some(get_user_request::Identifier::FirebaseUid(uid)) => {
                crate::db::get_user_by_firebase_uid(&self.state.db, &uid).await
            }
            None => return Err(Status::invalid_argument("No identifier provided")),
        }
        .map_err(|e| Status::internal(format!("Database error: {}", e)))?
        .ok_or_else(|| Status::not_found("User not found"))?;

        Ok(Response::new(UserResponse {
            id: user.id,
            firebase_uid: user.firebase_uid,
            email: user.email,
            display_name: user.display_name,
            photo_url: user.photo_url,
            role: user.role,
            created_at: user.created_at.to_string(),
        }))
    }

    async fn create_session(
        &self,
        request: Request<CreateSessionRequest>,
    ) -> Result<Response<SessionResponse>, Status> {
        let req = request.into_inner();

        let scheduled_at = chrono::NaiveDateTime::parse_from_str(&req.scheduled_at, "%Y-%m-%d %H:%M:%S")
            .map_err(|e| Status::invalid_argument(format!("Invalid datetime format: {}", e)))?;

        let session = crate::models::CreateSession {
            user_id: req.user_id,
            mentor_id: req.mentor_id,
            title: req.title,
            description: req.description,
            scheduled_at,
            duration_minutes: req.duration_minutes,
            meeting_link: req.meeting_link,
        };

        let session_id = crate::db::create_session(&self.state.db, &session)
            .await
            .map_err(|e| Status::internal(format!("Failed to create session: {}", e)))?;

        Ok(Response::new(SessionResponse {
            id: session_id,
            user_id: session.user_id,
            mentor_id: session.mentor_id,
            title: session.title,
            description: session.description,
            scheduled_at: session.scheduled_at.to_string(),
            duration_minutes: session.duration_minutes.unwrap_or(60),
            status: "scheduled".to_string(),
            meeting_link: session.meeting_link,
        }))
    }

    async fn get_user_sessions(
        &self,
        request: Request<GetUserSessionsRequest>,
    ) -> Result<Response<SessionListResponse>, Status> {
        let user_id = request.into_inner().user_id;

        let sessions = crate::db::get_sessions_by_user(&self.state.db, user_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to get sessions: {}", e)))?;

        let session_responses: Vec<SessionResponse> = sessions
            .into_iter()
            .map(|s| SessionResponse {
                id: s.id,
                user_id: s.user_id,
                mentor_id: s.mentor_id,
                title: s.title,
                description: s.description,
                scheduled_at: s.scheduled_at.to_string(),
                duration_minutes: s.duration_minutes,
                status: s.status,
                meeting_link: s.meeting_link,
            })
            .collect();

        Ok(Response::new(SessionListResponse {
            sessions: session_responses,
        }))
    }

    async fn send_notification(
        &self,
        request: Request<SendNotificationRequest>,
    ) -> Result<Response<NotificationResponse>, Status> {
        let req = request.into_inner();

        let notification = crate::models::CreateNotification {
            user_id: req.user_id,
            title: req.title.clone(),
            body: req.body.clone(),
            notification_type: req.notification_type.clone(),
            data: req.data.clone(),
        };

        let notification_id = crate::db::create_notification(&self.state.db, &notification)
            .await
            .map_err(|e| Status::internal(format!("Failed to create notification: {}", e)))?;

        // Get user's device tokens and send via FCM
        let tokens = crate::db::get_user_device_tokens(&self.state.db, req.user_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to get device tokens: {}", e)))?;

        // Send to each device token
        for token in tokens {
            let notification_data = crate::firebase::NotificationData {
                link: None,
                image: None,
                chat_id: None,
                sender_id: None,
                caller_id: None,
                call_id: None,
                is_video: None,
            };

            if let Err(e) = self.state.firebase
                .send_notification(&token, &req.title, &req.body, notification_data)
                .await
            {
                eprintln!("Failed to send FCM notification: {}", e);
            }
        }

        Ok(Response::new(NotificationResponse {
            id: notification_id,
            user_id: req.user_id,
            title: req.title,
            body: req.body,
            notification_type: req.notification_type,
            data: req.data,
            is_read: false,
            created_at: chrono::Utc::now().naive_utc().to_string(),
        }))
    }

    async fn get_unread_notifications(
        &self,
        request: Request<GetUnreadNotificationsRequest>,
    ) -> Result<Response<NotificationListResponse>, Status> {
        let user_id = request.into_inner().user_id;

        let notifications = crate::db::get_unread_notifications(&self.state.db, user_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to get notifications: {}", e)))?;

        let notification_responses: Vec<NotificationResponse> = notifications
            .into_iter()
            .map(|n| NotificationResponse {
                id: n.id,
                user_id: n.user_id,
                title: n.title,
                body: n.body,
                notification_type: n.notification_type,
                data: n.data,
                is_read: n.is_read,
                created_at: n.created_at.to_string(),
            })
            .collect();

        Ok(Response::new(NotificationListResponse {
            notifications: notification_responses,
        }))
    }

    async fn mark_notification_read(
        &self,
        request: Request<MarkNotificationReadRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let notification_id = request.into_inner().notification_id;

        crate::db::mark_notification_read(&self.state.db, notification_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to mark notification as read: {}", e)))?;

        Ok(Response::new(EmptyResponse {}))
    }

    async fn register_device_token(
        &self,
        request: Request<RegisterDeviceTokenRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let req = request.into_inner();

        let token = crate::models::CreateDeviceToken {
            user_id: req.user_id,
            token: req.token,
            device_type: req.device_type,
        };

        crate::db::upsert_device_token(&self.state.db, &token)
            .await
            .map_err(|e| Status::internal(format!("Failed to register device token: {}", e)))?;

        Ok(Response::new(EmptyResponse {}))
    }
}

pub async fn run(host: &str, port: u16, state: Arc<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", host, port).parse()?;
    let service = MyLinkWithMentor { state };

    println!("gRPC server listening on {}", addr);

    Server::builder()
        .add_service(LinkWithMentorServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
