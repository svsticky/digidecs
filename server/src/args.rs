use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Clone, Parser)]
pub struct AppArgs {
    #[clap(long, short)]
    pub config: PathBuf,
    #[clap(long)]
    pub dry_run: bool,
}
