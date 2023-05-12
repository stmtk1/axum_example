# 実行方法

1. postgresqlのサーバを用意します。
2. `cargo install sqlx-cli`を実行します。
3. `$DATABASE_URL`を設定します。
4. `sqlx database create`を実行します。
5. `sqlx migrate run`を実行します。
6. `cargo run`を実行します