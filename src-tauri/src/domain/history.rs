use super::task_policy::{applicable_task_ids, TaskId};

pub struct DailyRecord {
    local_date: String,
    applicable_task_ids: Vec<TaskId>,
    checked_task_ids: Vec<TaskId>,
}

impl DailyRecord {
    pub fn new(local_date: &str) -> Result<Self, String> {
        Ok(Self {
            local_date: local_date.to_owned(),
            applicable_task_ids: applicable_task_ids(local_date)?,
            checked_task_ids: Vec::new(),
        })
    }

    pub fn local_date(&self) -> &str {
        &self.local_date
    }

    pub fn applicable_task_ids(&self) -> &[TaskId] {
        &self.applicable_task_ids
    }

    pub fn completed_count(&self) -> usize {
        self.checked_task_ids.len()
    }

    pub fn set_checked(&mut self, task_id: TaskId, checked: bool) -> Result<(), String> {
        if !self.applicable_task_ids.contains(&task_id) {
            return Err("task is not applicable to this date".to_owned());
        }

        if checked && !self.checked_task_ids.contains(&task_id) {
            self.checked_task_ids.push(task_id);
        }

        if !checked {
            self.checked_task_ids
                .retain(|checked_id| *checked_id != task_id);
        }

        Ok(())
    }
}
