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
