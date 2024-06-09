# How to use SeaORM 

## Prerequisite 

```sh 
cargo install sea-orm-cli
```

## Migration init

```sh 
sea-orm-cli migrate init
```

- This will create a folder `migration` at the same path your execute this command. 
- It is a standard bin project.
- Change the sample code from there to meet your need. And follow the generated `README`.

## Generate entities 

1. Generate entity

```sh 
sea-orm-cli generate entity --tables user -o entity/src --lib --with-serde both --model-extra-derives async_graphql::SimpleObject
```

- Table name is `user`, it is case sensitive.
- We generate the files into a new filder named `entity/src`. So, later we could change it into a dependent cargo `lib`.
- So the current tree structure is: 

```sh
zw@zwpdbh:~/code/rust_programming/axum_backend/seaorm_demo$ tree .
.
├── Cargo.toml
├── README.md
├── entity
│   └── src
│       ├── lib.rs
│       ├── prelude.rs
│       └── user.rs
├── migration
│   ├── Cargo.toml
│   ├── README.md
│   └── src
│       ├── lib.rs
│       ├── m20220101_000001_create_table.rs
│       └── main.rs
└── src
    ├── lib.rs
    └── query
        └── mod.rs

6 directories, 12 files
```

2. Initialize and configure the entity lib 

```sh 
cd entity
cargo init 
```

It shows warning:
```txt
warning: compiling this new package may not work due to invalid workspace configuration

current package believes it's in a workspace when it's not:
current:   /home/zw/code/rust_programming/axum_backend/seaorm_demo/entity/./Cargo.toml
workspace: /home/zw/code/rust_programming/axum_backend/Cargo.toml

this may be fixable by adding `seaorm_demo/entity` to the `workspace.members` array of the manifest located at: /home/zw/code/rust_programming/axum_backend/Cargo.toml
Alternatively, to keep it out of the workspace, add the package to the `workspace.exclude` array, or add an empty `[workspace]` table to the package's manifest.
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
```

So, we edit the **workspace**'s `Cargo.toml` to be:
```toml
[workspace]
resolver = "2"

members = [
  ...
  "seaorm_demo",
  "seaorm_demo/migration",
  "seaorm_demo/entity",
]
```

Add dependencies by edit `entity`'s `Cargo.toml`:

```toml
[dependencies]
sea-orm = { version = "0.12", features = [
  "sqlx-postgres",
  "runtime-tokio-rustls",
  "macros",
  "with-uuid",
  "with-chrono",
  "with-json",
  "with-bigdecimal",
] }
async-graphql = "6.0.6"
serde_json = "1.0"
serde = { version = "1" }
```

3. Configure `seaorm_demo` depends on `seaorm_demo/entity`

```toml
[dependencies]
...
entity = { path = "./entity" }
```

## References

- [Axum+SeaORM+Async-graphql: Building a GraphQL Service from Scratch](https://dev.to/yexiyue/axumseaormasync-graphql-building-a-graphql-service-from-scratch-52kk)