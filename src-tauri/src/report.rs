use chrono::{Local, NaiveDate};
use csv::WriterBuilder;
use encoding_rs::{UTF_8, WINDOWS_1252};
use serde::Serialize;
use std::io::Write;
use std::path::Path;

use crate::config::{get_config, EffortType, MainConfig, ReportVersion, ReportEncoding};
use crate::toggl_api::TimeEntry;

const DATE_FORMAT: &str = "%d/%m/%Y";
const INPUT_DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%z";
const MAX_NAME_LENGTH: usize = 50;
const DELIMITER: u8 = b";"[0];

/// V1 (Legacy) format with HORAS column
#[derive(Serialize, Debug)]
pub struct SGUTimeEntryLegacy {
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

/// V2 format with ESFORCO and TIPO_ESFORCO columns
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

    #[serde(rename = "ESFORCO")]
    effort: String,

    #[serde(rename = "USERNAME")]
    username: String,

    #[serde(rename = "TIPO_ESFORCO")]
    effort_type: String,
}

/// Transform duration in seconds to hours with 2 decimal places, with comma as decimal separator
/// Used for v1 HORAS column and v2 HORAS effort type
fn process_duration(duration: i64) -> String {
    let total_seconds = duration.abs();
    let res = total_seconds as f64 / 3600.0;
    format!("{:.2}", res).replace(".", ",")
}

/// Transform duration in seconds to integer minutes (floor/truncate)
/// Uses absolute value for negative durations.
fn process_duration_minutos(duration: i64) -> String {
    let total_seconds = duration.abs();
    let minutes = total_seconds / 60; // integer division = floor
    format!("{}", minutes)
}

/// Transform duration in seconds to HH:MM format with colon (floor/truncate)
/// Uses absolute value for negative durations.
fn process_duration_hhmm(duration: i64) -> String {
    let total_seconds = duration.abs();
    let total_minutes = total_seconds / 60; // integer division = floor
    let hours = total_minutes / 60;
    let minutes = total_minutes % 60;
    format!("{:02}:{:02}", hours, minutes)
}

/// Transform project name to SGU format
/// Crop name to MAX_NAME_LENGTH
fn process_name(name: &str) -> String {
    let mut name = name.to_string();

    if name.chars().count() > MAX_NAME_LENGTH {
        name = truncate(&name, MAX_NAME_LENGTH).to_string();
    }

    name
}

/// Truncate string to max_chars. Custom implementation to avoid panics with non-ASCII characters
fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

/// Transform classes to SGU format
/// Select only first class
fn process_classes(classes: &[String], cfg: &MainConfig) -> String {
    match classes.first() {
        Some(class) => class.to_string(),
        None => cfg.default_tag.clone().unwrap_or_default(),
    }
}

/// Process date from input format to output format
fn process_date(start: &str) -> String {
    NaiveDate::parse_from_str(start, INPUT_DATE_FORMAT)
        .expect("Could not find start date")
        .format(DATE_FORMAT)
        .to_string()
}

/// Check if tags vector contains IGNORED_TAG
fn is_ignored(tags: &[String], cfg: &MainConfig) -> bool {
    if let Some(ref ignore_tag) = cfg.ignore_tag {
        for tag in tags {
            if tag == ignore_tag {
                return true;
            }
        }
    }
    false
}

// ============================================================================
// V1 (Legacy) Functions
// ============================================================================

/// Transform time entry to SGU legacy format (v1)
fn transform_time_entry_legacy(entry: TimeEntry, cfg: &MainConfig) -> Option<SGUTimeEntryLegacy> {
    if is_ignored(&entry.tags, cfg) {
        return None;
    }

    Some(SGUTimeEntryLegacy {
        date: process_date(&entry.start),
        project: entry.project_name.unwrap_or_default(),
        class: process_classes(&entry.tags, cfg),
        name: entry.description.map(|d| process_name(&d)).unwrap_or_default(),
        card_key: "".to_string(),
        duration: process_duration(entry.duration),
        username: cfg.sgu_name.clone(),
    })
}

/// Transform multiple time entries to SGU legacy format (v1)
fn transform_multiple_time_entries_legacy(
    entries: Vec<TimeEntry>,
    cfg: &MainConfig,
) -> Vec<SGUTimeEntryLegacy> {
    let mut sgu_entries: Vec<SGUTimeEntryLegacy> = Vec::new();

    for entry in entries {
        if let Some(sgu_entry) = transform_time_entry_legacy(entry, cfg) {
            sgu_entries.push(sgu_entry);
        }
    }

    sgu_entries
}

/// Export legacy entries as CSV (v1)
fn export_as_csv_legacy(entries: Vec<SGUTimeEntryLegacy>, cfg: &MainConfig) -> String {
    let now = Local::now().format("%Y%m%d").to_string();
    let filename = now + "_sgu.csv";

    let output_file = Path::new(cfg.output_file_dir.as_str()).join(filename);

    let mut csv_writer = WriterBuilder::new()
        .delimiter(DELIMITER)
        .from_writer(vec![]);

    for entry in entries {
        let _ = csv_writer.serialize(entry);
    }

    let contents = csv_writer.into_inner().unwrap();

    // Change encoding based on user selection
    let mut file = std::fs::File::create(&output_file).unwrap();
    let source = String::from_utf8(contents).unwrap();
    
    let encoded_contents = match cfg.report_encoding {
        ReportEncoding::Utf8 => {
            let (encoded, _, _) = UTF_8.encode(&source);
            encoded
        }
        ReportEncoding::Windows1252 => {
            let (encoded, _, _) = WINDOWS_1252.encode(&source);
            encoded
        }
    };

    file.write_all(&encoded_contents).unwrap();

    output_file.to_str().unwrap().to_string()
}

// ============================================================================
// V2 (New) Functions
// ============================================================================

/// Transform time entry to SGU v2 format
fn transform_time_entry_v2(
    entry: TimeEntry,
    effort_type: &EffortType,
    cfg: &MainConfig,
) -> Option<SGUTimeEntry> {
    if is_ignored(&entry.tags, cfg) {
        return None;
    }

    let (effort_str, effort_type_str) = match effort_type {
        EffortType::Horas => (process_duration(entry.duration), "HORAS".to_string()),
        EffortType::Minutos => (process_duration_minutos(entry.duration), "MINUTOS".to_string()),
        EffortType::Hhmm => (process_duration_hhmm(entry.duration), "HHMM".to_string()),
    };

    Some(SGUTimeEntry {
        date: process_date(&entry.start),
        project: entry.project_name.unwrap_or_default(),
        class: process_classes(&entry.tags, cfg),
        name: entry.description.map(|d| process_name(&d)).unwrap_or_default(),
        card_key: "".to_string(),
        effort: effort_str,
        username: cfg.sgu_name.clone(),
        effort_type: effort_type_str,
    })
}

/// Transform multiple time entries to SGU v2 format
fn transform_multiple_time_entries_v2(
    entries: Vec<TimeEntry>,
    effort_type: &EffortType,
    cfg: &MainConfig,
) -> Vec<SGUTimeEntry> {
    let mut sgu_entries: Vec<SGUTimeEntry> = Vec::new();

    for entry in entries {
        if let Some(sgu_entry) = transform_time_entry_v2(entry, effort_type, cfg) {
            sgu_entries.push(sgu_entry);
        }
    }

    sgu_entries
}

/// Export v2 entries as CSV
fn export_as_csv_v2(entries: Vec<SGUTimeEntry>, cfg: &MainConfig) -> String {
    let now = Local::now().format("%Y%m%d").to_string();
    let filename = now + "_sgu.csv";

    let output_file = Path::new(cfg.output_file_dir.as_str()).join(filename);

    let mut csv_writer = WriterBuilder::new()
        .delimiter(DELIMITER)
        .from_writer(vec![]);

    for entry in entries {
        let _ = csv_writer.serialize(entry);
    }

    let contents = csv_writer.into_inner().unwrap();

    // Change encoding based on user selection
    let mut file = std::fs::File::create(&output_file).unwrap();
    let source = String::from_utf8(contents).unwrap();
    
    let encoded_contents = match cfg.report_encoding {
        ReportEncoding::Utf8 => {
            let (encoded, _, _) = UTF_8.encode(&source);
            encoded
        }
        ReportEncoding::Windows1252 => {
            let (encoded, _, _) = WINDOWS_1252.encode(&source);
            encoded
        }
    };

    file.write_all(&encoded_contents).unwrap();

    output_file.to_str().unwrap().to_string()
}

// ============================================================================
// Public API
// ============================================================================

/// Generate report based on configuration settings.
/// Branches between v1 (legacy) and v2 (new) formats based on report_version.
pub fn generate_report(entries: Vec<TimeEntry>) -> String {
    let cfg = get_config().unwrap();

    match cfg.report_version {
        ReportVersion::V1 => {
            let sgu_entries = transform_multiple_time_entries_legacy(entries, &cfg);
            export_as_csv_legacy(sgu_entries, &cfg)
        }
        ReportVersion::V2 => {
            let sgu_entries = transform_multiple_time_entries_v2(entries, &cfg.effort_type, &cfg);
            export_as_csv_v2(sgu_entries, &cfg)
        }
    }
}
