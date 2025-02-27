use schema::user as generated;

pub use generated::user_service_server::UserServiceServer as Server;
pub use generated::user_service_server::SERVICE_NAME;

use super::ErrorStatus;
use crate::{error::Failure, user as entity};

fn encode_user_id(value: entity::UserId) -> schema::id::UserId {
    let id = value.0.to_string();
    schema::id::UserId { id }
}

fn encode_user(value: entity::User) -> Result<generated::User, Failure> {
    use crate::prelude::convert_timestamp;

    let entity::User {
        id,
        name: entity::UserName(name),
        created_at,
        updated_at,
    } = value;
    let value = generated::User {
        id: Some(encode_user_id(id)),
        name,
        created_at: Some(convert_timestamp(created_at)?),
        updated_at: Some(convert_timestamp(updated_at)?),
    };
    Ok(value)
}

#[derive(Debug, Clone)]
pub struct Service<S>(S);

impl<S> Service<S>
where
    S: entity::ProvideUserService,
{
    pub fn new(inner: S) -> Self {
        Self(inner)
    }
}

#[async_trait::async_trait]
impl<S> generated::user_service_server::UserService for Service<S>
where
    S: entity::ProvideUserService,
{
    async fn get_user(
        &self,
        req: tonic::Request<generated::GetUserRequest>,
    ) -> tonic::Result<tonic::Response<generated::GetUserResponse>> {
        let (_, _, req) = req.into_parts();
        let generated::GetUserRequest { id } = req;
        let id = id
            .ok_or_else(|| Failure::reject_bad_request("User id must be specified"))
            .map_err(ErrorStatus)?
            .id;
        let id: uuid::Uuid = id
            .parse()
            .map_err(|e| Failure::reject_bad_request(format!("Not a UUID: {e}")))
            .map_err(ErrorStatus)?;
        let user = self
            .0
            .get_user(entity::GetUser {
                id: entity::UserId(id),
            })
            .await
            .map_err(ErrorStatus)?;
        let user = encode_user(user).map_err(ErrorStatus)?;
        let res = generated::GetUserResponse { user: Some(user) };
        Ok(tonic::Response::new(res))
    }

    async fn create_user(
        &self,
        req: tonic::Request<generated::CreateUserRequest>,
    ) -> tonic::Result<tonic::Response<generated::CreateUserResponse>> {
        let (_, _, req) = req.into_parts();
        let generated::CreateUserRequest { name } = req;
        let user = self
            .0
            .create_user(entity::CreateUser {
                name: entity::UserName(name),
            })
            .await
            .map_err(ErrorStatus)?;
        let user = encode_user(user).map_err(ErrorStatus)?;
        let res = generated::CreateUserResponse { user: Some(user) };
        Ok(tonic::Response::new(res))
    }

    async fn update_user(
        &self,
        req: tonic::Request<generated::UpdateUserRequest>,
    ) -> tonic::Result<tonic::Response<generated::UpdateUserResponse>> {
        let (_, _, req) = req.into_parts();
        let generated::UpdateUserRequest { id, name } = req;
        let id = id
            .ok_or_else(|| Failure::reject_bad_request("User id must be specified"))
            .map_err(ErrorStatus)?
            .id
            .parse()
            .map_err(|e| Failure::reject_bad_request(format!("Not a UUID: {e}")))
            .map_err(ErrorStatus)?;
        let user = self
            .0
            .update_user(entity::UpdateUser {
                id: entity::UserId(id),
                name: entity::UserName(name),
            })
            .await
            .map_err(ErrorStatus)?;
        let user = encode_user(user).map_err(ErrorStatus)?;
        let res = generated::UpdateUserResponse { user: Some(user) };
        Ok(tonic::Response::new(res))
    }

    async fn delete_user(
        &self,
        req: tonic::Request<generated::DeleteUserRequest>,
    ) -> tonic::Result<tonic::Response<generated::DeleteUserResponse>> {
        let (_, _, req) = req.into_parts();
        let generated::DeleteUserRequest { id } = req;
        let id = id
            .ok_or_else(|| Failure::reject_bad_request("User id must be specified"))
            .map_err(ErrorStatus)?
            .id
            .parse()
            .map_err(|e| Failure::reject_bad_request(format!("Not a UUID: {e}")))
            .map_err(ErrorStatus)?;
        let user = self
            .0
            .delete_user(entity::DeleteUser {
                id: entity::UserId(id),
            })
            .await
            .map_err(ErrorStatus)?;
        let user = encode_user(user).map_err(ErrorStatus)?;
        let res = generated::DeleteUserResponse { user: Some(user) };
        Ok(tonic::Response::new(res))
    }
}
