use crate::command_line::Arguments;
use crate::command_line::SubCommand;
use clap::Parser;
use command_line::sqlx;
use dotenv::dotenv;

mod command_line;

#[tokio::main]
async fn main() {
    let _ = dotenv().ok();
    let _ = tracer::setup_tracer();

    let args = Arguments::parse();
    match args.cmd {
        SubCommand::GraphqlDemo { port } => {
            let _ = graphql_demo::run(&port).await;
        }
        SubCommand::SqlxDemo { case } => {
            let _ = sqlx::run(case).await;
        }
        SubCommand::DieselDemo { case } => command_line::diesel::run(case),
    }
}
