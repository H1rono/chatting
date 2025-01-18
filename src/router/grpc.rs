use std::sync::Arc;

use axum::body::Body as AxumBody;
use tonic::{server::NamedService, Status};
use tower::Service;

use crate::user::ProvideUserService;

mod id {
    tonic::include_proto!("chatting.id");
}

mod user {
    tonic::include_proto!("chatting.user");

    pub use user_service_server::{UserService, UserServiceServer, SERVICE_NAME};
}

// MARK: type conversions

fn error_into_status<E: std::error::Error + 'static>(e: E) -> Status {
    tracing::error!(error = &e as &dyn std::error::Error);
    Status::internal(e.to_string())
}

fn convert_timestamp(t: crate::user::Timestamp) -> Result<prost_types::Timestamp, tonic::Status> {
    let seconds = t.timestamp();
    let nanos = t.timestamp_subsec_nanos() as i32;
    let t = prost_types::Timestamp { seconds, nanos };
    Ok(t)
}

impl TryFrom<crate::user::User> for user::User {
    type Error = Status;
    fn try_from(value: crate::user::User) -> Result<Self, Self::Error> {
        let crate::user::User {
            id: crate::user::UserId(id),
            name: crate::user::UserName(name),
            created_at,
            updated_at,
        } = value;
        let id = id::UserId { id: id.to_string() };
        let res = user::User {
            id: Some(id),
            name,
            created_at: Some(convert_timestamp(created_at)?),
            updated_at: Some(convert_timestamp(updated_at)?),
        };
        Ok(res)
    }
}

impl TryFrom<user::GetUserRequest> for crate::user::GetUser {
    type Error = Status;

    fn try_from(value: user::GetUserRequest) -> Result<Self, Self::Error> {
        let user::GetUserRequest { id } = value;
        let Some(id) = id else {
            let status = Status::invalid_argument("unspecified user id");
            return Err(status);
        };
        let id = id
            .id
            .parse::<uuid::Uuid>()
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let slf = crate::user::GetUser {
            id: crate::user::UserId(id),
        };
        Ok(slf)
    }
}

impl TryFrom<user::CreateUserRequest> for crate::user::CreateUser {
    type Error = Status;

    fn try_from(value: user::CreateUserRequest) -> Result<Self, Self::Error> {
        let name = crate::user::UserName(value.name);
        let slf = crate::user::CreateUser { name };
        Ok(slf)
    }
}

impl TryFrom<user::UpdateUserRequest> for crate::user::UpdateUser {
    type Error = Status;

    fn try_from(value: user::UpdateUserRequest) -> Result<Self, Self::Error> {
        let user::UpdateUserRequest { id, name } = value;
        let Some(id) = id else {
            let status = Status::invalid_argument("unspecified user id");
            return Err(status);
        };
        let id = id
            .id
            .parse::<uuid::Uuid>()
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let slf = crate::user::UpdateUser {
            id: crate::user::UserId(id),
            name: crate::user::UserName(name),
        };
        Ok(slf)
    }
}

impl TryFrom<user::DeleteUserRequest> for crate::user::DeleteUser {
    type Error = Status;

    fn try_from(value: user::DeleteUserRequest) -> Result<Self, Self::Error> {
        let Some(id) = value.id else {
            let status = Status::invalid_argument("unspecified user id");
            return Err(status);
        };
        let id = id
            .id
            .parse::<uuid::Uuid>()
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let slf = crate::user::DeleteUser {
            id: crate::user::UserId(id),
        };
        Ok(slf)
    }
}

// MARK: user service

#[derive(Debug)]
struct UserService<State> {
    state: Arc<State>,
}

impl<State> Clone for UserService<State> {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

#[async_trait::async_trait]
impl<State> user::UserService for UserService<State>
where
    State: ProvideUserService<Context = State>,
{
    async fn get_user(
        &self,
        request: tonic::Request<user::GetUserRequest>,
    ) -> tonic::Result<tonic::Response<user::GetUserResponse>> {
        let (_, _, request) = request.into_parts();
        let request = request.try_into()?;
        let user = self
            .state
            .get_user(request)
            .await
            .map_err(error_into_status)?;
        let Some(user) = user else {
            tracing::info!("No user found");
            return Err(Status::not_found(""));
        };
        let res = user::GetUserResponse {
            user: Some(user.try_into()?),
        };
        Ok(tonic::Response::new(res))
    }

    async fn create_user(
        &self,
        request: tonic::Request<user::CreateUserRequest>,
    ) -> tonic::Result<tonic::Response<user::CreateUserResponse>> {
        let (_, _, request) = request.into_parts();
        let request = request.try_into()?;
        let user = self
            .state
            .create_user(request)
            .await
            .map_err(error_into_status)?;
        let res = user::CreateUserResponse {
            user: Some(user.try_into()?),
        };
        Ok(tonic::Response::new(res))
    }

    async fn update_user(
        &self,
        request: tonic::Request<user::UpdateUserRequest>,
    ) -> tonic::Result<tonic::Response<user::UpdateUserResponse>> {
        let (_, _, request) = request.into_parts();
        let request = request.try_into()?;
        let user = self
            .state
            .update_user(request)
            .await
            .map_err(error_into_status)?;
        let Some(user) = user else {
            tracing::info!("No user found");
            return Err(Status::not_found(""));
        };
        let res = user::UpdateUserResponse {
            user: Some(user.try_into()?),
        };
        Ok(tonic::Response::new(res))
    }

    async fn delete_user(
        &self,
        request: tonic::Request<user::DeleteUserRequest>,
    ) -> tonic::Result<tonic::Response<user::DeleteUserResponse>> {
        let (_, _, request) = request.into_parts();
        let request = request.try_into()?;
        let user = self
            .state
            .delete_user(request)
            .await
            .map_err(error_into_status)?;
        let Some(user) = user else {
            tracing::info!("No user found");
            return Err(Status::not_found(""));
        };
        let res = user::DeleteUserResponse {
            user: Some(user.try_into()?),
        };
        Ok(tonic::Response::new(res))
    }
}

pub fn user_service<State: ProvideUserService<Context = State>>(
    state: Arc<State>,
) -> impl Service<
    http::Request<AxumBody>,
    Response = http::Response<AxumBody>,
    Error = std::convert::Infallible,
> + NamedService
       + Clone
       + Send
       + 'static {
    use std::task::{Context, Poll};
    use tower::ServiceExt;

    #[derive(Clone)]
    struct NamedUserService<S>(S);
    impl<S, B> Service<http::Request<B>> for NamedUserService<S>
    where
        S: Service<http::Request<B>>,
    {
        type Response = S::Response;
        type Error = S::Error;
        type Future = S::Future;
        fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.0.poll_ready(cx)
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            self.0.call(req)
        }
    }
    impl<S> NamedService for NamedUserService<S> {
        const NAME: &'static str = user::SERVICE_NAME;
    }

    let service = UserService { state };
    let service = tower::ServiceBuilder::new()
        .layer(tower_http::trace::TraceLayer::new_for_grpc())
        .service(user::UserServiceServer::new(service))
        .map_request(|r| r) // workaround to pass `map_response`
        .map_response(|r| r.map(AxumBody::new));
    NamedUserService(service)
}
