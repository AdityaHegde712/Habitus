//! Frozen Phase 1 behavioral contract. Do not weaken without owner approval.

use std::path::PathBuf;

use tauri_app_lib::persistence::{repository::Repository, transfer::FullState};

fn isolated_directory(test_name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("habit-tracker-{test_name}-{}", std::process::id()))
}

#[test]
fn valid_multiday_state_round_trips_through_json_export_and_import() {
    let source = Repository::open(isolated_directory("round-trip-source")).unwrap();
    source.seed(FullState::sample_multiday()).unwrap();
    let export = source.export_state().unwrap();

    let destination = Repository::open(isolated_directory("round-trip-destination")).unwrap();
    destination.import_state(&export, "2026-09-02").unwrap();

    assert_eq!(destination.export_state().unwrap(), export);
}

#[test]
fn invalid_imports_leave_live_state_unchanged() {
    let repository = Repository::open(isolated_directory("invalid-import")).unwrap();
    repository.seed(FullState::sample_multiday()).unwrap();
    let original = repository.export_state().unwrap();

    for invalid_json in [
        "{}",
        r#"{"records":[{"local_date":"2026-09-02","checked_task_ids":["unknown"]}]}"#,
        r#"{"records":[{"local_date":"2026-09-03","applicable_count":6,"completed_count":0}]}"#,
        r#"{"records":[{"local_date":"2026-09-02","applicable_count":6,"completed_count":7}]}"#,
    ] {
        assert!(repository.import_state(invalid_json, "2026-09-02").is_err());
        assert_eq!(repository.export_state().unwrap(), original);
    }
}
