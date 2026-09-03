use std::{error::Error, path::PathBuf};

use chrono::{Local, SecondsFormat, Utc};
use tauri::{App, AppHandle, Manager, State};
use tauri_plugin_autostart::ManagerExt;

use crate::{
    application::HabitService,
    domain::task_policy::TaskId,
    persistence::transfer::{CalendarDay, DailyRecordView},
};

pub struct ApplicationState {
    service: HabitService,
}

impl ApplicationState {
    fn open(data_directory: PathBuf) -> Result<Self, String> {
        Ok(Self {
            service: HabitService::open(data_directory)?,
        })
    }
}

pub fn initialize_state(app: &App) -> Result<(), Box<dyn Error>> {
    let data_directory = app.path().app_local_data_dir()?;
    let state = ApplicationState::open(data_directory).map_err(std::io::Error::other)?;
    app.manage(state);
    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub fn get_day(
    local_date: String,
    state: State<'_, ApplicationState>,
) -> Result<DailyRecordView, String> {
    state.service.get_day(&local_date, &current_local_date())
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_task_checked(
    local_date: String,
    task_id: TaskId,
    checked: bool,
    state: State<'_, ApplicationState>,
) -> Result<(), String> {
    state.service.set_task_checked(
        &local_date,
        task_id,
        checked,
        &current_local_date(),
        &current_utc_timestamp(),
    )
}

#[tauri::command]
pub fn list_calendar_days(state: State<'_, ApplicationState>) -> Result<Vec<CalendarDay>, String> {
    state.service.list_calendar_days()
}

#[tauri::command]
pub fn export_state(state: State<'_, ApplicationState>) -> Result<String, String> {
    state.service.export_state()
}

#[tauri::command(rename_all = "snake_case")]
pub fn import_state(
    serialized_state: String,
    state: State<'_, ApplicationState>,
) -> Result<(), String> {
    state
        .service
        .import_state(&serialized_state, &current_local_date())
}

#[tauri::command]
pub fn get_autostart_status(app: AppHandle) -> Result<bool, String> {
    app.autolaunch()
        .is_enabled()
        .map_err(|error| format!("unable to read autostart status: {error}"))
}

#[tauri::command(rename_all = "snake_case")]
pub fn set_autostart_enabled(enabled: bool, app: AppHandle) -> Result<bool, String> {
    let autostart = app.autolaunch();

    if enabled {
        autostart
            .enable()
            .map_err(|error| format!("unable to enable autostart: {error}"))?;
    } else {
        autostart
            .disable()
            .map_err(|error| format!("unable to disable autostart: {error}"))?;
    }

    autostart
        .is_enabled()
        .map_err(|error| format!("unable to read autostart status: {error}"))
}

fn current_local_date() -> String {
    Local::now().date_naive().format("%Y-%m-%d").to_string()
}

fn current_utc_timestamp() -> String {
    Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true)
}
