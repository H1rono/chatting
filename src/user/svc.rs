use anyhow::Context;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, MySqlPool};
use uuid::Uuid;

use crate::error::Failure;

#[derive(Debug, Clone, Copy, Default)]
pub struct Impl;

// MARK: helper types

#[derive(Debug, Clone, Hash, Deserialize, Serialize, FromRow)]
struct UserRow {
    pub id: Uuid,
    pub name: String,
    pub created_at: super::Timestamp,
    pub updated_at: super::Timestamp,
}

impl From<UserRow> for super::User {
    fn from(value: UserRow) -> Self {
        Self {
            id: super::UserId(value.id),
            name: super::UserName(value.name),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

// MARK: helper fns

async fn get_user(
    pool: &MySqlPool,
    request: super::GetUser,
) -> Result<Option<super::User>, Failure> {
    let super::GetUser {
        id: super::UserId(id),
    } = request;
    let user: Option<UserRow> = sqlx::query_as(r#"SELECT * FROM `users` WHERE `id` = ?"#)
        .bind(id)
        .fetch_optional(pool)
        .await
        .context("Failed to fetch an user from DB")?;
    Ok(user.map(super::User::from))
}

async fn create_user(pool: &MySqlPool, request: super::CreateUser) -> Result<super::User, Failure> {
    let id = Uuid::now_v7();
    let super::CreateUser {
        name: super::UserName(name),
    } = request;
    sqlx::query(
        r#"
        INSERT INTO `users` (`id`, `name`, `created_at`, `updated_at`)
        VALUES (?, ?, NOW(), NOW())
    "#,
    )
    .bind(id)
    .bind(name)
    .execute(pool)
    .await
    .context("Failed to create an user to DB")?;
    let user: UserRow = sqlx::query_as(r#"SELECT * FROM `users` WHERE `id` = ?"#)
        .bind(id)
        .fetch_one(pool)
        .await
        .context("Failed to fetch an user from DB")?;
    Ok(user.into())
}

async fn update_user(
    pool: &MySqlPool,
    request: super::UpdateUser,
) -> Result<Option<super::User>, Failure> {
    // TODO: transaction
    let super::UpdateUser {
        id: super::UserId(id),
        name: super::UserName(name),
    } = request;
    sqlx::query(
        r#"
        UPDATE `users`
        SET `name` = ?, `updated_at` = NOW()
        WHERE `id` = ?
    "#,
    )
    .bind(name)
    .bind(id)
    .execute(pool)
    .await
    .context("Failed to update an user in DB")?;
    get_user(
        pool,
        super::GetUser {
            id: super::UserId(id),
        },
    )
    .await
}

async fn delete_user(
    pool: &MySqlPool,
    request: super::DeleteUser,
) -> Result<Option<super::User>, Failure> {
    // TODO: transaction
    let super::DeleteUser {
        id: super::UserId(id),
    } = request;
    let get_request = super::GetUser {
        id: super::UserId(id),
    };
    let Some(user) = get_user(pool, get_request).await? else {
        return Ok(None);
    };
    sqlx::query(r#"DELETE FROM `users` WHERE `id` = ?"#)
        .bind(id)
        .execute(pool)
        .await
        .context("Failed to delete an user from DB")?;
    Ok(Some(user))
}

// MARK: impl UserService

impl<Ctx> super::UserService<Ctx> for Impl
where
    Ctx: AsRef<MySqlPool> + Send + Sync,
{
    async fn get_user<'a>(
        &'a self,
        ctx: &'a Ctx,
        request: super::GetUser,
    ) -> Result<super::User, Failure> {
        get_user(ctx.as_ref(), request)
            .await?
            .ok_or_else(|| Failure::reject_not_found("User not found"))
    }

    async fn create_user<'a>(
        &'a self,
        ctx: &'a Ctx,
        request: super::CreateUser,
    ) -> Result<super::User, Failure> {
        create_user(ctx.as_ref(), request).await
    }

    async fn update_user<'a>(
        &'a self,
        ctx: &'a Ctx,
        request: super::UpdateUser,
    ) -> Result<super::User, Failure> {
        update_user(ctx.as_ref(), request)
            .await?
            .ok_or_else(|| Failure::reject_not_found("User not found"))
    }

    async fn delete_user<'a>(
        &'a self,
        ctx: &'a Ctx,
        request: super::DeleteUser,
    ) -> Result<super::User, Failure> {
        delete_user(ctx.as_ref(), request)
            .await?
            .ok_or_else(|| Failure::reject_not_found("User not found"))
    }
}
