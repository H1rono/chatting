use anyhow::Context;
use sqlx::MySqlPool;

use chatting::user::UserServiceImpl;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
struct State {
    pool: MySqlPool,
    user_service: UserServiceImpl,
}
async fn load_mysql_from_env(prefix: &str) -> anyhow::Result<MySqlPool> {
    macro_rules! var {
        ($n:ident) => {
            std::env::var(&format!(concat!("{}", stringify!($n)), prefix))
                .context(concat!("Failed to read", stringify!($n)))
        };
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
}
