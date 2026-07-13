use clap::{Parser, Subcommand};
use std::error::Error;
use std::path::PathBuf;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

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
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let cli = Cli::parse();
    let debug = match &cli.command {
        Some(Command::Run(args)) => args.debug_enabled(),
        None => cli.run_args.debug_enabled(),
        _ => false,
    };
    let _log_guard = initialize_logging(debug)?;

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

    Ok(())
}

fn initialize_logging(
    debug: bool,
) -> Result<tracing_appender::non_blocking::WorkerGuard, Box<dyn Error + Send + Sync>> {
    let log_directory = log_directory()?;
    std::fs::create_dir_all(&log_directory)?;

    let file_appender = tracing_appender::rolling::daily(
        &log_directory,
        format!("{}.jsonl", env!("CARGO_PKG_NAME")),
    );
    let (writer, guard) = tracing_appender::non_blocking(file_appender);
    let log_level = if debug {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };

    let file_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_current_span(false)
        .with_span_list(false)
        .with_writer(writer)
        .with_filter(log_level);
    let console_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_filter(log_level);

    tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer)
        .try_init()?;

    info!(log_directory = %log_directory.display(), "File logging initialized");
    Ok(guard)
}

fn log_directory() -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
    dirs::data_local_dir()
        .map(|path| path.join("tech.qsys").join(env!("CARGO_PKG_NAME")))
        .ok_or_else(|| "local data directory is unavailable".into())
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
