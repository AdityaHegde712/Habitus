use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FullState {
    marker: String,
    records: Vec<TransferRecord>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct TransferRecord {
    local_date: String,
    applicable_count: u32,
    completed_count: u32,
}

impl FullState {
    pub fn empty() -> Self {
        Self {
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
            marker: "sample".to_owned(),
            records: vec![
                TransferRecord {
                    local_date: "2026-08-31".to_owned(),
                    applicable_count: 6,
                    completed_count: 1,
                },
                TransferRecord {
                    local_date: "2026-09-01".to_owned(),
                    applicable_count: 7,
                    completed_count: 3,
                },
            ],
        }
    }

    pub fn validate(&self, current_local_date: &str) -> Result<(), String> {
        for record in &self.records {
            if record.local_date.as_str() > current_local_date {
                return Err("import contains a future record".to_owned());
            }

            if record.applicable_count == 0 || record.completed_count > record.applicable_count {
                return Err("import contains inconsistent record totals".to_owned());
            }
        }

        Ok(())
    }

    pub fn marker(&self) -> &str {
        &self.marker
    }
}
