use crate::command_line::Arguments;
use crate::command_line::BookstoreEx;
use crate::command_line::SubCommand;
use clap::Parser;
use command_line::DieselDemo;
use command_line::SqlCase;
#[allow(unused)]
use dotenv::dotenv;

mod command_line;

#[tokio::main]
async fn main() {
    let _ = tracer::setup_tracer();

    let args = Arguments::parse();
    match args.cmd {
        SubCommand::GraphqlDemo { port } => {
            let _ = graphql_demo::run(&port).await;
        }
        SubCommand::SqlxDemo { case } => match case {
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
        },
        SubCommand::DieselDemo { demo } => match demo {
            DieselDemo::ShowPost => diesel_demo::show_post(),
            DieselDemo::CreatePost { title, body } => diesel_demo::create_post(&title, &body),
            DieselDemo::UpdatePost { id } => diesel_demo::update_post(id),
            DieselDemo::SelectPost { id } => diesel_demo::select_post(id),
            DieselDemo::DeletePost { target } => diesel_demo::delete_post(&target),
        },
    }
}
