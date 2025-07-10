#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    pub task: Task,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(clap::Subcommand)]
pub enum Task {
    #[default]
    Check,
    Fmt {
        #[arg(last = true)]
        cargo_fmt_args: Vec<String>,
    },
    Test,
}
