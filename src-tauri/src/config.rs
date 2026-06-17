extern crate confy;
use directories::UserDirs;
use serde::{Deserialize, Serialize};

const APP_NAME: &str = "toggl_reports_config";
const LAST_USED_OPTIONS_CONFIG_NAME: &str = "last_used_options";

/// Report version enum for selecting between legacy (v1) and new (v2) CSV formats
#[derive(Serialize, Deserialize, Default, Clone, Copy, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ReportVersion {
    V1,
    #[default]
    V2,
}

/// Effort type enum for v2 reports - determines how duration is formatted
#[derive(Serialize, Deserialize, Default, Clone, Copy, PartialEq, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum EffortType {
    Horas,
    Minutos,
    #[default]
    Hhmm,
}

/// Report encoding enum - determines the character encoding for the CSV output
#[derive(Serialize, Deserialize, Default, Clone, Copy, PartialEq, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum ReportEncoding {
    #[default]
    Windows1252,
    Utf8,
}

/// Helper function for serde default
fn default_report_version() -> ReportVersion {
    ReportVersion::default()
}

/// Helper function for serde default
fn default_effort_type() -> EffortType {
    EffortType::default()
}

/// Helper function for serde default
fn default_report_encoding() -> ReportEncoding {
    ReportEncoding::default()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MainConfig {
    pub api_token: String,
    pub workspace_filter: Option<u32>,
    pub output_file_dir: String,
    pub sgu_name: String,
    pub default_tag: Option<String>,
    pub ignore_tag: Option<String>,
    #[serde(default = "default_report_version")]
    pub report_version: ReportVersion,
    #[serde(default = "default_effort_type")]
    pub effort_type: EffortType,
    #[serde(default = "default_report_encoding")]
    pub report_encoding: ReportEncoding,
}
impl Default for MainConfig {
    fn default() -> Self {
        MainConfig {
            api_token: "".to_string(),
            workspace_filter: None,
            output_file_dir: UserDirs::new()
                .expect("Could not find default desktop directory")
                .desktop_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            sgu_name: "".to_string(),
            default_tag: Some("Geral".to_string()),
            ignore_tag: Some("<IGNORE>".to_string()),
            report_version: ReportVersion::default(),
            effort_type: EffortType::default(),
            report_encoding: ReportEncoding::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct LastUsedOptions {
    pub command: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

pub fn get_config() -> Result<MainConfig, confy::ConfyError> {
    let cfg: MainConfig = confy::load(APP_NAME, None).unwrap();

    Ok(cfg)
}

pub fn set_config(cfg: MainConfig) -> Result<(), confy::ConfyError> {
    let _ = confy::store(APP_NAME, None, cfg);
    Ok(())
}

pub fn restore_defaults() -> Result<(), confy::ConfyError> {
    let cfg = MainConfig::default();
    let _ = confy::store(APP_NAME, None, cfg);
    Ok(())
}

pub fn get_last_used_options() -> Result<LastUsedOptions, confy::ConfyError> {
    let cfg: LastUsedOptions = confy::load(APP_NAME, LAST_USED_OPTIONS_CONFIG_NAME).unwrap();

    // let file = confy::get_configuration_file_path(APP_NAME, LAST_USED_OPTIONS_CONFIG_NAME);
    // println!("The configuration file path is: {:#?}", file);
    Ok(cfg)
}

// pub fn set_last_used_options(cfg: LastUsedOptions) -> Result<(), confy::ConfyError> {
//     let _ = confy::store(APP_NAME, LAST_USED_OPTIONS_CONFIG_NAME, cfg);
//     Ok(())
// }
