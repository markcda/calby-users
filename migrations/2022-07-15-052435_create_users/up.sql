create table users (
  id bigserial primary key,
  nick varchar not null,
  displayed_name varchar not null,
  avatar varchar not null,
  email varchar not null,
  phone varchar not null,
  pass bytea not null
)
