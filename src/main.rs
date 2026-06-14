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
            let installed = ovr_manifest::status().await;
            match installed {
                Ok(true) => println!("VRCWatch is installed."),
                Ok(false) => println!("VRCWatch is not installed."),
                Err(e) => eprintln!("Error checking installation status: {:#?}", e),
            }
        }
        Some(Command::Install) => {
            println!("Installation functionality is not implemented yet.");
        }
        Some(Command::Uninstall) => match ovr_manifest::status().await {
            Ok(true) => {
                println!("Uninstallation functionality is not implemented yet.")
            }
            Ok(false) => println!("VRCWatch is not installed, so it cannot be uninstalled."),
            Err(e) => eprintln!("Error checking installation status: {:#?}", e),
        },
        None => {
            run_watch::run_watch(&run_watch::RunArgs::default()).await;
        }
    }
}
