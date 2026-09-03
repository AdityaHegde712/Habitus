use chrono::NaiveDate;

pub fn validate_mutation_date(local_date: &str, current_local_date: &str) -> Result<(), String> {
    let target_date = NaiveDate::parse_from_str(local_date, "%Y-%m-%d")
        .map_err(|error| format!("invalid local date {local_date}: {error}"))?;
    let today = NaiveDate::parse_from_str(current_local_date, "%Y-%m-%d")
        .map_err(|error| format!("invalid current local date {current_local_date}: {error}"))?;

    if target_date > today {
        return Err("future dates cannot be changed".to_owned());
    }

    Ok(())
}
