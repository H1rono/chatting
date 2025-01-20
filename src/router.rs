use std::sync::Arc;

use axum::body::Body as AxumBody;

pub fn make_router<State>(
    state: Arc<State>,
) -> tower::util::BoxCloneService<
    http::Request<AxumBody>,
    http::Response<AxumBody>,
    std::convert::Infallible,
>
where
    State: crate::user::ProvideUserService<Context = State>,
    <State::UserService as crate::user::UserService<State>>::Error: crate::prelude::Error,
{
    use tower::ServiceExt;

    state.build_tower_service().boxed_clone()
}
