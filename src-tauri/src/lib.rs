use std::sync::atomic::{AtomicBool, Ordering};

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, RunEvent, WindowEvent,
};

pub mod application;
mod commands;
pub mod domain;
pub mod persistence;

const EXIT_MENU_ID: &str = "exit";
const MAIN_WINDOW_LABEL: &str = "main";
const OPEN_MENU_ID: &str = "open";

#[derive(Default)]
struct LifecycleState {
    is_quitting: AtomicBool,
}

fn main_window(app: &tauri::AppHandle) -> Option<tauri::WebviewWindow> {
    let window = app.get_webview_window(MAIN_WINDOW_LABEL);

    if window.is_none() {
        eprintln!("Unable to locate the main window for a lifecycle operation.");
    }

    window
}

fn report_window_error(action: &str, result: tauri::Result<()>) {
    if let Err(error) = result {
        eprintln!("Unable to {action} the main window: {error}");
    }
}

fn show_main_window(app: &tauri::AppHandle) {
    let Some(window) = main_window(app) else {
        return;
    };

    report_window_error("unminimize", window.unminimize());
    report_window_error("show", window.show());
    report_window_error("focus", window.set_focus());
}

fn hide_main_window(app: &tauri::AppHandle) {
    let Some(window) = main_window(app) else {
        return;
    };

    report_window_error("hide", window.hide());
}

fn begin_shutdown(app: &tauri::AppHandle) {
    app.state::<LifecycleState>()
        .is_quitting
        .store(true, Ordering::SeqCst);
    app.exit(0);
}

fn handle_tray_menu_event(app: &tauri::AppHandle, menu_id: &str) {
    match menu_id {
        OPEN_MENU_ID => show_main_window(app),
        EXIT_MENU_ID => begin_shutdown(app),
        _ => {}
    }
}

fn handle_window_event(app: &tauri::AppHandle, label: &str, event: &WindowEvent) {
    if label != MAIN_WINDOW_LABEL {
        return;
    }

    let WindowEvent::CloseRequested { api, .. } = event else {
        return;
    };

    let is_quitting = app
        .state::<LifecycleState>()
        .is_quitting
        .load(Ordering::SeqCst);

    if is_quitting {
        return;
    }

    api.prevent_close();
    hide_main_window(app);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(LifecycleState::default())
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .setup(|app| {
            commands::initialize_state(app)?;

            let open_item = MenuItem::with_id(app, OPEN_MENU_ID, "Open", true, None::<&str>)?;
            let exit_item = MenuItem::with_id(app, EXIT_MENU_ID, "Exit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_item, &exit_item])?;

            TrayIconBuilder::new()
                .icon(
                    app.default_window_icon()
                        .expect("missing application icon")
                        .clone(),
                )
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| handle_tray_menu_event(app, event.id.as_ref()))
                .build(app)?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_day,
            commands::set_task_checked,
            commands::list_calendar_days,
            commands::export_state,
            commands::import_state,
            commands::get_autostart_status,
            commands::set_autostart_enabled,
        ])
        .build(tauri::generate_context!())
        .expect("failed to build Habit Tracker")
        .run(|app, event| {
            if let RunEvent::WindowEvent { label, event, .. } = event {
                handle_window_event(app, label.as_ref(), &event);
            }
        });
}
