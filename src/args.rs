use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(
    name = "Image Manager",
    author = "ssasaki",
    version = "v0.1",
    about = "Image manage command tool"
)]

pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    #[clap(arg_required_else_help = true)]
    Exifinfo {
        #[clap(required = true)]
        target_path: PathBuf,
    },
    Info {
        #[clap(required = true)]
        target_path: PathBuf,
    },
    Exiflist {
        #[clap(required = true)]
        target_path: PathBuf,
    },
    Exiflist2 {
        #[clap(required = true)]
        target_path: PathBuf,
    },
    Regist {
        #[clap(short, long)]
        dryrun: bool,

        #[clap(required = true)]
        target_path: PathBuf,
    },
}
