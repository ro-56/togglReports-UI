// extern crate tokio;
extern crate serde_json;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use reqwest::header::CONTENT_TYPE;
use reqwest::Error;

#[derive(Debug, Deserialize, Serialize)]
struct TimeEntry {
  at: String,
  billable: bool,
  description: Option<String>,
  duration: i64,
  duronly: bool,
  id: u32,
  project_id: Option<u32>,
  server_deleted_at: Option<String>,
  start: String,
  stop: Option<String>,
  tag_ids: Vec<u32>,
  tags: Vec<String>,
  task_id: Option<u32>,
  user_id: u32,
  workspace_id: u32,
  uid: Option<u32>,  //Legacy
  wid: Option<u32>,  //Legacy
  pid: Option<u32>,  //Legacy
  tid: Option<u32>  //Legacy
}


#[derive(Debug, Deserialize, Serialize)]
struct Workspace {
  admin: bool,
  api_token: String,
  at: String,
  business_ws: bool,
  // csv_upload: Option<CsvUpload>,  //DISABLED
  default_currency: String,
  default_hourly_rate: Option<f32>,
  ical_enabled: bool,
  ical_url: String,
  id: u64,
  logo_url: String,
  name: String,
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
  working_hours_in_minutes: Option<String>
}


async fn get_workspaces() -> Result<Vec<Workspace>, Error> {
  let client = Client::new();
  let url = "https://api.track.toggl.com/api/v9/workspaces";

  let response = client
    .get(url)
    .basic_auth("20b6e9ead84d01cbf4fb37913e5a82ed", Some("api_token"))
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
        },
        Err(e) => {
          println!("Error: {:?}", e);
          Err(e)
        }
      }
    },
    _ => {
      panic!("Error: {:?}", response.status());
    }
  }

}


async fn get_time_entries((start_date, end_date): (String, String)) -> Result<Vec<TimeEntry>, Error> {
  let client = Client::new();
  let url = format!(
    "https://api.track.toggl.com/api/v9/me/time_entries?start_date={start_date}&end_date={end_date}",
    start_date=start_date,
    end_date=end_date
  );

  let response = client
    .get(url)
    .basic_auth("20b6e9ead84d01cbf4fb37913e5a82ed", Some("api_token"))
    .header(CONTENT_TYPE, "application/json")
    .send()
    .await
    .unwrap();

  match response.status() {
    reqwest::StatusCode::OK => {
      match response.json::<Vec<TimeEntry>>().await {
        Ok(time_entries) => {
          println!("Time Entries: {:?}", time_entries.len());
          // for time_entry in time_entries {
          //   println!("Time Entry: {:?}", time_entry);
          // }
          Ok(time_entries)
        },
        Err(e) => {
          println!("Error: {:?}", e);
          Err(e)
        }
      }
    },
    _ => {
      panic!("Error: {:?}", response.status());
    }
  }
}
