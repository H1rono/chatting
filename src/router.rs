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

    let user = crate::user::ProvideUserService::build_tower_service(state);
    let user = tower::ServiceBuilder::new()
        .layer(tower_http::trace::TraceLayer::new_for_grpc())
        .service(user)
        .map_request(|r: http::Request<_>| r)
        .map_response(|r| r.map(axum::body::Body::new));
    user.boxed_clone()
}
