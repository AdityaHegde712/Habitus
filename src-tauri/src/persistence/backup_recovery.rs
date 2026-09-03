use std::{fs, path::PathBuf};

use super::transfer::FullState;

pub struct BackupStore {
    directory: PathBuf,
}

impl BackupStore {
    pub fn new(directory: PathBuf) -> Result<Self, String> {
        fs::create_dir_all(&directory)
            .map_err(|error| format!("unable to create backup directory: {error}"))?;
        let store = Self { directory };
        store.recover_interrupted_rotation()?;
        Ok(store)
    }

    pub fn capture_pre_change(&mut self, state: &FullState) -> Result<(), String> {
        self.stage_pre_change(state)?;
        self.promote_staged_pre_change()
    }

    pub fn stage_pre_change(&mut self, state: &FullState) -> Result<(), String> {
        let serialized = serde_json::to_vec(state)
            .map_err(|error| format!("unable to serialize backup state: {error}"))?;
        fs::write(self.staging_path(), serialized)
            .map_err(|error| format!("unable to stage backup state: {error}"))
    }

    pub fn previous_state(&self, position: u8) -> Result<FullState, String> {
        let path = match position {
            1 => self.previous_one_path(),
            2 => self.previous_two_path(),
            _ => return Err("backup position must be one or two".to_owned()),
        };
        let serialized =
            fs::read(path).map_err(|error| format!("unable to read backup state: {error}"))?;
        serde_json::from_slice(&serialized)
            .map_err(|error| format!("backup state is invalid JSON: {error}"))
    }

    pub fn contains_marker(&self, marker: &str) -> Result<bool, String> {
        for position in [1, 2] {
            let path = if position == 1 {
                self.previous_one_path()
            } else {
                self.previous_two_path()
            };

            if path.exists() && self.previous_state(position)?.marker() == marker {
                return Ok(true);
            }
        }

        Ok(false)
    }

    fn recover_interrupted_rotation(&self) -> Result<(), String> {
        if !self.staging_path().exists() {
            return Ok(());
        }

        self.promote_staged_pre_change()
    }

    fn promote_staged_pre_change(&self) -> Result<(), String> {
        let previous_one = self.previous_one_path();
        let previous_two = self.previous_two_path();

        if previous_one.exists() {
            fs::copy(&previous_one, &previous_two)
                .map_err(|error| format!("unable to rotate second backup: {error}"))?;
        }

        fs::rename(self.staging_path(), previous_one)
            .map_err(|error| format!("unable to promote staged backup: {error}"))
    }

    fn previous_one_path(&self) -> PathBuf {
        self.directory.join("previous-1.json")
    }

    fn previous_two_path(&self) -> PathBuf {
        self.directory.join("previous-2.json")
    }

    fn staging_path(&self) -> PathBuf {
        self.directory.join("staged-pre-change.json")
    }
}
