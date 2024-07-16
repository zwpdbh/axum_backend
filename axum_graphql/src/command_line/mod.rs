use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author = "zhaowei", version, about)]
pub struct Arguments {
    #[clap(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Subcommand, Debug, Clone)]
pub enum SubCommand {
    /// Ex: cargo run -- axum-sqlx --port 3000,
    /// then visit http://localhost:3000/graphql to see the graphql playground
    SeaormDemo {
        #[arg(long, short)]
        port: i32,
    },
}
