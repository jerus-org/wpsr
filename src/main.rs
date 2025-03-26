use clap::Parser;
use slb::{Cli, Commands};
use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() {
    let args = Cli::parse();
    get_logging(args.logging.log_level_filter());

    match args.cmd {
        Commands::Prepare(prepare) => prepare.run(),
        Commands::Solve(solve) => solve.run(),
    }
}

pub fn get_logging(verbosity: log::LevelFilter) {
    let filter = EnvFilter::from(format!(
        "slb={}",
        if verbosity == log::LevelFilter::Trace {
            log::LevelFilter::Debug
        } else {
            verbosity
        }
    ));

    let log_subscriber = tracing_subscriber::FmtSubscriber::builder()
        .pretty()
        .with_env_filter(filter)
        .finish();

    let _ = tracing::subscriber::set_global_default(log_subscriber)
        .map_err(|_| eprintln!("Unable to set global default subscriber!"));

    info!("Initialised logging to console at {verbosity}");
}
