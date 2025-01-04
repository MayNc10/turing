use std::path::PathBuf;

use clap::{ArgGroup, Parser};


#[derive(Parser)]
#[command(version, about, long_about = None)]
#[clap(group(
    ArgGroup::new("source")
        .required(true)
        .args(&["input", "file"]),
))]
pub struct Cli {
    #[clap(short, long)]
    pub file: Option<PathBuf>,

    #[clap(short, long)]
    pub input: bool,

    #[clap(short, long, conflicts_with = "file")]
    pub output: Option<String>,
}