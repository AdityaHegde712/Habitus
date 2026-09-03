use std::path::PathBuf;

use rusqlite::{params, Connection, OptionalExtension};

use super::transfer::FullState;

pub struct Repository {
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

        Ok(Self { connection })
    }

    pub fn seed(&self, state: FullState) -> Result<(), String> {
        let serialized = serde_json::to_string(&state)
            .map_err(|error| format!("unable to serialize application state: {error}"))?;
        self.connection
            .execute(
                "
                INSERT INTO application_state (id, serialized_state)
                VALUES (1, ?1)
                ON CONFLICT(id) DO UPDATE SET serialized_state = excluded.serialized_state
                ",
                params![serialized],
            )
            .map_err(|error| format!("unable to persist application state: {error}"))?;
        Ok(())
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
        self.seed(state)
    }
}
