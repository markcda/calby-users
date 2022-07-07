use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

type MResult<T> = Result<T, Box<dyn std::error::Error>>;

#[tokio::main]
pub async fn main() -> MResult<()> {
  dotenv()?;
  
}
