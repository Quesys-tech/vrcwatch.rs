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
    #[command(flatten)]
    run_args: run_watch::RunArgs,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
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
            run_watch::run_watch(&cli.run_args).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cli, Command};
    use clap::Parser;

    #[test]
    fn parses_demo_as_default_run_option() {
        let cli = Cli::try_parse_from(["vrcwatch-rs", "--demo"]).unwrap();

        assert!(matches!(cli.command, None));
        assert!(cli.run_args.is_demo());
    }

    #[test]
    fn parses_demo_on_run_subcommand() {
        let cli = Cli::try_parse_from(["vrcwatch-rs", "run", "--demo"]).unwrap();

        match cli.command {
            Some(Command::Run(args)) => assert!(args.is_demo()),
            _ => panic!("expected run subcommand"),
        }
    }
}
