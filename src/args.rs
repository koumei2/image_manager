use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[clap(arg_required_else_help = true)]
    Info {
        #[clap(required = true)]
        target_path: PathBuf,
    },
    Exiflist {
        #[clap(required = true)]
        target_path: PathBuf,
    },
    Regist {
        #[clap(required = true)]
        target_path: PathBuf,
    },
}
