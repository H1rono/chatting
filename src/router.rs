use std::sync::Arc;

use axum::body::Body as AxumBody;
use tower::Service;

mod grpc;

pub fn make_router<State>(
    state: Arc<State>,
) -> impl Service<
    http::Request<AxumBody>,
    Response = http::Response<AxumBody>,
    Error = std::convert::Infallible,
>
where
    State: crate::user::ProvideUserService<Context = State>,
{
    grpc::user_service(state)
}
