use crate::{Value, error::JuizError};





pub fn type_name(manifest: &Value) -> Result<&str, JuizError> {
    match manifest.as_object() {
        None => return Err(JuizError::ManifestIsNotObjectError{}),
        Some(obj) => {
            match obj.get("type_name") {
                None => return Err(JuizError::ManifestDoesNotIncludeTypeNameError{}),
                Some(v) => {
                    match v.as_str() {
                        None => return Err(JuizError::ManifestTypeNameIsNotStringError{}),
                        Some(s) => return Ok(s)
                    }
                }
            }
        }
    }
}