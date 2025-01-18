use futures::future::BoxFuture;
use sqlx::MySqlPool;

#[derive(Debug, Clone, Copy, Default)]
pub struct Impl;

impl<Ctx> super::UserService<Ctx> for Impl
where
    Ctx: AsRef<MySqlPool>,
{
    type Error = super::error::Error;

    fn get_user<'a>(
        &'a self,
        ctx: &'a Ctx,
        request: super::GetUser,
    ) -> BoxFuture<'a, Result<Option<super::User>, Self::Error>> {
        todo!()
    }

    fn create_user<'a>(
        &'a self,
        ctx: &'a Ctx,
        request: super::CreateUser,
    ) -> BoxFuture<'a, Result<super::User, Self::Error>> {
        todo!()
    }

    fn update_user<'a>(
        &'a self,
        ctx: &'a Ctx,
        request: super::UpdateUser,
    ) -> BoxFuture<'a, Result<Option<super::User>, Self::Error>> {
        todo!()
    }

    fn delete_user<'a>(
        &'a self,
        ctx: &'a Ctx,
        request: super::DeleteUser,
    ) -> BoxFuture<'a, Result<Option<super::User>, Self::Error>> {
        todo!()
    }
}
