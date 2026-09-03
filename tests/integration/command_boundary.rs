use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use tauri_app_lib::{application::HabitService, domain::task_policy::TaskId};

fn isolated_directory(test_name: &str) -> PathBuf {
    let unique_suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after the Unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!(
        "habit-tracker-command-{test_name}-{}-{unique_suffix}",
        std::process::id()
    ))
}

#[test]
fn command_service_uses_its_host_owned_directory_and_rejects_invalid_inputs() {
    let service = HabitService::open(isolated_directory("validation")).unwrap();
    let original = service.export_state().unwrap();

    assert!(serde_json::from_str::<TaskId>("\"unknown\"").is_err());
    assert!(service.get_day("2026-09-03", "2026-09-02").is_err());
    assert!(service
        .set_task_checked(
            "2026-09-03",
            TaskId::Meals,
            true,
            "2026-09-02",
            "2026-09-02T12:00:00Z",
        )
        .is_err());
    assert_eq!(service.export_state().unwrap(), original);
}

#[test]
fn command_service_exposes_only_typed_daily_and_transfer_operations() {
    let service = HabitService::open(isolated_directory("operations")).unwrap();

    let empty_day = service.get_day("2026-09-02", "2026-09-02").unwrap();
    assert_eq!(empty_day.local_date, "2026-09-02");
    assert_eq!(empty_day.applicable_count, 7);
    assert_eq!(empty_day.completed_count, 0);

    service
        .set_task_checked(
            "2026-09-02",
            TaskId::Meals,
            true,
            "2026-09-02",
            "2026-09-02T12:00:00Z",
        )
        .unwrap();

    let updated_day = service.get_day("2026-09-02", "2026-09-02").unwrap();
    assert_eq!(updated_day.checked_task_ids, vec![TaskId::Meals]);
    assert_eq!(service.list_calendar_days().unwrap().len(), 1);

    let exported = service.export_state().unwrap();
    service.import_state(&exported, "2026-09-02").unwrap();
    assert_eq!(service.export_state().unwrap(), exported);
}
