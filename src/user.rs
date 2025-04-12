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
pub struct GetUserParams {
    pub id: UserId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct CreateUserParams {
    pub name: UserName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct UpdateUserParams {
    pub id: UserId,
    pub name: UserName,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct DeleteUserParams {
    pub id: UserId,
}

pub trait UserService<Context: ?Sized>: Send + Sync + 'static {
    fn get_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: GetUserParams,
    ) -> impl Future<Output = Result<User, Failure>> + Send;
    fn create_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: CreateUserParams,
    ) -> impl Future<Output = Result<User, Failure>> + Send;
    fn update_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: UpdateUserParams,
    ) -> impl Future<Output = Result<User, Failure>> + Send;
    fn delete_user<'a>(
        &'a self,
        ctx: &'a Context,
        params: DeleteUserParams,
    ) -> impl Future<Output = Result<User, Failure>> + Send;
}

pub trait ProvideUserService: Send + Sync + 'static {
    type Context: ?Sized;
    type UserService: UserService<Self::Context>;

    fn user_service(&self) -> &Self::UserService;
    fn context(&self) -> &Self::Context;

    fn get_user(
        &self,
        params: GetUserParams,
    ) -> impl Future<Output = Result<User, Failure>> + Send {
        let ctx = self.context();
        self.user_service().get_user(ctx, params)
    }
    fn create_user(
        &self,
        params: CreateUserParams,
    ) -> impl Future<Output = Result<User, Failure>> + Send {
        let ctx = self.context();
        self.user_service().create_user(ctx, params)
    }
    fn update_user(
        &self,
        params: UpdateUserParams,
    ) -> impl Future<Output = Result<User, Failure>> + Send {
        let ctx = self.context();
        self.user_service().update_user(ctx, params)
    }
    fn delete_user(
        &self,
        params: DeleteUserParams,
    ) -> impl Future<Output = Result<User, Failure>> + Send {
        let ctx = self.context();
        self.user_service().delete_user(ctx, params)
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
