use clap::{Parser, Subcommand};
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
        path_list: Vec<String>,
    },
    Exiflist {
        #[clap(required = true)]
        path_list: Vec<String>,
    },
    Regist {
        #[clap(required = true)]
        path_list: Vec<String>,
    },
}
