use crate::command_line::Arguments;
use crate::command_line::SubCommand;
use clap::Parser;
use dotenv::dotenv;

mod command_line;

#[tokio::main]
async fn main() {
    let _ = dotenv().ok();
    let _ = tracer::setup_tracer();

    let args = Arguments::parse();
    match args.cmd {
        SubCommand::SeaormDemo { port } => {
            let _ = seaorm_demo::run(port).await;
        }
    }
}
