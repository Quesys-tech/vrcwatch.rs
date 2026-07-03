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
#[command(version, about, long_about = None)]
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
    use clap::error::ErrorKind;
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

    #[test]
    fn parses_short_version_flag() {
        assert_version_flag("-V");
    }

    #[test]
    fn parses_long_version_flag() {
        assert_version_flag("--version");
    }

    fn assert_version_flag(flag: &str) {
        let err = match Cli::try_parse_from(["vrcwatch-rs", flag]) {
            Ok(_) => panic!("expected version display"),
            Err(err) => err,
        };

        assert_eq!(err.kind(), ErrorKind::DisplayVersion);
        assert!(err
            .to_string()
            .contains(&format!("vrcwatch-rs {}", env!("CARGO_PKG_VERSION"))));
    }
}
