//! Микросервис, отвечающий за управление пользователями.
#[macro_use]
extern crate diesel;

pub mod data;
pub mod tonic_logic;

use std::env;
use tonic::transport::Server;
use tonic_logic::UsersService;
use tonic_logic::users::users_server::UsersServer;

type MResult<T> = Result<T, Box<dyn std::error::Error>>;

/// Создаёт пул соединений с базой данных и запускает микросервис.
#[tokio::main]
async fn main() -> MResult<()> {
  dotenv::dotenv()?;
  let addr: std::net::SocketAddr = env::var("CALBY_USERS_ADDR")?.parse()?;
  let manager = bb8_diesel
                  ::DieselConnectionManager
                  ::<diesel::pg::PgConnection>
                  ::new(
    env::var("DATABASE_URL")?
  );
  let pool = bb8::Pool::builder().max_size(15).build(manager).await?;
  let users = UsersService { db: pool };
  let svc = UsersServer::new(users);
  Server::builder().add_service(svc).serve(addr).await?;
  Ok(())
}
