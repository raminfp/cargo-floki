mod cargo;
mod config;
mod error;
mod projects;

use clap::{Parser, Subcommand};
pub use config::Config;
pub use error::{Error, Reportable};
use log::LevelFilter;
use simplelog::{ColorChoice, ConfigBuilder, TermLogger, TerminalMode};
use std::env;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Build artifacts in release mode, with optimizations
    #[arg(short, long)]
    release: bool,

    /// Verbosity (none: errors & warnings, -v: verbose, --vv: very verbose, --vvv: output everything)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Path to configuration file (defaults to './floki.toml')
    #[arg(short, long)]
    config: Option<String>,
}

impl Cli {
    pub fn read_config(&self) -> Result<Config, Reportable> {
        Config::read(&self.config)
    }
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Adds a default floki.toml file to current directory
    Init,
    /// Compile the client and server
    Build,
    /// Remove the target directories (in app, client and server)
    Clean,
    /// Run the cargo tests for app, client and server
    Test,
    /// Run the cargo update for app, client and server
    Update,
    /// Run app
    Run,
    /// Docs
    Doc,
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // when running as cargo floki, the second argument is "floki" which
    // clap doesn't expect
    if args.get(1).map(|a| a == "floki").unwrap_or(false) {
        args.remove(1);
    }

    let args = Cli::parse_from(&args);

    setup_logging(args.verbose);
    if let Err(e) = try_main(args) {
        log::error!("{e}")
    }
}

fn try_main(args: Cli) -> Result<(), Reportable> {
    match args.command {
        Commands::Init => Config::save_default_file(),
        Commands::Build => cargo::run("build", args),
        Commands::Test => cargo::run("test", args),
        Commands::Clean => cargo::run("clean", args),
        Commands::Update => cargo::run("update", args),
        Commands::Doc => cargo::run("doc", args),
        Commands::Run => todo!(),
    }
}

fn setup_logging(verbose: u8) {
    let log_level = match verbose {
        0 => LevelFilter::Warn,
        1 => LevelFilter::Info,
        2 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
    };
    let config = ConfigBuilder::default()
        .set_time_level(LevelFilter::Off)
        .build();
    TermLogger::init(log_level, config, TerminalMode::Stderr, ColorChoice::Auto)
        .expect("Failed to start logger");
    log::info!("Log level set to: {log_level}");
}
