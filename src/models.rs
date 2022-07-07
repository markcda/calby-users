#[derive(Queryable)]
pub struct Post {
  pub id: i64,
  pub nick: String,
  pub displayed_name: String,
  pub avatar: String,
  pub email: String,
  pub phone: String,
  pub pass: Vec<u8>,
}
