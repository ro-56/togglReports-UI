#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio;

use std::path::Path;
use toggl_api::Command;
mod config;
mod report;
mod toggl_api;

#[tauri::command]
fn echo(name: &str) -> String {
    format!("{}", name)
}

#[tauri::command]
fn get_available_commands() -> Vec<Command<'static>> {
    toggl_api::get_available_commands()
}

#[tauri::command]
fn run(command: Command) -> String {
    println!("Running: {:#?}", command);

    // Get time entries from Toggl API
    // Get from sync function to this sync function
    let rt = tokio::runtime::Runtime::new().unwrap();
    let entries = match rt.block_on(toggl_api::get_time_entries_with_command(&command)) {
        Ok(res) => res,
        Err(e) => return format!("{}", e),
    };
    let entries_number = entries.len();

    // Transform time entries to SGU format
    let sgu_entries = report::transform_multiple_time_entries(entries);
    let csv_file = report::export_as_csv(sgu_entries);

    format!("Saved to {}. Found {} entries.", csv_file, entries_number)
}

#[tauri::command]
fn get_last_used_options() -> config::LastUsedOptions {
    let cfg: config::LastUsedOptions = config::get_last_used_options().unwrap();
    cfg
}

#[tauri::command]
fn get_config() -> config::MainConfig {
    let cfg: config::MainConfig = config::get_config().unwrap();
    cfg
}

#[tauri::command]
fn set_config(cfg: config::MainConfig) -> bool {
    println!("{:#?}", cfg);

    let config = config::set_config(cfg);

    match config {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[tauri::command]
fn check_if_dir_exists(dir: &str) -> bool {
    let b: bool = Path::new(dir).is_dir();
    println!("{:#?}", b);
    println!("{:#?}", dir);

    b
}

#[tauri::command]
fn restore_defaults() {
    config::restore_defaults().unwrap();
}

#[tauri::command]
fn get_workspaces(api_token: String) -> Vec<toggl_api::SimpleWorkspace> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let workspaces = rt
        .block_on(toggl_api::get_workspaces(Some(api_token)))
        .unwrap();

    let mut export_workspace: Vec<toggl_api::SimpleWorkspace> = Vec::new();

    for workspace in workspaces {
        export_workspace.push(toggl_api::SimpleWorkspace {
            id: workspace.id,
            name: workspace.name,
        })
    }

    export_workspace
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            echo,
            run,
            get_available_commands,
            get_last_used_options,
            get_config,
            set_config,
            check_if_dir_exists,
            restore_defaults,
            get_workspaces,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
