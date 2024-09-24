use chrono::{Local, NaiveDate};
use csv::WriterBuilder;
use encoding_rs::WINDOWS_1252;
use serde::Serialize;
use std::io::Write;
use std::path::Path;

use crate::config::get_config;
use crate::toggl_api::TimeEntry;

const DATE_FORMAT: &str = "%d/%m/%Y";
const INPUT_DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%z";
const MAX_NAME_LENGTH: usize = 50;
const DELIMITER: u8 = b";"[0];

#[derive(Serialize, Debug)]
pub struct SGUTimeEntry {
    #[serde(rename = "DATA")]
    date: String,

    #[serde(rename = "PROJETO")]
    project: String,

    #[serde(rename = "CATEGORIA")]
    class: String,

    #[serde(rename = "ATIVIDADE")]
    name: String,

    #[serde(rename = "CARD_KEY")]
    card_key: String,

    #[serde(rename = "HORAS")]
    duration: String,

    #[serde(rename = "USERNAME")]
    username: String,
}

/// Transform duration in seconds to hours with 2 decimal places, with comma as decimal separator
fn process_duration(duration: i64) -> String {
    let res = duration as f64 / 3600.0;
    format!("{:.2}", res).replace(".", ",")
}

/// Transform project name to SGU format
/// Crop name to MAX_NAME_LENGTH
fn process_name(name: &str) -> String {
    let mut name = name.to_string();

    if name.chars().count() > MAX_NAME_LENGTH {
        name.truncate(MAX_NAME_LENGTH);
    }

    name
}

/// Transfort classes to SGU format
/// Select only first class
fn process_classes(classes: Vec<String>) -> String {
    match classes.first() {
        Some(class) => class.to_string(),
        None => "".to_string(),
    }
}

/// Check if tags vector containg IGNORED_TAG
fn is_ignored(tags: &Vec<String>) -> bool {
    let cfg = get_config().unwrap();
    let ignore_tag = cfg.ignore_tag.unwrap();
    for tag in tags {
        match tag {
            _ if tag == &ignore_tag => return true,
            _ => continue,
        }
    }
    false
}

/// Transform time entry to SGU format
fn transform_time_entry(entry: TimeEntry) -> Option<SGUTimeEntry> {
    let cfg = get_config().unwrap();

    if is_ignored(&entry.tags) {
        return None;
    }

    Some(SGUTimeEntry {
        date: NaiveDate::parse_from_str(&entry.start, INPUT_DATE_FORMAT)
            .expect("Could not find start date")
            .format(DATE_FORMAT)
            .to_string(),
        project: match entry.project_name {
            Some(project_name) => project_name,
            None => "".to_string(),
        },
        class: process_classes(entry.tags),
        name: match entry.description {
            Some(desc) => process_name(&desc),
            None => "".to_string(),
        },
        card_key: "".to_string(),
        duration: process_duration(entry.duration),
        username: cfg.sgu_name,
    })
}

pub fn transform_multiple_time_entries(entries: Vec<TimeEntry>) -> Vec<SGUTimeEntry> {
    let mut sgu_entries: Vec<SGUTimeEntry> = Vec::new();

    for entry in entries {
        match transform_time_entry(entry) {
            Some(entry) => sgu_entries.push(entry),
            None => continue,
        };
    }

    sgu_entries
}

/// TODO: Implement encoding change to ANSI
pub fn export_as_csv(entries: Vec<SGUTimeEntry>) -> String {
    let cfg = get_config().unwrap();
    let now = Local::now().format("%Y%m%d").to_string();
    let filename = now + "_sgu.csv";

    let output_file = Path::new(cfg.output_file_dir.as_str()).join(filename);

    let mut csv_writer = WriterBuilder::new()
        .delimiter(DELIMITER)
        // .from_path(&output_file)
        // .unwrap();
        .from_writer(vec![]);

    for entry in entries {
        let _ = csv_writer.serialize(entry);
    }

    let contents = csv_writer.into_inner().unwrap();

    // Change encoding to ANSI
    let mut file = std::fs::File::create(&output_file).unwrap();
    let source = String::from_utf8(contents).unwrap();
    let (encoded_contents, _, _) = WINDOWS_1252.encode(&source);

    file.write_all(&encoded_contents).unwrap();

    output_file.to_str().unwrap().to_string()
}
