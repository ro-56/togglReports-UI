// extern crate tokio;
extern crate serde_json;
use std::str::FromStr;

use chrono::{naive::NaiveDate, Datelike, Local, TimeDelta};
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use reqwest::Error;
use serde::{Deserialize, Serialize};

use crate::config::get_config;

const DATE_FORMAT: &str = "%Y-%m-%d";
const DATE_FORMAT_DAY_ONE: &str = "%Y-%m-01";

#[derive(Debug, Deserialize, Serialize)]
pub enum TimeInterval {
    ThisWeek,
    LastWeek,
    ThisMonth,
    LastMonth,
    Custom,
}
impl ToString for TimeInterval {
    fn to_string(&self) -> String {
        match self {
            TimeInterval::ThisWeek => String::from("this_week"),
            TimeInterval::LastWeek => String::from("last_week"),
            TimeInterval::ThisMonth => String::from("this_month"),
            TimeInterval::LastMonth => String::from("last_month"),
            TimeInterval::Custom => String::from("custom"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Command<'a> {
    pub command: TimeInterval,
    pub label: &'a str,
    pub requires_date_range: bool,
    pub start_date: Option<&'a str>,
    pub end_date: Option<&'a str>,
}

pub fn get_available_commands() -> Vec<Command<'static>> {
    vec![
        Command {
            command: TimeInterval::ThisWeek,
            label: "This Week",
            requires_date_range: false,
            start_date: None,
            end_date: None,
        },
        Command {
            command: TimeInterval::LastWeek,
            label: "Last Week",
            requires_date_range: false,
            start_date: None,
            end_date: None,
        },
        Command {
            command: TimeInterval::ThisMonth,
            label: "This Month",
            requires_date_range: false,
            start_date: None,
            end_date: None,
        },
        Command {
            command: TimeInterval::LastMonth,
            label: "Last Month",
            requires_date_range: false,
            start_date: None,
            end_date: None,
        },
        Command {
            command: TimeInterval::Custom,
            label: "Custom",
            requires_date_range: true,
            start_date: None,
            end_date: None,
        },
    ]
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Workspace {
    admin: bool,
    api_token: String,
    at: String,
    business_ws: bool,
    // csv_upload: Option<CsvUpload>,  //DISABLED
    default_currency: String,
    default_hourly_rate: Option<f32>,
    ical_enabled: bool,
    ical_url: String,
    pub id: u64,
    logo_url: String,
    pub name: String,
    only_admins_may_create_projects: bool,
    only_admins_may_create_tags: bool,
    only_admins_see_billable_rates: bool,
    only_admins_see_team_dashboard: bool,
    organization_id: u32,
    premium: bool,
    profile: u32,
    projects_billable_by_default: bool,
    rate_last_updated: Option<String>,
    reports_collapse: bool,
    role: Option<String>,
    rounding: u32,
    rounding_minutes: u32,
    server_deleted_at: Option<String>,
    // subscription: Option<String>,  //DISABLED
    suspended_at: Option<String>,
    working_hours_in_minutes: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleWorkspace {
    pub id: u64,
    pub name: String,
}

pub async fn get_workspaces(token: Option<String>) -> Result<Vec<Workspace>, Error> {
    let api_token;
    match token {
        Some(token) => api_token = token,
        None => {
            println!("No API token found");
            api_token = get_config().unwrap().api_token;
        }
    }

    let client = Client::new();
    let url = "https://api.track.toggl.com/api/v9/workspaces";

    let response = client
        .get(url)
        .basic_auth(api_token, Some("api_token"))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<Vec<Workspace>>().await {
                Ok(workspaces) => {
                    println!("Found {:?} workspaces", workspaces.len());
                    // for workspace in workspaces {
                    //   println!("Workspace: {:?}", workspace);
                    // }
                    Ok(workspaces)
                }
                Err(e) => {
                    println!("Workspace Json Error: {:?}", e);
                    Err(e)
                }
            }
        }
        _ => {
            println!("Workspace Request Error: {:?}", response.status());
            // Err(response.error_for_status().unwrap_err())
            Ok(vec![])
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Project {
    pub id: u32,
    pub workspace_id: u32,
    client_id: Option<u32>,
    pub name: String,
    is_private: bool,
    active: bool,
    at: String,
    created_at: String,
    server_deleted_at: Option<String>,
    color: String,
    billable: Option<bool>,
    template: Option<bool>,
    auto_estimates: Option<bool>,
    estimated_hours: Option<u32>,
    rate: Option<f32>,
    rate_last_updated: Option<String>,
    currency: Option<String>,
    recurring: bool,
    recurring_parameters: Option<String>,
    current_period: Option<String>,
    fixed_fee: Option<f32>,
    actual_hours: Option<u32>,
    wid: Option<u32>,
    cid: Option<u32>,
}

async fn get_projects() -> Result<Vec<Project>, Error> {
    let api_token = get_config().unwrap().api_token;
    let client = Client::new();
    let url = "https://track.toggl.com/api/v9/me/projects".to_string();

    let response = client
        .get(url)
        .basic_auth(api_token, Some("api_token"))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<Vec<Project>>().await {
                Ok(projects) => {
                    println!("Found {:?} projects", projects.len());
                    // for project in projects {
                    //   println!("Project: {:?}", project);
                    // }
                    Ok(projects)
                }
                Err(e) => {
                    println!("Error while casting to <Project>: {:?}", e);
                    Err(e)
                }
            }
        }
        _ => {
            panic!("Error: {:?}", response.status());
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TimeEntry {
    at: String,
    billable: bool,
    pub description: Option<String>,
    pub duration: i64,
    duronly: bool,
    id: u32,
    pub project_id: Option<u32>,
    pub project_name: Option<String>,
    server_deleted_at: Option<String>,
    pub start: String,
    stop: Option<String>,
    tag_ids: Vec<u32>,
    pub tags: Vec<String>,
    task_id: Option<u32>,
    user_id: u32,
    pub workspace_id: u32,
    uid: Option<u32>, //Legacy
    wid: Option<u32>, //Legacy
    pid: Option<u32>, //Legacy
    tid: Option<u32>, //Legacy
}

async fn get_time_entries(
    (start_date, end_date): (String, String),
) -> Result<Vec<TimeEntry>, Error> {
    let api_token = get_config().unwrap().api_token;
    let client = Client::new();
    let url = format!(
    "https://api.track.toggl.com/api/v9/me/time_entries?start_date={start_date}&end_date={end_date}",
    start_date=start_date,
    end_date=end_date
  );

    let response = client
        .get(url)
        .basic_auth(api_token, Some("api_token"))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap();

    match response.status() {
        reqwest::StatusCode::OK => {
            match response.json::<Vec<TimeEntry>>().await {
                Ok(time_entries) => {
                    println!("Got '{:?}' time Entries", time_entries.len());
                    // for time_entry in time_entries {
                    //   println!("Time Entry: {:?}", time_entry);
                    // }
                    Ok(time_entries)
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                    Err(e)
                }
            }
        }
        _ => {
            panic!("Error: {:?}", response.status());
        }
    }
}

pub async fn get_time_entries_with_command(command: &Command<'_>) -> Result<Vec<TimeEntry>, Error> {
    let start_date: String;
    let end_date: String;

    // End date is exclusive, so we need to add 1 day to every end_date
    if command.requires_date_range {
        start_date = command.start_date.unwrap().to_string();
        end_date = (NaiveDate::from_str(command.end_date.unwrap())
            .expect("End date format should be 'yyyy-mm-dd'")
            + TimeDelta::try_days(1).unwrap())
        .format(DATE_FORMAT)
        .to_string();
    } else {
        let now = Local::now();
        start_date = match command.command {
            TimeInterval::ThisWeek => (now
                - TimeDelta::try_days(now.weekday().num_days_from_monday() as i64).unwrap())
            .format(DATE_FORMAT)
            .to_string(),
            TimeInterval::LastWeek => (now
                - TimeDelta::try_days(7 + now.weekday().num_days_from_monday() as i64).unwrap())
            .format(DATE_FORMAT)
            .to_string(),
            TimeInterval::ThisMonth => now.format(DATE_FORMAT_DAY_ONE).to_string(),
            TimeInterval::LastMonth => (now - TimeDelta::try_days(now.day() as i64).unwrap())
                .format(DATE_FORMAT_DAY_ONE)
                .to_string(),
            _ => "".to_string(),
        };
        end_date = match command.command {
            TimeInterval::ThisWeek => (now
                + TimeDelta::try_days(6 - now.weekday().num_days_from_monday() as i64).unwrap()
                + TimeDelta::try_days(1).unwrap())
            .format(DATE_FORMAT)
            .to_string(),
            TimeInterval::LastWeek => (now
                - TimeDelta::try_days(1 + now.weekday().num_days_from_monday() as i64).unwrap()
                + TimeDelta::try_days(1).unwrap())
            .format(DATE_FORMAT)
            .to_string(),
            TimeInterval::ThisMonth => {
                let next_month = {
                    if now.month() == 12 {
                        1
                    } else {
                        now.month() + 1
                    }
                };
                let next_year = {
                    if now.month() == 12 {
                        now.year() + 1
                    } else {
                        now.year()
                    }
                };
                (NaiveDate::from_ymd_opt(next_year, next_month, 1).unwrap()
                    - TimeDelta::try_days(1).unwrap()
                    + TimeDelta::try_days(1).unwrap())
                .format(DATE_FORMAT)
                .to_string()
            }
            TimeInterval::LastMonth => (now - TimeDelta::try_days(now.day() as i64).unwrap()
                + TimeDelta::try_days(1).unwrap())
            .format(DATE_FORMAT)
            .to_string(),
            _ => "".to_string(),
        };
    }

    println!("Start Date: {:?}", start_date);
    println!("End Date: {:?}", end_date);
    let mut time_entries = get_time_entries((start_date, end_date)).await.unwrap();
    
    let projects = get_projects().await?;

    for time_entry in &mut time_entries {
        // println!("Time Entry: {:?}", time_entry);
        let project_id = time_entry.project_id;
        let workspace_id = time_entry.workspace_id;
        let project_name = {
            match project_id {
                Some(id) => Some(
                    projects
                        .iter()
                        .find(|&p| (p.id == id) && (p.workspace_id == workspace_id))
                        .expect("Project name not found")
                        .name
                        .clone(),
                ),
                None => None,
            }
        };

        time_entry.project_name = project_name;
    }

    Ok(time_entries)
}
