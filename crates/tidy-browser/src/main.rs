use clap::Parser;
use tidy_browser::args::{self};
use tracing_subscriber::EnvFilter;

#[snafu::report]
#[tokio::main]
async fn main() -> tidy_browser::error::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn")),
        )
        .with_test_writer()
        .init();

    let args = args::Args::parse();
    tidy_browser::cli::run_cli(args).await
}
