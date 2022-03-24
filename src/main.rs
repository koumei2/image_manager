use clap::Parser;
use once_cell::sync::OnceCell;

mod args;
mod command;
mod db;
mod image;

static CONFIG: OnceCell<config::Config> = OnceCell::new();
static DB: OnceCell<sqlx::Pool<sqlx::sqlite::Sqlite>> = OnceCell::new();

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = args::Args::parse();

    let config = config::Config::builder()
        .add_source(config::File::with_name("config.json"))
        .build()
        .unwrap();
    let _ = CONFIG.set(config);

    match args.command {
        args::Command::Info { path_list } => command::info(path_list)?,
        args::Command::Exiflist { path_list } => command::exiflist(path_list)?,
        args::Command::Regist { path_list } => {
            db::init().await?;
            command::regist(path_list).await?;
        }
    };
    Ok(())
}
