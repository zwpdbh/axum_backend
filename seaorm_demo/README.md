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

## References

- [Axum+SeaORM+Async-graphql: Building a GraphQL Service from Scratch](https://dev.to/yexiyue/axumseaormasync-graphql-building-a-graphql-service-from-scratch-52kk)