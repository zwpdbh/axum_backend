# Learn GraphQL with Axum

## References for GraphQL

- [Async-graphql Book](https://async-graphql.github.io/async-graphql/en/index.html)
- [GraphQL in Rust](https://romankudryashov.com/blog/2020/12/graphql-rust/)
- [Async GraphQL with Rust: Data and Graphs](https://commerce.nearform.com/blog/2022/data-and-graphs)
- [How to Build a Powerful GraphQL API with Rust](https://oliverjumpertz.com/blog/how-to-build-a-powerful-graphql-api-with-rust/)

## References for Axum

- [Axum Tutorial For Beginners](https://github.com/programatik29/axum-tutorial)

## Troubleshooting 

- Compiling `axum_graphql` error: "/usr/bin/ld: cannot find -lpq"
  Solution: 
  
  ```sh
  sudo apt-get update
  sudo apt-get install libpq-dev
  ```