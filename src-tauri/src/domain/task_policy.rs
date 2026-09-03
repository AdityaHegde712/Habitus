use chrono::{Datelike, NaiveDate, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum TaskId {
    Meals,
    Sleep7h,
    Exercise,
    JobApplication,
    Vitamins,
    LeetcodeOrDsa,
    SurfacesClean,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub const fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

pub fn applicable_task_ids(local_date: &str) -> Result<Vec<TaskId>, String> {
    let date = NaiveDate::parse_from_str(local_date, "%Y-%m-%d")
        .map_err(|error| format!("invalid local date {local_date}: {error}"))?;
    let mut task_ids = vec![TaskId::Meals, TaskId::Sleep7h];

    if matches!(date.weekday(), Weekday::Mon | Weekday::Wed | Weekday::Fri) {
        task_ids.push(TaskId::Exercise);
    }

    task_ids.extend([
        TaskId::JobApplication,
        TaskId::Vitamins,
        TaskId::LeetcodeOrDsa,
        TaskId::SurfacesClean,
    ]);
    Ok(task_ids)
}

pub fn completion_color(completed_count: u32, applicable_count: u32) -> Result<Rgb, String> {
    if applicable_count == 0 {
        return Err("applicable count must be positive".to_owned());
    }

    if completed_count > applicable_count {
        return Err("completed count cannot exceed applicable count".to_owned());
    }

    let green = (255.0 * completed_count as f64 / applicable_count as f64).round() as u8;
    Ok(Rgb::new(0, green, 0))
}
