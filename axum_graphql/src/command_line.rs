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
        demo: DieselDemo,
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
    UpdatePost {
        #[arg(long)]
        id: i32,
    },
    SelectPost {
        #[arg(long)]
        id: i32,
    },
    DeletePost {
        #[arg(long)]
        target: String,
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

#[derive(Debug, Clone, Subcommand)]
pub enum BookstoreEx {
    Create,
    Update,
    Read {
        #[arg(short)]
        v: i32,
    },
}
