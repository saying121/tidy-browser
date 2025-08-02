use std::path::PathBuf;

use clap::{
    builder::{IntoResettable, OsStr, Resettable},
    ArgAction, ValueHint,
};

#[derive(Clone)]
#[derive(Debug)]
#[derive(clap::Parser)]
#[command(version, about, long_about)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Option<SubCmd>,

    #[arg(
        short,
        long,
        default_value("results"),
        verbatim_doc_comment,
        value_hint(ValueHint::DirPath)
    )]
    /// Specify output dir
    /// binary-cookies ignore the arg
    pub output_dir: PathBuf,

    #[arg(short, long)]
    /// All browsers data
    pub all_browsers: bool,

    #[arg(long, default_value(","), value_hint(ValueHint::Other))]
    /// Csv separator
    pub sep: String,

    #[arg(long, id("domain"), value_hint(ValueHint::Url))]
    /// Filter by host/domain
    pub host: Option<String>,

    #[arg(long, default_value(Format::Csv))]
    /// Out format
    pub out_format: Format,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
pub enum Format {
    #[default]
    Csv,
    Json,
    JsonLines,
}

impl clap::ValueEnum for Format {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Csv, Self::Json, Self::JsonLines]
    }
    fn to_possible_value<'a>(&self) -> ::std::option::Option<clap::builder::PossibleValue> {
        match self {
            Self::Csv => Some(clap::builder::PossibleValue::new("csv")),
            Self::Json => Some(clap::builder::PossibleValue::new("json")),
            Self::JsonLines => Some(clap::builder::PossibleValue::new("jsonl")),
        }
    }
}

impl IntoResettable<OsStr> for Format {
    fn into_resettable(self) -> clap::builder::Resettable<OsStr> {
        Resettable::Value(match self {
            Format::Csv => "csv".into(),
            Format::Json => "json".into(),
            Format::JsonLines => "jsonl".into(),
        })
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(clap::Subcommand)]
pub enum SubCmd {
    /// Chromium based
    Chromium(ChromiumArgs),
    /// Firefox based
    Firefox(FirefoxArgs),
    /// Parse BinaryCookies
    BinaryCookies(BinaryCookiesArgs),
    #[cfg(target_os = "macos")]
    /// Safari
    Safari(SafariArgs),
    /// Generates completions for the specified SHELL, sends them to `stdout` and exits.
    Completions {
        shell: clap_complete_command::Shell,
    },
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(clap::ValueEnum)]
#[derive(Hash)]
#[derive(strum::EnumIter)]
pub enum Value {
    #[default]
    Cookie,
    Login,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(clap::Args)]
pub struct SafariArgs {
    #[arg(short, long, value_delimiter(','), action(ArgAction::Append))]
    /// Only support cookie
    pub values: Vec<Value>,

    #[arg(long, value_hint(ValueHint::FilePath))]
    pub cookies_path: Option<PathBuf>,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(clap::Args)]
pub struct ChromiumArgs {
    #[arg(short, long)]
    pub name: ChromiumName,

    #[arg(long, id("DIR"), verbatim_doc_comment, value_hint(ValueHint::DirPath))]
    /// When browser is started with `--user-data-dir=DIR   Specify the directory that user data (your "profile") is kept in.`
    #[cfg_attr(target_os = "linux", doc = "[example value: ~/.config/google-chrome]")]
    #[cfg_attr(
        target_os = "macos",
        doc = "[example value: ~/Library/Application Support/Google/Chrome]"
    )]
    #[cfg_attr(
        target_os = "windows",
        doc = r"[example value: ~\AppData\Local\Google\Chrome\User Data]"
    )]
    pub user_data_dir: Option<PathBuf>,

    #[arg(short, long, value_delimiter(','), action(ArgAction::Append))]
    pub values: Vec<Value>,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(clap::ValueEnum)]
#[clap(rename_all = "PascalCase")]
#[derive(strum::EnumIter)]
pub enum ChromiumName {
    Chrome,
    Edge,
    Chromium,
    Brave,
    Vivaldi,
    Yandex,
    Opera,
    #[cfg(not(target_os = "linux"))]
    Arc,
    #[cfg(not(target_os = "linux"))]
    OperaGX,
    #[cfg(not(target_os = "linux"))]
    CocCoc,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(clap::Args)]
pub struct FirefoxArgs {
    #[arg(short, long)]
    pub name: FirefoxName,

    #[arg(long, id("DIR"), value_hint(ValueHint::DirPath))]
    /// Browser data dir.
    #[cfg_attr(target_os = "linux", doc = "[example value: ~/.mozilla/firefox]")]
    #[cfg_attr(
        target_os = "macos",
        doc = "[example value: ~/Library/Application Support/Firefox]"
    )]
    #[cfg_attr(
        target_os = "windows",
        doc = r"[example value: ~\AppData\Roaming\Mozilla\Firefox]"
    )]
    pub base: Option<PathBuf>,

    #[arg(short('P'), id("profile"), value_hint(ValueHint::Other))]
    /// When browser is started with `-P <profile>       Start with <profile>.`
    pub profile: Option<String>,

    #[arg(long("profile"), id("path"))]
    #[arg(verbatim_doc_comment)]
    /// When browser is started with `--profile <path>   Start with profile at <path>.`
    /// When the arg is used, other args (such as `--base`, `-P`) are ignore.
    pub profile_path: Option<PathBuf>,

    #[arg(short, long, value_delimiter(','), action(ArgAction::Append))]
    /// Only support cookie
    pub values: Vec<Value>,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(clap::ValueEnum)]
#[clap(rename_all = "PascalCase")]
#[derive(strum::EnumIter)]
pub enum FirefoxName {
    Firefox,
    Librewolf,
    Floorp,
    Zen,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(clap::Args)]
pub struct BinaryCookiesArgs {
    #[arg(short('i'), long, value_hint(ValueHint::FilePath))]
    pub cookies_path: PathBuf,
    #[arg(short, long, value_hint(ValueHint::FilePath))]
    pub out_file: Option<PathBuf>,
}
