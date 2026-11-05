pub async fn get_session() -> Result<String, String> {
    Ok("session-123".to_string())
}

pub async fn logout() -> Result<(), String> {
    Ok(())
}
