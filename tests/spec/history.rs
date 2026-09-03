//! Frozen Phase 1 behavioral contract. Do not weaken without owner approval.

use tauri_app_lib::domain::{
    date_validation::validate_mutation_date, history::DailyRecord, task_policy::TaskId,
};

#[test]
fn historic_record_retains_its_tuesday_six_task_snapshot_when_viewed_later() {
    let tuesday_record = DailyRecord::new("2026-09-08").unwrap();

    assert_eq!(tuesday_record.local_date(), "2026-09-08");
    assert_eq!(tuesday_record.applicable_task_ids().len(), 6);
    assert!(!tuesday_record
        .applicable_task_ids()
        .contains(&TaskId::Exercise));
}

#[test]
fn editing_a_past_record_changes_only_that_records_checked_snapshot() {
    let mut tuesday_record = DailyRecord::new("2026-09-08").unwrap();
    let wednesday_record = DailyRecord::new("2026-09-09").unwrap();

    tuesday_record.set_checked(TaskId::Meals, true).unwrap();

    assert_eq!(tuesday_record.completed_count(), 1);
    assert_eq!(wednesday_record.completed_count(), 0);
    assert_eq!(wednesday_record.applicable_task_ids().len(), 7);
}

#[test]
fn future_dates_are_rejected_against_the_injected_current_local_date() {
    assert!(validate_mutation_date("2026-09-02", "2026-09-02").is_ok());
    assert!(validate_mutation_date("2026-09-01", "2026-09-02").is_ok());
    assert!(validate_mutation_date("2026-09-03", "2026-09-02").is_err());
}
