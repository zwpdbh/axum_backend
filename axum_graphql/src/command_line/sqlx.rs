use clap::Subcommand;

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

pub async fn run(case: SqlCase) {
    match case {
        SqlCase::Test => {
            let _ = sqlx_demo::test().await.unwrap();
        }
        SqlCase::Bookstore { example } => match example {
            BookstoreEx::Create => {
                let _ = sqlx_demo::create_book_example().await.unwrap();
            }
            BookstoreEx::Update => {
                let _ = sqlx_demo::update_book_example().await.unwrap();
            }
            BookstoreEx::Read { v } => {
                let _ = sqlx_demo::read_book_example(v).await.unwrap();
            }
        },
    }
}
