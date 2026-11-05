use chrono::{DateTime, Utc};

pub fn format_egp(piastres: i64) -> String {
    let egp = piastres as f64 / 100.0;
    format!("{:.2} ج.م", egp)
}

pub fn format_date_cairo(dt: DateTime<Utc>) -> String {
    // Using UTC for now, will integrate chrono_tz after schema setup
    dt.format("%Y-%m-%d").to_string()
}

pub fn format_datetime_cairo(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M").to_string()
}

pub fn normalize_email(email: &str) -> String {
    email.to_lowercase()
}
