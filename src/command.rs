use chrono::{Local, TimeZone};
use exif::{In, Reader, Tag};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;

pub fn info(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn exiflist(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn regist(args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    let exifreader = Reader::new();
    for file_or_dir in args {
        let mut filelist = Vec::new();
        get_filelist(file_or_dir, &mut filelist);
        for f in filelist {
            let mut image = super::Image::default();
            let dandb = super::get_dirname_and_basename(&f);
            image.file_path = dandb.0;
            image.file_name = dandb.1;
            let file = std::fs::File::open(&f).expect(&format!("Cannot open file: {}", f));
            let mut bufreader = std::io::BufReader::new(&file);
            let exif = exifreader.read_from_container(&mut bufreader);
            if let Ok(e) = exif {
                let mut props = HashMap::new();

                insert_props_from_exif_field(&mut props, Tag::DateTimeOriginal, &e);
                insert_props_from_exif_field(&mut props, Tag::DateTimeDigitized, &e);
                insert_props_from_exif_field(&mut props, Tag::DateTime, &e);

                if let Some(v) = props.get(&Tag::DateTimeOriginal.to_string()) {
                    image.digitized_at = Local
                        .datetime_from_str(v, "%Y-%m-%d %H:%M:%S")
                        .unwrap()
                        .timestamp();
                }

                image.props = props;
            }

            println!("{:?}", image);
            let image = super::db_regist_image(image);
            println!("{:?}", image);
        }
    }

    Ok(())
}

pub fn get_filelist(file_or_dir: String, filelist: &mut Vec<String>) {
    let ignore_list = [""];
    //let ignore_list = [".DS_Store"];
    let file_or_dir_basename = regex::Regex::new(r".*/").unwrap().replace(&file_or_dir, "");
    for i in ignore_list {
        if file_or_dir_basename == i {
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
