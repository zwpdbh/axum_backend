use clap::{Parser, Subcommand, ValueEnum};

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
    AxumSqlx {
        #[arg(long, short)]
        port: String,
    },
    AxumSeaorm {
        #[arg(long, short)]
        port: String,
    },
    Sqlx {
        #[clap(subcommand)]
        case: SqlCase,
    },
    DieselDemo {
        #[clap(subcommand)]
        demo: DieselDemo,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum SqlCase {
    Test,
    Bookstore {
        #[clap(subcommand)]
        example: BookstoreEx,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum DieselDemo {
    ShowPost,
    CreatePost {
        #[arg(long)]
        title: String,
        #[arg(long)]
        body: String,
    },
}

#[derive(Debug, Clone, Subcommand)]
pub enum BookstoreEx {
    Create,
    Update,
    Read {
        #[arg(short)]
        v: ExVersion,
    },
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ExVersion {
    V1,
    V2,
    V3,
    V4,
}
