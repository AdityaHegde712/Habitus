use std::{path::PathBuf, sync::Mutex};

use crate::{
    domain::{date_validation::validate_mutation_date, task_policy::TaskId},
    persistence::{
        repository::Repository,
        transfer::{CalendarDay, DailyRecordView},
    },
};

pub struct HabitService {
    repository: Mutex<Repository>,
}

impl HabitService {
    pub fn open(data_directory: PathBuf) -> Result<Self, String> {
        let repository = Repository::open(data_directory)?;

        Ok(Self {
            repository: Mutex::new(repository),
        })
    }

    pub fn get_day(
        &self,
        local_date: &str,
        current_local_date: &str,
    ) -> Result<DailyRecordView, String> {
        validate_mutation_date(local_date, current_local_date)?;
        self.repository()?.day_view(local_date)
    }

    pub fn set_task_checked(
        &self,
        local_date: &str,
        task_id: TaskId,
        checked: bool,
        current_local_date: &str,
        updated_at_utc: &str,
    ) -> Result<(), String> {
        self.repository()?.set_task_checked(
            local_date,
            task_id,
            checked,
            current_local_date,
            updated_at_utc,
        )
    }

    pub fn list_calendar_days(&self) -> Result<Vec<CalendarDay>, String> {
        self.repository()?.calendar_days()
    }

    pub fn export_state(&self) -> Result<String, String> {
        self.repository()?.export_state()
    }

    pub fn import_state(&self, serialized: &str, current_local_date: &str) -> Result<(), String> {
        self.repository()?
            .import_state(serialized, current_local_date)
    }

    fn repository(&self) -> Result<std::sync::MutexGuard<'_, Repository>, String> {
        self.repository
            .lock()
            .map_err(|_| "habit service repository lock is unavailable".to_owned())
    }
}
