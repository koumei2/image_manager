use chrono::{DateTime, Local, TimeZone};
use exif::{In, Reader, Tag};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

static MP4_CREATION_TIME_KEY: &str = "creation_time";

pub fn info(target_path: PathBuf) -> anyhow::Result<()> {
    let exifreader = Reader::new();
    let mut filelist = Vec::new();
    get_filelist(target_path, &mut filelist);
    for f in filelist {
        let file = File::open(&f).expect(&format!("Cannot open file: {}", f.to_string_lossy()));
        let mut bufreader = BufReader::new(&file);
        if is_movie(&f) {
            println!(
                "--- movie filename: {},  creation time:  {}",
                f.to_string_lossy().into_owned(),
                get_mp4_creation_time(f, bufreader),
            );
        } else {
            let exif = exifreader.read_from_container(&mut bufreader);
            match exif {
                Err(err) => {
                    println!("filename: {}, err={}", f.to_string_lossy(), err);
                    continue;
                }
                Ok(e) => {
                    println!("--- {}", f.to_string_lossy());
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

pub fn exiflist(target_path: PathBuf) -> anyhow::Result<()> {
    let mut exiflist: BTreeMap<String, i32> = BTreeMap::new();
    let exifreader = Reader::new();
    let mut filelist = Vec::new();
    get_filelist(target_path, &mut filelist);
    for f in filelist {
        let file = File::open(&f).expect(&format!("Cannot open file: {}", f.to_string_lossy()));
        let mut bufreader = std::io::BufReader::new(&file);
        if is_movie(&f) {
            let _ = get_mp4_creation_time(f, bufreader);
            match exiflist.get(MP4_CREATION_TIME_KEY) {
                Some(&count) => exiflist.insert(MP4_CREATION_TIME_KEY.to_string(), count + 1),
                _ => exiflist.insert(MP4_CREATION_TIME_KEY.to_string(), 1),
            };
        } else {
            let exif = exifreader.read_from_container(&mut bufreader);
            match exif {
                Err(err) => {
                    println!("filename: {}, err={}", f.to_string_lossy(), err);
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

pub async fn regist(target_path: PathBuf) -> anyhow::Result<()> {
    let exifreader = Reader::new();
    let mut filelist = Vec::new();
    get_filelist(target_path, &mut filelist);
    for f in filelist {
        let mut create_image = crate::image::RegistImage {
            file_path: f.parent().unwrap().to_string_lossy().into_owned(),
            file_name: f.file_name().unwrap().to_string_lossy().into_owned(),
            digitized_at: 0,
            props: HashMap::new(),
        };

        let file = File::open(&f).expect(&format!("Cannot open file: {}", f.to_string_lossy()));
        let mut bufreader = std::io::BufReader::new(&file);
        if is_movie(&f) {
            create_image.digitized_at = get_mp4_creation_time(f, bufreader).timestamp();
        } else {
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
        }
        let image = create_image.db_regist_image().await?;
        println!("{:?}", image);
    }

    Ok(())
}

pub fn get_filelist(file_or_dir: std::path::PathBuf, filelist: &mut Vec<std::path::PathBuf>) {
    let ignore_list = super::CONFIG
        .get()
        .unwrap()
        .get_array("IGNORE_PATH")
        .unwrap();
    //let file_or_dir_basename = regex::Regex::new(r".*/").unwrap().replace(&file_or_dir, "");
    if let Some(file_or_dir_basename) = file_or_dir.file_name() {
        for i in ignore_list {
            if file_or_dir_basename == std::ffi::OsStr::new(&i.to_string()) {
                return;
            }
        }
    }
    let metadata = fs::metadata(&file_or_dir).unwrap();
    if metadata.is_dir() {
        for d in fs::read_dir(file_or_dir).unwrap() {
            //let path = d.unwrap().path().to_string_lossy().into_owned();
            let path = d.unwrap().path();
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

fn _get_dirname_and_basename(path: &String) -> (String, String) {
    let re = regex::Regex::new(r"(.*)/([^/]+)").unwrap();
    let caps = re.captures(path).unwrap();
    let r = (caps[1].to_string(), caps[2].to_string());
    r
}

fn get_mp4_creation_time(f: PathBuf, bufreader: BufReader<&File>) -> DateTime<Local> {
    let size = f.metadata().unwrap().len();
    let mp4 = mp4::Mp4Reader::read_header(bufreader, size).unwrap();
    let dt1 = Local.timestamp(
        mp4_creation_time_convert(mp4.moov.mvhd.creation_time)
            .try_into()
            .unwrap(),
        0,
    );
    dt1
}

fn mp4_creation_time_convert(creation_time: u64) -> u64 {
    // convert from MP4 epoch (1904-01-01) to Unix epoch (1970-01-01)
    if creation_time >= 2082844800 {
        creation_time - 2082844800
    } else {
        creation_time
    }
}

fn is_movie(f: &PathBuf) -> bool {
    let ex = f.extension().unwrap().to_string_lossy().into_owned();
    let movie_extension = super::CONFIG
        .get()
        .unwrap()
        .get_array("MOVIE_EXTENSION")
        .unwrap();
    movie_extension.iter().any(|x| x.to_string() == ex)
}
