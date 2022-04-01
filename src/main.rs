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
        args::Command::Info { target_path } => command::info(target_path)?,
        args::Command::Exiflist { target_path } => command::exiflist(target_path)?,
        args::Command::Exiflist2 { target_path } => command::exiflist2(target_path)?,
        args::Command::Regist { target_path } => {
            db::init().await?;
            command::regist(target_path).await?;
        }
    };
    Ok(())
}
