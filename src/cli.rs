//! Module contains commandline interface

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Treat undefined variables as errors
    #[arg(short, long)]
    pub strict: bool,

    /// Extract original template from file (will fail if there is nothing)
    #[arg(short, long)]
    pub extract: bool,

    /// Run following templates on same context before running the main template
    #[arg(long, conflicts_with = "extract")]
    pub include: Vec<String>,

    /// Path to output the file at, if not set then prints to stdout
    #[arg(short, long, conflicts_with = "extract")]
    pub outfile: Option<String>,

    /// Path to template file
    pub template: String,

    /// Args to pass to template (e.g. color1=#ffffff)
    #[arg(last = true)]
    pub args: Vec<String>,
}

