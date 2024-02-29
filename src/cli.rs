//! Module contains commandline interface

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Treat undefined variables as errors
    #[arg(short, long)]
    pub strict: bool,

    /// Print information about template
    #[arg(short, long)]
    pub info: bool,

    /// Run following templates on same context before running the main template
    #[arg(long, conflicts_with = "info")]
    pub include: Vec<String>,

    /// Path to output the file at, if not set then prints to stdout
    #[arg(short, long, conflicts_with = "info")]
    pub outfile: Option<String>,

    /// Path to template file
    pub template: String,

    /// Args to pass to template (e.g. color1=#ffffff)
    #[arg(last = true)]
    pub args: Vec<String>,
}

