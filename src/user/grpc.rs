use std::sync::Arc;

use axum::body::Body as AxumBody;
use tonic::{server::NamedService, Status};
use tower::Service;

use crate::grpc::user as generated;

pub use generated::user_service_server::SERVICE_NAME;

// MARK: type conversions

fn error_into_status<E: std::error::Error + 'static>(e: E) -> Status {
    tracing::error!(error = &e as &dyn std::error::Error);
    Status::internal(e.to_string())
}

fn convert_timestamp(t: super::Timestamp) -> Result<prost_types::Timestamp, tonic::Status> {
    let seconds = t.timestamp();
    let nanos = t.timestamp_subsec_nanos() as i32;
    let t = prost_types::Timestamp { seconds, nanos };
    Ok(t)
}

impl TryFrom<super::User> for generated::User {
    type Error = Status;
    fn try_from(value: super::User) -> Result<Self, Self::Error> {
        let super::User {
            id: super::UserId(id),
            name: super::UserName(name),
            created_at,
            updated_at,
        } = value;
        let id = crate::grpc::id::UserId { id: id.to_string() };
        let res = generated::User {
            id: Some(id),
            name,
            created_at: Some(convert_timestamp(created_at)?),
            updated_at: Some(convert_timestamp(updated_at)?),
        };
        Ok(res)
    }
}

impl TryFrom<generated::GetUserRequest> for super::GetUser {
    type Error = Status;

    fn try_from(value: generated::GetUserRequest) -> Result<Self, Self::Error> {
        let generated::GetUserRequest { id } = value;
        let Some(id) = id else {
            let status = Status::invalid_argument("unspecified user id");
            return Err(status);
        };
        let id = id
            .id
            .parse::<uuid::Uuid>()
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let slf = super::GetUser {
            id: super::UserId(id),
        };
        Ok(slf)
    }
}

impl TryFrom<generated::CreateUserRequest> for super::CreateUser {
    type Error = Status;

    fn try_from(value: generated::CreateUserRequest) -> Result<Self, Self::Error> {
        let name = super::UserName(value.name);
        let slf = super::CreateUser { name };
        Ok(slf)
    }
}

impl TryFrom<generated::UpdateUserRequest> for super::UpdateUser {
    type Error = Status;

    fn try_from(value: generated::UpdateUserRequest) -> Result<Self, Self::Error> {
        let generated::UpdateUserRequest { id, name } = value;
        let Some(id) = id else {
            let status = Status::invalid_argument("unspecified user id");
            return Err(status);
        };
        let id = id
            .id
            .parse::<uuid::Uuid>()
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let slf = super::UpdateUser {
            id: super::UserId(id),
            name: super::UserName(name),
        };
        Ok(slf)
    }
}

impl TryFrom<generated::DeleteUserRequest> for super::DeleteUser {
    type Error = Status;

    fn try_from(value: generated::DeleteUserRequest) -> Result<Self, Self::Error> {
        let Some(id) = value.id else {
            let status = Status::invalid_argument("unspecified user id");
            return Err(status);
        };
        let id = id
            .id
            .parse::<uuid::Uuid>()
            .map_err(|e| Status::invalid_argument(e.to_string()))?;
        let slf = super::DeleteUser {
            id: super::UserId(id),
        };
        Ok(slf)
    }
}

// MARK: user service

#[derive(Debug)]
struct ServiceImpl<State> {
    state: Arc<State>,
}

impl<State> Clone for ServiceImpl<State> {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

#[async_trait::async_trait]
impl<State> generated::user_service_server::UserService for ServiceImpl<State>
where
    State: super::ProvideUserService<Context = State>,
{
    async fn get_user(
        &self,
        request: tonic::Request<generated::GetUserRequest>,
    ) -> tonic::Result<tonic::Response<generated::GetUserResponse>> {
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
        let res = generated::GetUserResponse {
            user: Some(user.try_into()?),
        };
        Ok(tonic::Response::new(res))
    }

    async fn create_user(
        &self,
        request: tonic::Request<generated::CreateUserRequest>,
    ) -> tonic::Result<tonic::Response<generated::CreateUserResponse>> {
        let (_, _, request) = request.into_parts();
        let request = request.try_into()?;
        let user = self
            .state
            .create_user(request)
            .await
            .map_err(error_into_status)?;
        let res = generated::CreateUserResponse {
            user: Some(user.try_into()?),
        };
        Ok(tonic::Response::new(res))
    }

    async fn update_user(
        &self,
        request: tonic::Request<generated::UpdateUserRequest>,
    ) -> tonic::Result<tonic::Response<generated::UpdateUserResponse>> {
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
        let res = generated::UpdateUserResponse {
            user: Some(user.try_into()?),
        };
        Ok(tonic::Response::new(res))
    }

    async fn delete_user(
        &self,
        request: tonic::Request<generated::DeleteUserRequest>,
    ) -> tonic::Result<tonic::Response<generated::DeleteUserResponse>> {
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
        let res = generated::DeleteUserResponse {
            user: Some(user.try_into()?),
        };
        Ok(tonic::Response::new(res))
    }
}

pub fn user_service<State: super::ProvideUserService<Context = State>>(
    state: Arc<State>,
) -> impl Service<
    http::Request<AxumBody>,
    Response = http::Response<AxumBody>,
    Error = std::convert::Infallible,
    Future: Send,
> + NamedService
       + Clone
       + Send
       + 'static {
    use generated::user_service_server::{UserServiceServer, SERVICE_NAME};
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
        const NAME: &'static str = SERVICE_NAME;
    }

    let service = ServiceImpl { state };
    let service = tower::ServiceBuilder::new()
        .layer(tower_http::trace::TraceLayer::new_for_grpc())
        .service(UserServiceServer::new(service))
        .map_request(|r| r) // workaround to pass `map_response`
        .map_response(|r| r.map(AxumBody::new));
    NamedUserService(service)
}
