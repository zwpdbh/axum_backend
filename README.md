# How to Build a Powerful GraphQL API with Rust

## Show installed rust versions

```sh
$ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/zw/.rustup

installed toolchains
--------------------

stable-x86_64-unknown-linux-gnu
nightly-2024-01-18-x86_64-unknown-linux-gnu
nightly-2024-01-19-x86_64-unknown-linux-gnu
nightly-2024-01-29-x86_64-unknown-linux-gnu
nightly-x86_64-unknown-linux-gnu (default)
1.72.0-x86_64-unknown-linux-gnu

active toolchain
----------------

1.72.0-x86_64-unknown-linux-gnu (overridden by '/home/zw/code/rust_programming/axum-graphql/rust-toolchain.toml')
rustc 1.72.0 (5680fa18f 2023-08-23)
```

## Update stable rust

```sh 
rustup update stable
info: syncing channel updates for 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.78.0 (9b00956e5 2024-04-29)

info: checking for self-update
```

## Troubleshooting 

- `rustup update` failing with could not rename component file
  - Solution: `rustup toolchain uninstall stable && rustup toolchain install stable`