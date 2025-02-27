use std::fmt;

#[must_use]
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RejectKind {
    Unauthenticated,
    BadRequest,
    NotFound,
}

impl fmt::Display for RejectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Unauthenticated => "Unauthenticated",
            Self::BadRequest => "Bad request",
            Self::NotFound => "Not found",
        };
        f.write_str(s)
    }
}

impl std::error::Error for Reject {}

#[derive(Debug, Clone)]
pub struct Reject {
    kind: RejectKind,
    message: String,
}

impl fmt::Display for Reject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl Reject {
    pub fn new(kind: RejectKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn unauthenticated(message: impl Into<String>) -> Self {
        Self::new(RejectKind::Unauthenticated, message)
    }

    pub fn bad_request(message: impl Into<String>) -> Self {
        Self::new(RejectKind::BadRequest, message)
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::new(RejectKind::NotFound, message)
    }

    pub fn kind(&self) -> RejectKind {
        self.kind
    }

    pub fn as_message(&self) -> &str {
        &self.message
    }

    pub fn into_message(self) -> String {
        self.message
    }
}

pub enum Failure {
    Reject(Reject),
    Error(anyhow::Error),
}

impl fmt::Debug for Failure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Reject(r) => fmt::Debug::fmt(r, f),
            Self::Error(e) => fmt::Debug::fmt(e, f),
        }
    }
}

impl From<Reject> for Failure {
    fn from(value: Reject) -> Self {
        Self::Reject(value)
    }
}

impl From<anyhow::Error> for Failure {
    fn from(value: anyhow::Error) -> Self {
        Self::Error(value)
    }
}

impl Failure {
    pub fn reject(kind: RejectKind, message: impl Into<String>) -> Self {
        Reject::new(kind, message).into()
    }

    pub fn reject_unauthenticated(message: impl Into<String>) -> Self {
        Reject::unauthenticated(message).into()
    }

    pub fn reject_bad_request(message: impl Into<String>) -> Self {
        Reject::bad_request(message).into()
    }

    pub fn reject_not_found(message: impl Into<String>) -> Self {
        Reject::not_found(message).into()
    }
}
