#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use dotenv::dotenv;
use std::{env, net::SocketAddr};
use diesel::{r2d2::ConnectionManager, PgConnection, QueryDsl};
use tokio_diesel::*;

type MResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> MResult<()> {
  dotenv()?;
  let _addr: SocketAddr = env::var("CALBY_USERS_ADDR")?.parse()?;
  println!("{}", env::var("CALBY_DB_AUTH")?);
  let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL")?);
  let pool = r2d2::Pool::builder().max_size(15).build(manager)?;
  let num_users: i64 = schema::users::table.count().get_result_async(&pool).await?;
  println!("There are {:?} users.", num_users);
  Ok(())
}
