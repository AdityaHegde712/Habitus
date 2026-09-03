//! Frozen Phase 1 behavioral contract. Do not weaken without owner approval.

use tauri_app_lib::domain::task_policy::{applicable_task_ids, completion_color, Rgb, TaskId};

fn all_task_ids() -> Vec<TaskId> {
    vec![
        TaskId::Meals,
        TaskId::Sleep7h,
        TaskId::Exercise,
        TaskId::JobApplication,
        TaskId::Vitamins,
        TaskId::LeetcodeOrDsa,
        TaskId::SurfacesClean,
    ]
}

#[test]
fn monday_wednesday_and_friday_apply_all_seven_stable_tasks() {
    for date in ["2026-09-07", "2026-09-09", "2026-09-11"] {
        assert_eq!(applicable_task_ids(date).unwrap(), all_task_ids(), "{date}");
    }
}

#[test]
fn non_exercise_days_exclude_exercise_and_retain_the_other_six_tasks() {
    let expected = vec![
        TaskId::Meals,
        TaskId::Sleep7h,
        TaskId::JobApplication,
        TaskId::Vitamins,
        TaskId::LeetcodeOrDsa,
        TaskId::SurfacesClean,
    ];

    for date in ["2026-09-08", "2026-09-10", "2026-09-12", "2026-09-13"] {
        assert_eq!(applicable_task_ids(date).unwrap(), expected, "{date}");
    }
}

#[test]
fn completion_color_uses_the_approved_rounded_green_fraction() {
    assert_eq!(completion_color(0, 6).unwrap(), Rgb::new(0, 0, 0));
    assert_eq!(completion_color(3, 6).unwrap(), Rgb::new(0, 128, 0));
    assert_eq!(completion_color(3, 7).unwrap(), Rgb::new(0, 109, 0));
    assert_eq!(completion_color(7, 7).unwrap(), Rgb::new(0, 255, 0));
}
