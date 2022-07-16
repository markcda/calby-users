# calby-users

Микросервис для управления пользователями в веб-приложении Calby.

## Сборка

Выполните команду `cargo build --release`.

## Настройка сервера

Запустите инстанс PostgreSQL и создайте файл `.env` следующего содержания:

```env
CALBY_USERS_ADDR=ip:port
DATABASE_URL=postgres://user:pass@localhost/database
```

Установите Diesel: `cargo install diesel_cli --no-default-features --features postgres`.

Запустите настройку Diesel: `diesel setup` - и выполните первичную миграцию: `diesel migration run`.

## Запуск сервера

Выполните команду `cargo run --release`.
