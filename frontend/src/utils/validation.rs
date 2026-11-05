// frontend/src/utils/validation.rs

/// Validate Egyptian phone number
pub fn validate_phone(phone: &str) -> bool {
    // Egyptian phone: +20 or 0 followed by 10 digits
    let phone = phone.trim();
    if phone.starts_with("+20") {
        phone.len() == 13 && phone[3..].chars().all(|c| c.is_numeric())
    } else if phone.starts_with("0") {
        phone.len() == 11 && phone.chars().all(|c| c.is_numeric())
    } else {
        false
    }
}

/// Validate email
pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

/// Validate password per D-111 (12+ chars)
pub fn validate_password(password: &str) -> bool {
    password.len() >= 12
}
