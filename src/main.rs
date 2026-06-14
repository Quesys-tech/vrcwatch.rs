use clap::{Parser, Subcommand};

mod osc_sender;
mod run_watch;

#[derive(Subcommand)]
enum Command {
    Run(run_watch::RunArgs),
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[tokio::main]
async fn main() {
    match Cli::parse().command {
        Some(Command::Run(args)) => {
            run_watch::run_watch(&args).await;
        }
        None => {
            run_watch::run_watch(&run_watch::RunArgs::default()).await;
        }
    }
}
