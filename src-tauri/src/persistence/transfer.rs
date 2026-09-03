use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::domain::task_policy::TaskId;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FullState {
    schema_version: u32,
    marker: String,
    records: Vec<TransferRecord>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct TransferRecord {
    local_date: String,
    applicable_task_ids: Vec<TaskId>,
    checked_task_ids: Vec<TaskId>,
    applicable_count: u32,
    completed_count: u32,
    policy_version: u32,
    updated_at_utc: String,
}

impl FullState {
    pub fn empty() -> Self {
        Self {
            schema_version: 1,
            marker: String::new(),
            records: Vec::new(),
        }
    }

    pub fn with_marker(&self, marker: &str) -> Self {
        let mut state = self.clone();
        state.marker = marker.to_owned();
        state
    }

    pub fn sample_multiday() -> Self {
        Self {
            schema_version: 1,
            marker: "sample".to_owned(),
            records: vec![
                TransferRecord {
                    local_date: "2026-08-31".to_owned(),
                    applicable_task_ids: vec![
                        TaskId::Meals,
                        TaskId::Sleep7h,
                        TaskId::JobApplication,
                        TaskId::Vitamins,
                        TaskId::LeetcodeOrDsa,
                        TaskId::SurfacesClean,
                    ],
                    checked_task_ids: vec![TaskId::Meals],
                    applicable_count: 6,
                    completed_count: 1,
                    policy_version: 1,
                    updated_at_utc: "2026-08-31T12:00:00Z".to_owned(),
                },
                TransferRecord {
                    local_date: "2026-08-28".to_owned(),
                    applicable_task_ids: vec![
                        TaskId::Meals,
                        TaskId::Sleep7h,
                        TaskId::Exercise,
                        TaskId::JobApplication,
                        TaskId::Vitamins,
                        TaskId::LeetcodeOrDsa,
                        TaskId::SurfacesClean,
                    ],
                    checked_task_ids: vec![TaskId::Meals, TaskId::Vitamins, TaskId::LeetcodeOrDsa],
                    applicable_count: 7,
                    completed_count: 3,
                    policy_version: 1,
                    updated_at_utc: "2026-08-28T12:00:00Z".to_owned(),
                },
            ],
        }
    }

    pub fn validate(&self, current_local_date: &str) -> Result<(), String> {
        if self.schema_version != 1 {
            return Err("import has an unsupported schema version".to_owned());
        }

        for record in &self.records {
            let record_date = NaiveDate::parse_from_str(&record.local_date, "%Y-%m-%d")
                .map_err(|error| format!("import has an invalid local date: {error}"))?;
            let current_date = NaiveDate::parse_from_str(current_local_date, "%Y-%m-%d")
                .map_err(|error| format!("invalid current local date: {error}"))?;

            if record_date > current_date {
                return Err("import contains a future record".to_owned());
            }

            let has_duplicate_applicable_ids = has_duplicates(&record.applicable_task_ids);
            let has_duplicate_checked_ids = has_duplicates(&record.checked_task_ids);
            let checked_ids_are_applicable = record
                .checked_task_ids
                .iter()
                .all(|task_id| record.applicable_task_ids.contains(task_id));
            let totals_match_snapshots = record.applicable_count as usize
                == record.applicable_task_ids.len()
                && record.completed_count as usize == record.checked_task_ids.len();

            if record.applicable_count == 0
                || record.completed_count > record.applicable_count
                || has_duplicate_applicable_ids
                || has_duplicate_checked_ids
                || !checked_ids_are_applicable
                || !totals_match_snapshots
            {
                return Err("import contains inconsistent record totals".to_owned());
            }
        }

        Ok(())
    }

    pub fn marker(&self) -> &str {
        &self.marker
    }
}

fn has_duplicates(task_ids: &[TaskId]) -> bool {
    task_ids
        .iter()
        .enumerate()
        .any(|(index, task_id)| task_ids[index + 1..].contains(task_id))
}
