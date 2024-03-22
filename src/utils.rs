use chrono::{Local, Utc};

/// Misc utility functions
///
///
///

/// Converts an ISO8601 string timestamp to a chrono::DateTime.
///
/// Assumes that the input string is a valid ISO8601 string, without a timezone.
/// Examples include 2024-03-19T15:18:54.058
pub fn iso8601_to_datetime(iso8601: &str) -> chrono::DateTime<chrono::Utc> {
    // Via stack exchange
    // let date_str = "2020-04-12";
    // // From string to a NaiveDate
    // let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap();
    // // Add some default time to convert it into a NaiveDateTime
    // let naive_datetime: NaiveDateTime = naive_date.and_hms(0,0,0);
    // // Add a timezone to the object to convert it into a DateTime<UTC>
    // let datetime_utc = DateTime::<Utc>::from_utc(naive_datetime, Utc);

    let naive = chrono::NaiveDateTime::parse_from_str(iso8601, "%Y-%m-%dT%H:%M:%S%.f").unwrap();
    let datetime = naive.and_utc();

    return datetime;
}

/// Convert a chrono::DateTime to a relative time string,
/// e.g. "5 minutes ago", "2 days ago", "1 year ago", "just now"
///
/// # Example
///
/// ```
/// use chrono::Utc;
/// use comind::utils::datetime_to_relative;
///
/// let now = Utc::now();
/// let five_minutes_ago = now - chrono::Duration::minutes(5);
/// let five_minutes_ago_relative = datetime_to_relative(&five_minutes_ago);
/// println!("{}", five_minutes_ago_relative);
/// ```
pub fn datetime_to_relative(datetime: &chrono::DateTime<chrono::Utc>) -> String {
    let now = chrono::Utc::now();
    let duration = now - *datetime;
    let duration = duration.num_seconds();
    if duration < 60 {
        return "just now".to_string();
    }
    if duration < 3600 {
        let minutes = duration / 60;
        return format!("{} minutes ago", minutes);
    }
    if duration < 86400 {
        let hours = duration / 3600;
        return format!("{} hours ago", hours);
    }
    if duration < 2592000 {
        let days = duration / 86400;
        return format!("{} days ago", days);
    }
    if duration < 31536000 {
        let months = duration / 2592000;
        return format!("{} months ago", months);
    }
    let years = duration / 31536000;
    return format!("{} years ago", years);
}
