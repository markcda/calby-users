//! Логика микросервиса.

/// Модуль, который подключает код, скомпилированный из Protobuf в Rust.
pub mod users {
  tonic::include_proto!("users");
}

use tonic::{Request, Response, Status};
use users::users_server::Users;
use users::{CreateUserRequest, ShortenedUserData};

/// Структура, содержащая данные нашего микросервиса.
#[derive(Clone)]
pub struct UsersService {
  pub db: bb8::Pool<bb8_diesel::DieselConnectionManager<diesel::pg::PgConnection>>,
}

#[tonic::async_trait]
impl Users for UsersService {
  /// Создаёт пользователя.
  async fn create_user(&self, _request: Request<CreateUserRequest>)
    -> Result<Response<ShortenedUserData>, Status>
  {
    unimplemented!();
  }
}
