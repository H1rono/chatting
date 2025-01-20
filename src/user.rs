use std::sync::Arc;

use futures::future::BoxFuture;
use serde::{Deserialize, Serialize};

use crate::prelude::Timestamp;

pub mod error;
pub mod grpc;
mod svc;

pub use error::{Error, Result};
pub use svc::Impl as UserServiceImpl;

pub type Server<S> = schema::user::user_service_server::UserServiceServer<grpc::ServiceImpl<S>>;

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

pub trait UserService<Context: ?Sized>: Send + Sync + 'static {
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

#[expect(clippy::type_complexity)]
pub trait ProvideUserService: Send + Sync + 'static {
    type Context: ?Sized;
    type UserService: UserService<Self::Context>;

    fn user_service(&self) -> &Self::UserService;
    fn context(&self) -> &Self::Context;

    fn get_user(
        &self,
        request: GetUser,
    ) -> BoxFuture<'_, Result<Option<User>, <Self::UserService as UserService<Self::Context>>::Error>>
    {
        let ctx = self.context();
        self.user_service().get_user(ctx, request)
    }
    fn create_user(
        &self,
        request: CreateUser,
    ) -> BoxFuture<'_, Result<User, <Self::UserService as UserService<Self::Context>>::Error>> {
        let ctx = self.context();
        self.user_service().create_user(ctx, request)
    }
    fn update_user(
        &self,
        request: UpdateUser,
    ) -> BoxFuture<'_, Result<Option<User>, <Self::UserService as UserService<Self::Context>>::Error>>
    {
        let ctx = self.context();
        self.user_service().update_user(ctx, request)
    }
    fn delete_user(
        &self,
        request: DeleteUser,
    ) -> BoxFuture<'_, Result<Option<User>, <Self::UserService as UserService<Self::Context>>::Error>>
    {
        let ctx = self.context();
        self.user_service().delete_user(ctx, request)
    }
    fn build_tower_service(self: Arc<Self>) -> Server<Self>
    where
        <Self::UserService as UserService<Self::Context>>::Error: crate::prelude::Error,
    {
        let service = grpc::ServiceImpl { state: self };
        schema::user::user_service_server::UserServiceServer::new(service)
    }
}
