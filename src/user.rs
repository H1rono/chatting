use std::future::Future;

use serde::{Deserialize, Serialize};

use crate::{error::Failure, prelude::Timestamp};

mod svc;

pub use svc::Impl as UserServiceImpl;

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
    fn get_user<'a>(
        &'a self,
        ctx: &'a Context,
        request: GetUser,
    ) -> impl Future<Output = Result<User, Failure>> + Send;
    fn create_user<'a>(
        &'a self,
        ctx: &'a Context,
        request: CreateUser,
    ) -> impl Future<Output = Result<User, Failure>> + Send;
    fn update_user<'a>(
        &'a self,
        ctx: &'a Context,
        request: UpdateUser,
    ) -> impl Future<Output = Result<User, Failure>> + Send;
    fn delete_user<'a>(
        &'a self,
        ctx: &'a Context,
        request: DeleteUser,
    ) -> impl Future<Output = Result<User, Failure>> + Send;
}

pub trait ProvideUserService: Send + Sync + 'static {
    type Context: ?Sized;
    type UserService: UserService<Self::Context>;

    fn user_service(&self) -> &Self::UserService;
    fn context(&self) -> &Self::Context;

    fn get_user(&self, request: GetUser) -> impl Future<Output = Result<User, Failure>> + Send {
        let ctx = self.context();
        self.user_service().get_user(ctx, request)
    }
    fn create_user(
        &self,
        request: CreateUser,
    ) -> impl Future<Output = Result<User, Failure>> + Send {
        let ctx = self.context();
        self.user_service().create_user(ctx, request)
    }
    fn update_user(
        &self,
        request: UpdateUser,
    ) -> impl Future<Output = Result<User, Failure>> + Send {
        let ctx = self.context();
        self.user_service().update_user(ctx, request)
    }
    fn delete_user(
        &self,
        request: DeleteUser,
    ) -> impl Future<Output = Result<User, Failure>> + Send {
        let ctx = self.context();
        self.user_service().delete_user(ctx, request)
    }
}

impl<T> ProvideUserService for std::sync::Arc<T>
where
    T: ProvideUserService,
{
    type Context = T::Context;
    type UserService = T::UserService;

    fn context(&self) -> &Self::Context {
        T::context(self)
    }
    fn user_service(&self) -> &Self::UserService {
        T::user_service(self)
    }
}
