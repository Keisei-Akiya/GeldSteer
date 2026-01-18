use validator::ValidationError;

pub fn validate_non_blank(value: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        return Err(ValidationError::new("non_blank"));
    }
    Ok(())
}
