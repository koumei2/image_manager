use chrono::{DateTime, Local, TimeZone};
use exif::{In, Reader, Tag};
use rexif::ExifData;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

static MOVIE_CREATION_TIME_KEY: &str = "creation_time";
static CONF_MOVIE_EXTENTION: &str = "MOVIE_EXTENSION";
static CONF_IGNORE_EXTENTION: &str = "EXIF_IGNORE_EXTENSION";

static PROPS_KEY_DATETIME_ORIGINAL: &str = "Date of original image";
static PROPS_KEY_DATETIME_DIGITIZED: &str = "Date of image digitalization";
static PROPS_KEY_DATETIME: &str = "Image date";

pub fn exifinfo(filename: PathBuf) -> anyhow::Result<()> {
    let file =
        File::open(&filename).expect(&format!("Cannot open file: {}", filename.to_string_lossy()));
    let mut bufreader = BufReader::new(&file);
    let exifreader = exif::Reader::new();
    let exif = exifreader.read_from_container(&mut bufreader)?;
    for f in exif.fields() {
        println!(
            "{} {} {}",
            f.tag,
            f.ifd_num,
            f.display_value().with_unit(&exif)
        );
    }
    Ok(())
}

pub fn info(target_path: PathBuf) -> anyhow::Result<()> {
    let mut filelist = Vec::new();
    get_filelist(target_path, &mut filelist);
    for f in filelist {
        let file = File::open(&f).expect(&format!("Cannot open file: {}", f.to_string_lossy()));
        let bufreader = BufReader::new(&file);
        if is_movie(&f) {
            println!(
                "--- movie filename: {},  creation time:  {}",
                f.to_string_lossy().into_owned(),
                Local.timestamp(get_mp4_creation_time(&f, bufreader), 0)
            );
        } else {
            match rexif::parse_file(&f) {
                Ok(exif) => {
                    println!("filename: {} ", f.to_string_lossy().into_owned());
                    for entry in &exif.entries {
                        println!("\t{}: {}", entry.tag, entry.value_more_readable);
                    }
                }
                Err(err) => {
                    println!("filename: {}, err={}", f.to_string_lossy(), err);
                }
            }
        }
    }
    Ok(())
}

// kamadak-exif ?????????????????????????????????????????????
// ??????????????????????????????????????????????????????
// 0.5.4 -> NG
pub fn exiflist(target_path: PathBuf) -> anyhow::Result<()> {
    let mut exiflist: BTreeMap<String, i32> = BTreeMap::new();
    let mut filelist = Vec::new();
    get_filelist(target_path, &mut filelist);
    for f in &filelist {
        let file = File::open(&f).expect(&format!("Cannot open file: {}", f.to_string_lossy()));
        let mut bufreader = std::io::BufReader::new(&file);
        if is_movie(&f) {
            let _ = get_mp4_creation_time(f, bufreader);
            match exiflist.get(MOVIE_CREATION_TIME_KEY) {
                Some(&count) => exiflist.insert(MOVIE_CREATION_TIME_KEY.to_string(), count + 1),
                _ => exiflist.insert(MOVIE_CREATION_TIME_KEY.to_string(), 1),
            };
        } else {
            let exifreader = Reader::new();
            let exif = exifreader.read_from_container(&mut bufreader);
            match exif {
                Err(err) => {
                    println!("filename: {}, err={}", f.to_string_lossy(), err);
                    /*let _: DateTime<Local> = f.metadata().unwrap().modified().unwrap().into();
                    match exiflist.get(FILE_MTIME_KEY) {
                        Some(&count) => exiflist.insert(FILE_MTIME_KEY.to_string(), count + 1),
                        _ => exiflist.insert(MP4_CREATION_TIME_KEY.to_string(), 1),
                    };*/
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

    println!("Total file = {}", filelist.len());
    for i in exiflist {
        println!("{:?}", i);
    }
    Ok(())
}

pub fn exiflist2(target_path: PathBuf) -> anyhow::Result<()> {
    let mut exiflist: BTreeMap<String, i32> = BTreeMap::new();
    let mut filelist = Vec::new();
    let mut ignore_file = Vec::new();
    get_filelist(target_path, &mut filelist);
    for f in &filelist {
        let file = File::open(&f).expect(&format!("Cannot open file: {}", f.to_string_lossy()));
        let mut bufreader = std::io::BufReader::new(&file);
        if is_movie(&f) {
            let _ = get_mp4_creation_time(f, bufreader);
            match exiflist.get(MOVIE_CREATION_TIME_KEY) {
                Some(&count) => exiflist.insert(MOVIE_CREATION_TIME_KEY.to_string(), count + 1),
                _ => exiflist.insert(MOVIE_CREATION_TIME_KEY.to_string(), 1),
            };
        } else if is_png(&f) {
            let exifreader = Reader::new();
            let exif = exifreader.read_from_container(&mut bufreader);
            match exif {
                Err(err) => {
                    println!("filename: {}, type=PNG, err={}", f.to_string_lossy(), err);
                    /*let _: DateTime<Local> = f.metadata().unwrap().modified().unwrap().into();
                    match exiflist.get(FILE_MTIME_KEY) {
                        Some(&count) => exiflist.insert(FILE_MTIME_KEY.to_string(), count + 1),
                        _ => exiflist.insert(MP4_CREATION_TIME_KEY.to_string(), 1),
                    };*/
                }
                Ok(e) => {
                    println!("filename: {}, type=PNGOK", f.to_string_lossy());
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
        } else if is_ignore(&f) {
            ignore_file.push(f.to_string_lossy());
        } else {
            match rexif::parse_file(&f) {
                Ok(exif) => {
                    for entry in &exif.entries {
                        let tag_string = entry.tag.to_string();
                        match exiflist.get(&tag_string) {
                            Some(&count) => exiflist.insert(tag_string, count + 1),
                            _ => exiflist.insert(tag_string, 1),
                        };
                    }
                }
                Err(err) => {
                    println!("filename: {}, type=rexif, err={}", f.to_string_lossy(), err);
                }
            }
        }
    }

    println!("Total file = {}", filelist.len());
    for i in ignore_file {
        println!("Ignore file = {}", i);
    }
    for i in exiflist {
        println!("{:?}", i);
    }
    Ok(())
}

pub async fn regist(target_path: PathBuf, dryrun: bool) -> anyhow::Result<()> {
    let mut filelist = Vec::new();
    get_filelist(target_path, &mut filelist);
    for f in filelist {
        //println!("{}", f.to_string_lossy());
        let mut create_image = crate::image::RegistImage {
            file_path: f.parent().unwrap().to_string_lossy().into_owned(),
            file_name: f.file_name().unwrap().to_string_lossy().into_owned(),
            digitized_at: 0,
            props: HashMap::new(),
        };

        let file = File::open(&f).expect(&format!("Cannot open file: {}", f.to_string_lossy()));
        let mut bufreader = std::io::BufReader::new(&file);
        if is_movie(&f) {
            create_image.digitized_at = get_mp4_creation_time(&f, bufreader);
            create_image.props.insert(
                MOVIE_CREATION_TIME_KEY.to_string(),
                create_image.digitized_at.to_string(),
            );
        } else if is_png(&f) {
            let exifreader = Reader::new();
            let exif = exifreader.read_from_container(&mut bufreader);
            match exif {
                Err(_) => {
                    create_image.digitized_at = get_file_modified_time(f);
                }
                Ok(e) => {
                    insert_props_from_exif_field(
                        &mut create_image.props,
                        Tag::DateTimeOriginal,
                        &e,
                    );
                    insert_props_from_exif_field(
                        &mut create_image.props,
                        Tag::DateTimeDigitized,
                        &e,
                    );
                    insert_props_from_exif_field(&mut create_image.props, Tag::DateTime, &e);
                    if let Some(v) = create_image.props.get(&Tag::DateTimeOriginal.to_string()) {
                        create_image.digitized_at = Local
                            .datetime_from_str(v, "%Y-%m-%d %H:%M:%S")
                            .unwrap()
                            .timestamp();
                    }
                }
            }
        } else if is_ignore(&f) {
            println!("skip file = {}", f.to_string_lossy());
        } else {
            match rexif::parse_file(&f) {
                Ok(exif) => {
                    insert_props_from_exif_field_by_rexif(
                        &mut create_image.props,
                        PROPS_KEY_DATETIME_DIGITIZED,
                        &exif,
                    );
                    insert_props_from_exif_field_by_rexif(
                        &mut create_image.props,
                        PROPS_KEY_DATETIME_ORIGINAL,
                        &exif,
                    );
                    insert_props_from_exif_field_by_rexif(
                        &mut create_image.props,
                        PROPS_KEY_DATETIME,
                        &exif,
                    );
                    create_image.digitized_at =
                        match create_image.props.get(PROPS_KEY_DATETIME_DIGITIZED) {
                            Some(v) => Local
                                .datetime_from_str(v, "%Y:%m:%d %H:%M:%S")
                                .unwrap()
                                .timestamp(),
                            None => get_file_modified_time(f),
                        };
                }
                Err(_) => {
                    create_image.digitized_at = get_file_modified_time(f);
                }
            }
        }

        if dryrun {
            println!("[dryrun] {:?}", create_image);
        } else {
            let image = create_image.db_regist_image().await?;
            println!("[regist] {:?}", image);
        }
    }

    Ok(())
}

fn get_file_modified_time(f: std::path::PathBuf) -> i64 {
    let modified: DateTime<Local> = f.metadata().unwrap().modified().unwrap().into();
    modified.timestamp()
}

fn get_filelist(file_or_dir: std::path::PathBuf, filelist: &mut Vec<std::path::PathBuf>) {
    let ignore_list = super::CONFIG
        .get()
        .unwrap()
        .get_array("IGNORE_PATH")
        .unwrap();
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

fn insert_props_from_exif_field_by_rexif(
    props: &mut HashMap<String, String>,
    tag: &str,
    exif: &ExifData,
) {
    if let Some(v) = &exif.entries.iter().find(|e| e.tag.to_string() == tag) {
        if v.value != rexif::TagValue::Ascii("0000:00:00 00:00:00".to_string()) {
            props.insert(tag.to_string(), v.value.to_string());
        }
    }
}

fn get_mp4_creation_time(f: &PathBuf, bufreader: BufReader<&File>) -> i64 {
    let size = f.metadata().unwrap().len();
    let mp4 = mp4::Mp4Reader::read_header(bufreader, size).unwrap();
    mp4_creation_time_convert(mp4.moov.mvhd.creation_time)
        .try_into()
        .unwrap()
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
        .get_array(CONF_MOVIE_EXTENTION)
        .unwrap();
    movie_extension.iter().any(|x| x.to_string() == ex)
}

fn is_png(f: &PathBuf) -> bool {
    let ex = f.extension().unwrap().to_string_lossy().into_owned();
    match ex.as_str() {
        "png" => true,
        "PNG" => true,
        _ => false,
    }
}

fn is_ignore(f: &PathBuf) -> bool {
    let ex = f.extension().unwrap().to_string_lossy().into_owned();
    let extension = super::CONFIG
        .get()
        .unwrap()
        .get_array(CONF_IGNORE_EXTENTION)
        .unwrap();
    extension.iter().any(|x| x.to_string() == ex)
}
