use std::sync::atomic::{AtomicBool, Ordering};

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, RunEvent, WindowEvent,
};
use tauri_plugin_autostart::ManagerExt;

const EXIT_MENU_ID: &str = "exit";
const MAIN_WINDOW_LABEL: &str = "main";
const OPEN_MENU_ID: &str = "open";
const PHASE0_AUTOSTART_DISABLE_ARGUMENT: &str = "--phase0-autostart=disable";
const PHASE0_AUTOSTART_ENABLE_ARGUMENT: &str = "--phase0-autostart=enable";
const PHASE0_AUTOSTART_STATUS_ARGUMENT: &str = "--phase0-autostart=status";

#[derive(Default)]
struct LifecycleState {
    is_quitting: AtomicBool,
}

enum Phase0AutostartAction {
    Disable,
    Enable,
    Status,
}

fn phase0_autostart_action() -> Option<Phase0AutostartAction> {
    std::env::args().find_map(|argument| match argument.as_str() {
        PHASE0_AUTOSTART_DISABLE_ARGUMENT => Some(Phase0AutostartAction::Disable),
        PHASE0_AUTOSTART_ENABLE_ARGUMENT => Some(Phase0AutostartAction::Enable),
        PHASE0_AUTOSTART_STATUS_ARGUMENT => Some(Phase0AutostartAction::Status),
        _ => None,
    })
}

fn run_phase0_autostart_action(app: &tauri::App) -> Result<bool, Box<dyn std::error::Error>> {
    let Some(action) = phase0_autostart_action() else {
        return Ok(false);
    };

    let manager = app.autolaunch();

    match action {
        Phase0AutostartAction::Disable => manager.disable()?,
        Phase0AutostartAction::Enable => manager.enable()?,
        Phase0AutostartAction::Status => {}
    }

    println!("Phase 0 autostart enabled: {}", manager.is_enabled()?);
    Ok(true)
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
            let is_phase0_autostart_diagnostic = run_phase0_autostart_action(app)?;

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

            if is_phase0_autostart_diagnostic {
                app.handle().exit(0);
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("failed to build Habit Tracker")
        .run(|app, event| {
            if let RunEvent::WindowEvent { label, event, .. } = event {
                handle_window_event(app, label.as_ref(), &event);
            }
        });
}
