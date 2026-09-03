use std::{cell::RefCell, path::PathBuf};

use rusqlite::{params, Connection, OptionalExtension};

use crate::domain::task_policy::TaskId;

use super::{
    backup_recovery::BackupStore,
    transfer::{CalendarDay, DailyRecordView, FullState},
};

pub struct Repository {
    backups: RefCell<BackupStore>,
    connection: Connection,
}

impl Repository {
    pub fn open(directory: PathBuf) -> Result<Self, String> {
        std::fs::create_dir_all(&directory)
            .map_err(|error| format!("unable to create repository directory: {error}"))?;
        let connection = Connection::open(directory.join("habit-tracker.sqlite3"))
            .map_err(|error| format!("unable to open SQLite repository: {error}"))?;

        connection
            .execute_batch(
                "
                CREATE TABLE IF NOT EXISTS application_state (
                    id INTEGER PRIMARY KEY CHECK (id = 1),
                    serialized_state TEXT NOT NULL
                );
                ",
            )
            .map_err(|error| format!("unable to initialize SQLite schema: {error}"))?;

        let backups = BackupStore::new(directory.join("backups"))?;

        Ok(Self {
            backups: RefCell::new(backups),
            connection,
        })
    }

    pub fn seed(&self, state: FullState) -> Result<(), String> {
        let serialized = serde_json::to_string(&state)
            .map_err(|error| format!("unable to serialize application state: {error}"))?;
        self.persist_state(&serialized)
    }

    pub fn export_state(&self) -> Result<String, String> {
        let serialized = self
            .connection
            .query_row(
                "SELECT serialized_state FROM application_state WHERE id = 1",
                [],
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|error| format!("unable to read application state: {error}"))?;

        match serialized {
            Some(state) => Ok(state),
            None => serde_json::to_string(&FullState::empty())
                .map_err(|error| format!("unable to serialize empty application state: {error}")),
        }
    }

    pub fn import_state(&self, serialized: &str, current_local_date: &str) -> Result<(), String> {
        let state: FullState = serde_json::from_str(serialized)
            .map_err(|error| format!("import is not a valid full-state document: {error}"))?;
        state.validate(current_local_date)?;
        self.replace_validated_state(state)
    }

    pub fn previous_backup_state(&self, position: u8) -> Result<FullState, String> {
        self.backups.borrow().previous_state(position)
    }

    pub fn day_view(&self, local_date: &str) -> Result<DailyRecordView, String> {
        self.load_state()?.day_view(local_date)
    }

    pub fn calendar_days(&self) -> Result<Vec<CalendarDay>, String> {
        Ok(self.load_state()?.calendar_days())
    }

    pub fn set_task_checked(
        &self,
        local_date: &str,
        task_id: TaskId,
        checked: bool,
        current_local_date: &str,
        updated_at_utc: &str,
    ) -> Result<(), String> {
        let mut state = self.load_state()?;
        state.set_task_checked(
            local_date,
            task_id,
            checked,
            current_local_date,
            updated_at_utc,
        )?;
        state.validate(current_local_date)?;
        self.replace_validated_state(state)
    }

    fn replace_validated_state(&self, state: FullState) -> Result<(), String> {
        let previous_state = self.load_state()?;
        self.backups
            .borrow_mut()
            .stage_pre_change(&previous_state)?;

        let serialized = serde_json::to_string(&state)
            .map_err(|error| format!("unable to serialize application state: {error}"))?;
        self.persist_state(&serialized)?;
        self.backups.borrow().promote_staged_pre_change()
    }

    fn load_state(&self) -> Result<FullState, String> {
        let serialized = self.export_state()?;
        serde_json::from_str(&serialized)
            .map_err(|error| format!("stored application state is invalid JSON: {error}"))
    }

    fn persist_state(&self, serialized: &str) -> Result<(), String> {
        let transaction = self
            .connection
            .unchecked_transaction()
            .map_err(|error| format!("unable to begin application-state transaction: {error}"))?;
        transaction
            .execute(
                "
                INSERT INTO application_state (id, serialized_state)
                VALUES (1, ?1)
                ON CONFLICT(id) DO UPDATE SET serialized_state = excluded.serialized_state
                ",
                params![serialized],
            )
            .map_err(|error| format!("unable to persist application state: {error}"))?;
        transaction
            .commit()
            .map_err(|error| format!("unable to commit application-state transaction: {error}"))
    }
}
