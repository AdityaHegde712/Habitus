use std::{
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use tauri_app_lib::{
    domain::task_policy::TaskId,
    persistence::{repository::Repository, transfer::FullState},
};

fn isolated_directory(test_name: &str) -> PathBuf {
    let unique_suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time must be after the Unix epoch")
        .as_nanos();

    std::env::temp_dir().join(format!(
        "habit-tracker-{test_name}-{}-{unique_suffix}",
        std::process::id()
    ))
}

#[test]
fn valid_import_retains_the_complete_pre_import_state_as_the_latest_backup() {
    let repository = Repository::open(isolated_directory("import-backup")).unwrap();
    let original = FullState::sample_multiday();
    repository.seed(original.clone()).unwrap();

    let replacement = original.with_marker("replacement");
    let replacement_json = serde_json::to_string(&replacement).unwrap();
    repository
        .import_state(&replacement_json, "2026-09-02")
        .unwrap();

    assert_eq!(repository.previous_backup_state(1).unwrap(), original);
    assert_eq!(repository.export_state().unwrap(), replacement_json);
}

#[test]
fn reopening_an_existing_database_keeps_the_schema_and_saved_state_intact() {
    let directory = isolated_directory("idempotent-open");
    let state = FullState::sample_multiday();
    let expected_export = serde_json::to_string(&state).unwrap();

    let repository = Repository::open(directory.clone()).unwrap();
    repository.seed(state).unwrap();
    drop(repository);

    let reopened = Repository::open(directory).unwrap();
    assert_eq!(reopened.export_state().unwrap(), expected_export);
}

#[test]
fn typed_task_mutation_persists_a_historical_snapshot_and_retains_its_pre_change_backup() {
    let repository = Repository::open(isolated_directory("task-mutation")).unwrap();
    repository.seed(FullState::empty()).unwrap();

    repository
        .set_task_checked(
            "2026-09-01",
            TaskId::Meals,
            true,
            "2026-09-02",
            "2026-09-02T12:00:00Z",
        )
        .unwrap();
    let state_after_first_mutation = repository.export_state().unwrap();

    repository
        .set_task_checked(
            "2026-09-01",
            TaskId::Vitamins,
            true,
            "2026-09-02",
            "2026-09-02T12:01:00Z",
        )
        .unwrap();

    let current_state: serde_json::Value =
        serde_json::from_str(&repository.export_state().unwrap()).unwrap();
    assert_eq!(current_state["records"][0]["local_date"], "2026-09-01");
    assert_eq!(current_state["records"][0]["applicable_count"], 6);
    assert_eq!(current_state["records"][0]["completed_count"], 2);
    assert_eq!(
        repository.previous_backup_state(1).unwrap(),
        serde_json::from_str(&state_after_first_mutation).unwrap()
    );
}
