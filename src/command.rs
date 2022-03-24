use chrono::{Local, TimeZone};
use exif::{In, Reader, Tag};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;

pub fn info(args: Vec<String>) -> anyhow::Result<()> {
    let exifreader = Reader::new();
    for file_or_dir in args {
        let mut filelist = Vec::new();
        get_filelist(file_or_dir, &mut filelist);
        for f in filelist {
            let file = std::fs::File::open(&f).expect(&format!("Cannot open file: {}", f));
            let mut bufreader = std::io::BufReader::new(&file);
            let exif = exifreader.read_from_container(&mut bufreader);
            match exif {
                Err(err) => {
                    println!("filename: {}, err={}", f, err);
                    continue;
                }
                Ok(e) => {
                    println!("--- {}", f);
                    for field in e.fields() {
                        println!(
                            "{} {} {}",
                            field.tag,
                            field.ifd_num,
                            field.display_value().with_unit(&e),
                        );
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn exiflist(args: Vec<String>) -> anyhow::Result<()> {
    let mut exiflist: BTreeMap<String, i32> = BTreeMap::new();
    let exifreader = Reader::new();
    for file_or_dir in args {
        let mut filelist = Vec::new();
        get_filelist(file_or_dir, &mut filelist);
        for f in filelist {
            let file = std::fs::File::open(&f).expect(&format!("Cannot open file: {}", f));
            let mut bufreader = std::io::BufReader::new(&file);
            let exif = exifreader.read_from_container(&mut bufreader);
            match exif {
                Err(err) => {
                    println!("filename: {}, err={}", f, err);
                    continue;
                }
                Ok(e) => {
                    for f in e.fields() {
                        if f.ifd_num != In::PRIMARY {
                            continue;
                        }

                        let tag_string = f.tag.to_string();
                        match exiflist.get(&tag_string) {
                            Some(&count) => exiflist.insert(tag_string, count + 1),
                            _ => exiflist.insert(tag_string, 1),
                        };
                    }
                }
            }
        }
    }

    for i in exiflist {
        println!("{:?}", i);
    }
    Ok(())
}

pub async fn regist(args: Vec<String>) -> anyhow::Result<()> {
    let exifreader = Reader::new();
    for file_or_dir in args {
        let mut filelist = Vec::new();
        get_filelist(file_or_dir, &mut filelist);
        for f in filelist {
            let dandb = get_dirname_and_basename(&f);
            let mut create_image = crate::image::RegistImage {
                file_path: dandb.0,
                file_name: dandb.1,
                digitized_at: 0,
                props: HashMap::new(),
            };

            let file = std::fs::File::open(&f).expect(&format!("Cannot open file: {}", f));
            let mut bufreader = std::io::BufReader::new(&file);
            let exif = exifreader.read_from_container(&mut bufreader);
            if let Ok(e) = exif {
                insert_props_from_exif_field(&mut create_image.props, Tag::DateTimeOriginal, &e);
                insert_props_from_exif_field(&mut create_image.props, Tag::DateTimeDigitized, &e);
                insert_props_from_exif_field(&mut create_image.props, Tag::DateTime, &e);

                if let Some(v) = create_image.props.get(&Tag::DateTimeOriginal.to_string()) {
                    create_image.digitized_at = Local
                        .datetime_from_str(v, "%Y-%m-%d %H:%M:%S")
                        .unwrap()
                        .timestamp();
                }
            }

            let image = create_image.db_regist_image().await?;
            println!("{:?}", image);
            //println!("{}", image.props["DateTimeOriginal"]);
        }
    }

    Ok(())
}

pub fn get_filelist(file_or_dir: String, filelist: &mut Vec<String>) {
    let ignore_list = super::CONFIG
        .get()
        .unwrap()
        .get_array("IGNORE_PATH")
        .unwrap();
    let file_or_dir_basename = regex::Regex::new(r".*/").unwrap().replace(&file_or_dir, "");
    for i in ignore_list {
        if file_or_dir_basename == i.to_string() {
            return;
        }
    }
    let metadata = fs::metadata(&file_or_dir).unwrap();
    if metadata.is_dir() {
        for d in fs::read_dir(file_or_dir).unwrap() {
            let path = d.unwrap().path().to_string_lossy().into_owned();
            get_filelist(path, filelist);
        }
    } else {
        filelist.push(file_or_dir);
    }
}

fn insert_props_from_exif_field(
    props: &mut HashMap<String, String>,
    tag: exif::Tag,
    exif: &exif::Exif,
) {
    if let Some(field) = exif.get_field(tag, In::PRIMARY) {
        props.insert(tag.to_string(), field.display_value().to_string());
    }
}

fn get_dirname_and_basename(path: &String) -> (String, String) {
    let re = regex::Regex::new(r"(.*)/([^/]+)").unwrap();
    let caps = re.captures(path).unwrap();
    let r = (caps[1].to_string(), caps[2].to_string());
    r
}
