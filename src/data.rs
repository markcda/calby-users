//! Объявление типов данных и структур таблиц для ORM и логики.
use diesel::Queryable;

#[derive(Queryable)]
pub struct User {
  pub id: i64,
  pub nick: String,
  pub displayed_name: String,
  pub avatar: String,
  pub email: String,
  pub phone: String,
  pub pass: Vec<u8>,
}

table! {
  users (id) {
    id -> Int8,
    nick -> Varchar,
    displayed_name -> Varchar,
    avatar -> Varchar,
    email -> Varchar,
    phone -> Varchar,
    pass -> Bytea,
  }
}
