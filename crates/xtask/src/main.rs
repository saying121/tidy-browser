use clap::Parser;
use xtask::args::{self, Task};

fn main() -> anyhow::Result<()> {
    xtask::show_rust_env()?;

    let args = args::Args::parse();
    match args.task {
        Task::Check => {
            xtask::check_lb()?;
            xtask::check_tbe()?;
        },
        Task::Fmt { cargo_fmt_args: fmt } => xtask::fmt_task(fmt.into_iter().map(Into::into))?,
        Task::Test => xtask::test_task()?,
    }
    Ok(())
}
