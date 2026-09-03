//! Frozen Phase 1 behavioral contract. Do not weaken without owner approval.

use std::path::PathBuf;

use tauri_app_lib::persistence::{backup_recovery::BackupStore, transfer::FullState};

fn isolated_directory(test_name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("habit-tracker-{test_name}-{}", std::process::id()))
}

#[test]
fn successful_mutations_retain_the_two_immediately_preceding_full_states_in_order() {
    let mut backups = BackupStore::new(isolated_directory("two-prior")).unwrap();
    let first = FullState::empty();
    let second = first.with_marker("second");
    let third = second.with_marker("third");

    backups.capture_pre_change(&first).unwrap();
    backups.capture_pre_change(&second).unwrap();
    backups.capture_pre_change(&third).unwrap();

    assert_eq!(backups.previous_state(1).unwrap(), third);
    assert_eq!(backups.previous_state(2).unwrap(), second);
}

#[test]
fn a_third_backup_evicts_only_the_oldest_state() {
    let mut backups = BackupStore::new(isolated_directory("eviction")).unwrap();
    let first = FullState::empty().with_marker("first");
    let second = first.with_marker("second");
    let third = second.with_marker("third");

    backups.capture_pre_change(&first).unwrap();
    backups.capture_pre_change(&second).unwrap();
    backups.capture_pre_change(&third).unwrap();

    assert!(!backups.contains_marker("first").unwrap());
    assert!(backups.contains_marker("second").unwrap());
    assert!(backups.contains_marker("third").unwrap());
}

#[test]
fn interrupted_rotation_recovers_from_the_staged_pre_change_state() {
    let mut backups = BackupStore::new(isolated_directory("recovery")).unwrap();
    let state = FullState::empty().with_marker("recover-me");

    backups.stage_pre_change(&state).unwrap();
    drop(backups);

    let recovered = BackupStore::new(isolated_directory("recovery")).unwrap();
    assert_eq!(recovered.previous_state(1).unwrap(), state);
}
