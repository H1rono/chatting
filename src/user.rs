use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

pub mod error;
pub mod svc;

pub type Timestamp = chrono::DateTime<chrono::Utc>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UserId(pub uuid::Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(transparent)]
pub struct UserName(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct GetUser {
    pub id: UserId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CreateUser {
    pub name: UserName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct UpdateUser {
    pub id: UserId,
    pub name: UserName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct DeleteUser {
    pub id: UserId,
}

pub trait UserService<Context>: Send + Sync + 'static {
    type Error: std::error::Error + Send + Sync + 'static;

    fn get_user<'a>(
        &'a self,
        ctx: &'a Context,
        request: GetUser,
    ) -> BoxFuture<'a, Result<Option<User>, Self::Error>>;
    fn create_user<'a>(
        &'a self,
        ctx: &'a Context,
        request: CreateUser,
    ) -> BoxFuture<'a, Result<User, Self::Error>>;
    fn update_user<'a>(
        &'a self,
        ctx: &'a Context,
        request: UpdateUser,
    ) -> BoxFuture<'a, Result<Option<User>, Self::Error>>;
    fn delete_user<'a>(
        &'a self,
        ctx: &'a Context,
        request: DeleteUser,
    ) -> BoxFuture<'a, Result<Option<User>, Self::Error>>;
}
