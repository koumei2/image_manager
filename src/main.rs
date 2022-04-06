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
        args::Command::Exifinfo { target_path } => command::exifinfo(target_path)?,
        args::Command::Info { target_path } => command::info(target_path)?,
        args::Command::Exiflist { target_path } => command::exiflist(target_path)?,
        args::Command::Exiflist2 { target_path } => command::exiflist2(target_path)?,
        args::Command::Regist {
            dryrun,
            target_path,
        } => {
            db::init().await?;
            command::regist(target_path, dryrun).await?;
        }
    };
    Ok(())
}

// unused
fn _get_dirname_and_basename(path: &String) -> (String, String) {
    let re = regex::Regex::new(r"(.*)/([^/]+)").unwrap();
    let caps = re.captures(path).unwrap();
    let r = (caps[1].to_string(), caps[2].to_string());
    r
}
