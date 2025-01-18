use std::sync::Arc;

use axum::body::Body as AxumBody;
use tonic::server::NamedService;
use tower::Service;

use crate::user::ProvideUserService;

tonic::include_proto!("chatting.id");
tonic::include_proto!("chatting.user");

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
impl<State> user_service_server::UserService for UserService<State>
where
    State: ProvideUserService<State>,
{
    async fn get_user(
        &self,
        request: tonic::Request<GetUserRequest>,
    ) -> tonic::Result<tonic::Response<GetUserResponse>> {
        todo!()
    }

    async fn create_user(
        &self,
        request: tonic::Request<CreateUserRequest>,
    ) -> tonic::Result<tonic::Response<CreateUserResponse>> {
        todo!()
    }

    async fn update_user(
        &self,
        request: tonic::Request<UpdateUserRequest>,
    ) -> tonic::Result<tonic::Response<UpdateUserResponse>> {
        todo!()
    }

    async fn delete_user(
        &self,
        request: tonic::Request<DeleteUserRequest>,
    ) -> tonic::Result<tonic::Response<DeleteUserResponse>> {
        todo!()
    }
}

pub fn user_service<State: ProvideUserService<State>>(
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
        const NAME: &'static str = user_service_server::SERVICE_NAME;
    }

    let service = UserService { state };
    let service = tower::ServiceBuilder::new()
        .layer(tower_http::trace::TraceLayer::new_for_grpc())
        .service(user_service_server::UserServiceServer::new(service))
        .map_request(|r| r) // workaround to pass `map_response`
        .map_response(|r| r.map(AxumBody::new));
    NamedUserService(service)
}
