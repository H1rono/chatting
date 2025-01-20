use std::sync::Arc;

use anyhow::Context;
use futures::TryFutureExt;
use sqlx::MySqlPool;

use chatting::user::UserServiceImpl;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env_filter =
        tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into());
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    let pool = load_mysql_from_env("MYSQL_")
        .or_else(|_| load_mysql_from_env("MARIADB_"))
        .or_else(|_| load_mysql_from_env("NS_MARIADB_"))
        .await?;
    let user_service = UserServiceImpl;
    let state = Arc::new(State { pool, user_service });
    state.migrate().await?;
    let router = chatting::router::make_router(state);
    let make_service = axum::ServiceExt::into_make_service(router);
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| 8080.to_string())
        .parse()
        .context("Failed to read PORT value")?;
    let addr: std::net::SocketAddr = ([0, 0, 0, 0], port).into();
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("Failed to bind {addr}"))?;
    tracing::info!(%addr, "Listening");
    axum::serve(listener, make_service).await?;
    Ok(())
}

#[derive(Debug, Clone)]
struct State {
    pool: MySqlPool,
    user_service: UserServiceImpl,
}

async fn load_mysql_from_env(prefix: &str) -> anyhow::Result<MySqlPool> {
    macro_rules! var {
        ($n:ident) => {{
            let var_name = format!(concat!("{}", stringify!($n)), prefix);
            std::env::var(&var_name).with_context(|| format!("Failed to read {var_name}"))
        }};
    }

    let hostname = var!(HOSTNAME)?;
    let user = var!(USER)?;
    let password = var!(PASSWORD)?;
    let port: u16 = var!(PORT)?.parse().context("Failed to read PORT value")?;
    let database = var!(DATABASE)?;
    let options = sqlx::mysql::MySqlConnectOptions::new()
        .host(&hostname)
        .username(&user)
        .password(&password)
        .port(port)
        .database(&database);
    sqlx::MySqlPool::connect_with(options)
        .await
        .context("Failed to connect to MySQL")
}

impl State {
    const MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

    async fn migrate(&self) -> anyhow::Result<()> {
        Self::MIGRATOR
            .run(&self.pool)
            .await
            .context("Migration failed")?;
        Ok(())
    }
}

impl AsRef<MySqlPool> for State {
    fn as_ref(&self) -> &MySqlPool {
        &self.pool
    }
}

impl AsRef<UserServiceImpl> for State {
    fn as_ref(&self) -> &UserServiceImpl {
        &self.user_service
    }
}

impl chatting::user::ProvideUserService for State {
    type Context = State;
    type UserService = UserServiceImpl;

    fn user_service(&self) -> &Self::UserService {
        &self.user_service
    }
    fn context(&self) -> &Self::Context {
        self
    }
}
