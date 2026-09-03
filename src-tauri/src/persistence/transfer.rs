use chrono::{DateTime, NaiveDate};
use serde::{Deserialize, Serialize};

use crate::domain::{
    date_validation::validate_mutation_date,
    task_policy::{applicable_task_ids, TaskId},
};

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

        let current_date = NaiveDate::parse_from_str(current_local_date, "%Y-%m-%d")
            .map_err(|error| format!("invalid current local date: {error}"))?;

        if self.records.iter().enumerate().any(|(index, record)| {
            self.records[index + 1..]
                .iter()
                .any(|other| other.local_date == record.local_date)
        }) {
            return Err("import contains duplicate local dates".to_owned());
        }

        for record in &self.records {
            let record_date = NaiveDate::parse_from_str(&record.local_date, "%Y-%m-%d")
                .map_err(|error| format!("import has an invalid local date: {error}"))?;

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
            let updated_at_is_utc = DateTime::parse_from_rfc3339(&record.updated_at_utc).is_ok()
                && record.updated_at_utc.ends_with('Z');

            if record.applicable_count == 0
                || record.completed_count > record.applicable_count
                || has_duplicate_applicable_ids
                || has_duplicate_checked_ids
                || !checked_ids_are_applicable
                || !totals_match_snapshots
                || !updated_at_is_utc
            {
                return Err("import contains inconsistent record totals".to_owned());
            }
        }

        Ok(())
    }

    pub fn marker(&self) -> &str {
        &self.marker
    }

    pub fn set_task_checked(
        &mut self,
        local_date: &str,
        task_id: TaskId,
        checked: bool,
        current_local_date: &str,
        updated_at_utc: &str,
    ) -> Result<(), String> {
        validate_mutation_date(local_date, current_local_date)?;
        validate_utc_timestamp(updated_at_utc)?;

        let record_index = match self
            .records
            .iter()
            .position(|record| record.local_date == local_date)
        {
            Some(index) => index,
            None => {
                self.records
                    .push(TransferRecord::new(local_date, updated_at_utc)?);
                self.records.len() - 1
            }
        };

        self.records[record_index].set_task_checked(task_id, checked, updated_at_utc)
    }
}

impl TransferRecord {
    fn new(local_date: &str, updated_at_utc: &str) -> Result<Self, String> {
        let applicable_task_ids = applicable_task_ids(local_date)?;
        let applicable_count = applicable_task_ids.len() as u32;

        Ok(Self {
            local_date: local_date.to_owned(),
            applicable_task_ids,
            checked_task_ids: Vec::new(),
            applicable_count,
            completed_count: 0,
            policy_version: 1,
            updated_at_utc: updated_at_utc.to_owned(),
        })
    }

    fn set_task_checked(
        &mut self,
        task_id: TaskId,
        checked: bool,
        updated_at_utc: &str,
    ) -> Result<(), String> {
        if !self.applicable_task_ids.contains(&task_id) {
            return Err("task is not applicable to this date".to_owned());
        }

        if checked && !self.checked_task_ids.contains(&task_id) {
            self.checked_task_ids.push(task_id);
        }

        if !checked {
            self.checked_task_ids
                .retain(|checked_task_id| *checked_task_id != task_id);
        }

        self.completed_count = self.checked_task_ids.len() as u32;
        self.updated_at_utc = updated_at_utc.to_owned();
        Ok(())
    }
}

fn validate_utc_timestamp(timestamp: &str) -> Result<(), String> {
    if DateTime::parse_from_rfc3339(timestamp).is_err() || !timestamp.ends_with('Z') {
        return Err("updated timestamp must be an RFC 3339 UTC value".to_owned());
    }

    Ok(())
}

fn has_duplicates(task_ids: &[TaskId]) -> bool {
    task_ids
        .iter()
        .enumerate()
        .any(|(index, task_id)| task_ids[index + 1..].contains(task_id))
}
