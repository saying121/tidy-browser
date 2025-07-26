use std::path::PathBuf;

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    pub core: Core,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(clap::Subcommand)]
pub enum Core {
    /// Chromium based
    Chromium(ChromiumArgs),
    /// Firefox based
    Firefox(FirefoxArgs),
    #[cfg(target_os = "macos")]
    /// Safari
    Safari(SafariArgs),
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(clap::ValueEnum)]
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
    #[arg(short, long, value_delimiter(','))]
    /// Only support cookie
    pub values: Vec<Value>,
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(clap::Args)]
pub struct ChromiumArgs {
    #[arg(short, long, value_delimiter(','))]
    pub name: Vec<ChromiumName>,

    #[arg(long, id("DIR"))]
    #[arg(verbatim_doc_comment)]
    /// When browser is started with `--user-data-dir=DIR   Specify the directory that user data (your "profile") is kept in.`
    #[cfg_attr(target_os = "linux", doc = "[default value: ~/.config/google-chrome]")]
    #[cfg_attr(
        target_os = "macos",
        doc = "[default value: ~/Library/Application Support/Google/Chrome]"
    )]
    #[cfg_attr(
        target_os = "windows",
        doc = r"[default value: ~\AppData\Local\Google\Chrome\User Data]"
    )]
    pub user_data_dir: Option<PathBuf>,

    #[arg(long)]
    /// Filter by host/domain
    pub host: Option<String>,

    #[arg(short, long)]
    /// All browsers data
    pub all_browsers: bool,

    #[arg(short, long, value_delimiter(','))]
    pub values: Vec<Value>,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(clap::ValueEnum)]
#[clap(rename_all = "PascalCase")]
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
    #[arg(short, long, value_delimiter(','))]
    pub name: Vec<FirefoxName>,

    #[arg(long, id("DIR"))]
    /// Browser data dir.
    #[cfg_attr(target_os = "linux", doc = "[possible value: ~/.mozilla/firefox]")]
    #[cfg_attr(
        target_os = "macos",
        doc = "[possible value: ~/Library/Application Support/Firefox]"
    )]
    #[cfg_attr(
        target_os = "windows",
        doc = r"[possible value: ~\AppData\Roaming\Mozilla\Firefox]"
    )]
    pub base: Option<PathBuf>,

    #[arg(short('P'), id("profile"))]
    /// When browser is started with `-P <profile>       Start with <profile>.`
    pub profile: Option<String>,

    #[arg(long("profile"), id("path"))]
    #[arg(verbatim_doc_comment)]
    /// When browser is started with `--profile <path>   Start with profile at <path>.`
    /// When the arg is used, other args (such as `--base`, `-P`) are ignore.
    pub profile_path: Option<PathBuf>,

    #[arg(long)]
    /// Filter by host/domain
    pub host: Option<String>,
    #[arg(short, long)]
    /// All browsers data
    pub all_browsers: bool,

    #[arg(short, long, value_delimiter(','))]
    /// Only support cookie
    pub values: Vec<Value>,
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(clap::ValueEnum)]
#[clap(rename_all = "PascalCase")]
pub enum FirefoxName {
    Firefox,
    Librewolf,
    Floorp,
}
