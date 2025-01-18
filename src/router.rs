use std::sync::Arc;

use axum::body::Body as AxumBody;

mod grpc;

pub fn make_router<State>(
    state: Arc<State>,
) -> tower::util::BoxCloneService<
    http::Request<AxumBody>,
    http::Response<AxumBody>,
    std::convert::Infallible,
>
where
    State: crate::user::ProvideUserService<Context = State>,
{
    use tower::ServiceExt;

    grpc::user_service(state).boxed_clone()
}
