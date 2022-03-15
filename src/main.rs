use clap::Parser;
use std::collections::HashMap;
use std::default::Default;

mod args;
mod command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args::Args::parse();

    let settings = config::Config::builder()
        .add_source(config::File::with_name("config.json"))
        .build()
        .unwrap();

    println!("{:?}", settings.get_string("DATABASE_URL"));
    if let Ok(a) = settings.get_array("IGNORE_PATH") {
        for i in a {
            println!("{}", i);
        }
    }

    match args.command {
        args::Command::Info { path_list } => command::info(path_list)?,
        args::Command::Exiflist { path_list } => command::exiflist(path_list)?,
        args::Command::Regist { path_list } => command::regist(path_list)?,
    };
    Ok(())
}

fn db_regist_image(image: Image) -> Image {
    dotenv::dotenv().ok();
    let conn_str = dotenv::var("DATABASE_URL").expect("Env var DATABASE_URL is required.");
    image
}

fn get_dirname_and_basename(path: &String) -> (String, String) {
    let re = regex::Regex::new(r"(.*)/([^/]+)").unwrap();
    let caps = re.captures(path).unwrap();
    let r = (caps[1].to_string(), caps[2].to_string());
    r
}

#[derive(Debug, Default)]
struct Image {
    id: i64,
    file_path: String,
    file_name: String,
    digitized_at: i64,
    props: HashMap<String, String>,
    created_at: i64,
    updated_at: i64,
}

/*
CREATE TABLE images (
    id          BIGSERIAL   NOT NULL PRIMARY KEY,
    file_path   text        NOT NULL,
    file_name   text        NOT NULL,
    digitized_at TIMESTAMP,
    props       jsonb,
    created_at  TIMESTAMP   NOT NULL default CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP   NOT NULL default CURRENT_TIMESTAMP,
    UNIQUE(file_path, file_name)
);*/
