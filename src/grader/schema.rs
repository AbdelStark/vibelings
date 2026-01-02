//! JSON Schema validation.

use jsonschema::validator_for;
use serde_json::Value;

/// Validate JSON against a schema.
/// Returns Ok(()) if valid, or a list of error messages if invalid.
pub fn validate_json(value: &Value, schema: &Value) -> Result<(), Vec<String>> {
    let validator = validator_for(schema).map_err(|e| vec![e.to_string()])?;

    let result = validator.validate(value);

    match result {
        Ok(_) => Ok(()),
        Err(error) => {
            let error_messages: Vec<String> = vec![error.to_string()];
            Err(error_messages)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_valid_schema() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" },
                "age": { "type": "integer" }
            },
            "required": ["name"]
        });

        let valid_data = json!({
            "name": "Alice",
            "age": 30
        });

        assert!(validate_json(&valid_data, &schema).is_ok());
    }

    #[test]
    fn test_invalid_schema() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" }
            },
            "required": ["name"]
        });

        let invalid_data = json!({
            "age": 30
        });

        let result = validate_json(&invalid_data, &schema);
        assert!(result.is_err());
    }

    #[test]
    fn test_wrong_type() {
        let schema = json!({
            "type": "object",
            "properties": {
                "age": { "type": "integer" }
            }
        });

        let invalid_data = json!({
            "age": "thirty"
        });

        let result = validate_json(&invalid_data, &schema);
        assert!(result.is_err());
    }
}
