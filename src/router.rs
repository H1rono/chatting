use crate::error::Failure;

mod user;

struct ErrorStatus(Failure);

impl From<Failure> for ErrorStatus {
    fn from(value: Failure) -> Self {
        Self(value)
    }
}

impl From<crate::error::Reject> for ErrorStatus {
    fn from(value: crate::error::Reject) -> Self {
        Self(value.into())
    }
}

impl From<anyhow::Error> for ErrorStatus {
    fn from(value: anyhow::Error) -> Self {
        Self(value.into())
    }
}

impl From<ErrorStatus> for tonic::Status {
    fn from(value: ErrorStatus) -> Self {
        use crate::error::RejectKind;
        use Failure::{Error, Reject};

        fn encode_reject_kind(kind: RejectKind) -> tonic::Code {
            match kind {
                RejectKind::BadRequest => tonic::Code::InvalidArgument,
                RejectKind::Unauthenticated => tonic::Code::Unauthenticated,
                RejectKind::NotFound => tonic::Code::NotFound,
            }
        }

        match value.0 {
            Reject(r) => {
                tracing::info!(reject = %r);
                let code = encode_reject_kind(r.kind());
                let message = r.into_message();
                tonic::Status::new(code, message)
            }
            Error(e) => {
                tracing::error!(error = ?e);
                tonic::Status::internal(format!("{e}"))
            }
        }
    }
}

pub fn make_router<State>(state: State) -> axum::Router
where
    State: crate::user::ProvideUserService + Clone,
{
    use tower_http::ServiceBuilderExt;

    let user = user::Service::new(state);
    let layer = tower::ServiceBuilder::new().trace_for_grpc();
    axum::Router::new()
        .route_service(
            &format!("/{}/{{*rest}}", user::SERVICE_NAME),
            user::Server::new(user),
        )
        .layer(layer)
}
