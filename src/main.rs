use clap::{Parser, Subcommand};

mod osc_sender;
mod ovr_manifest;
mod run_watch;

#[derive(Subcommand)]
enum Command {
    /// Run VRCWatch with specified options
    Run(run_watch::RunArgs),
    /// Check if VRCWatch is installed in SteamVR
    Status,
    /// Install VRCWatch in SteamVR (not implemented yet)
    Install,
    /// Uninstall VRCWatch from SteamVR (not implemented yet)
    Uninstall,
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
        Some(Command::Status) => {
            ovr_manifest::status().await;
        }
        Some(Command::Install) => {
            ovr_manifest::install().await;
        }
        Some(Command::Uninstall) => {
            ovr_manifest::uninstall().await;
        }
        None => {
            run_watch::run_watch(&run_watch::RunArgs::default()).await;
        }
    }
}
