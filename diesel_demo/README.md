# How to setup diesel in workspace

## Install diesel-cli for postgresql

```sh
cargo install diesel_cli --no-default-features --features postgres
```

## Config `.env`

```sh
# To enable JAEGER in tracing 
JAEGER_ENABLED=true
# To enable compile time sql validation from sqlx
DATABASE_URL=postgres://postgres:postgres@localhost:5432/myapp
# Diesel config file: used by `diesel setup`
DIESEL_CONFIG_FILE=diesel.toml
```

## Config `diesel.toml`

```sh
# For documentation on how to configure this file,
# see https://diesel.rs/guides/configuring-diesel-cli

[print_schema]
file = "src/schema.rs"
custom_type_derives = ["diesel::query_builder::QueryId", "Clone"]

[migrations_directory]
dir = "/home/zw/code/rust_programming/axum_backend/diesel_demo/migrations"
```

## Run diesel-cli

From the path which contains the `.env` file run `diesel-cli`
  
1. diesel setup
  
     ```sh
     diesel setup
     ```

     This will create migrations folder defined from `diesel.toml` file.

2. create migration

     ``` sh
     diesel migration generate create_posts
     Creating migrations/2024-06-05-020419_create_posts/up.sql
     Creating migrations/2024-06-05-020419_create_posts/down.sql
     ```

     Edit the generated sql file to write your migration.

3. Run migration

    ```sh
    diesel migration run/redo 
    ```

    - This will run the migration.
    - In addition, this will generate a `schema.rs` file defined from `diesel.toml` file.
    - Any time we run or revert a migration, this file will get automatically updated.

## References

- [Getting Started with Diesel](https://diesel.rs/guides/getting-started)
- [axum-diesel-async-graphql-template](https://github.com/mishaszu/axum-diesel-async-graphql-template/tree/main)