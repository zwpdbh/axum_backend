pub mod diesel;
pub mod sqlx;

use clap::{Parser, Subcommand};
use diesel::DieselDemoCase;
use sqlx::SqlCase;

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
    GraphqlDemo {
        #[arg(long, short)]
        port: String,
    },
    SqlxDemo {
        #[clap(subcommand)]
        case: SqlCase,
    },
    DieselDemo {
        #[clap(subcommand)]
        case: DieselDemoCase,
    },
    SeaormDemo {
        #[arg(long, short)]
        port: i32,
    },
}
