#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataFormat {
    Date { format: String },
    DateTime { format: String },
    UnixTimestamp,
}

pub fn parse_date(value: &str, date_format: &DataFormat) -> Option<chrono::DateTime<chrono::Utc>> {
    let value = value.trim();

    match date_format {
        DataFormat::Date { format } => chrono::NaiveDate::parse_from_str(value, format)
            .ok()
            .and_then(|date| date.and_hms_opt(0, 0, 0))
            .map(|date| date.and_utc()),
        DataFormat::DateTime { format } => chrono::DateTime::parse_from_str(value, format)
            .map(|date| date.with_timezone(&chrono::Utc))
            .or_else(|_| {
                chrono::NaiveDateTime::parse_from_str(value, format).map(|date| date.and_utc())
            })
            .ok(),
        DataFormat::UnixTimestamp => value.parse::<i64>().ok().and_then(|timestamp| {
            if value.len() > 10 {
                chrono::DateTime::from_timestamp_millis(timestamp)
            } else {
                chrono::DateTime::from_timestamp(timestamp, 0)
            }
        }),
    }
}
