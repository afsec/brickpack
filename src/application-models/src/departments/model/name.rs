use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentName(String);

impl Deref for DepartmentName {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SQLx record type
impl From<String> for DepartmentName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[async_trait]
impl DataValidator for DepartmentName {
    async fn validate(&self) -> design_scaffold::Result<()> {
        const MAX_LENGTH: usize = 512;
        let name = &*(*self);
        if name.is_empty() {
            return Err(design_scaffold::Error::DataValidation("Invalid name: It's empty".into()));
        }
        if name.len() > MAX_LENGTH {
            return Err(design_scaffold::Error::DataValidation(
                "Invalid name: More than 512 Bytes".into(),
            ));
        }

        for c in name.as_bytes() {
            if c.is_ascii_alphabetic() || c.is_ascii_whitespace() {
                ()
            } else {
                return Err(design_scaffold::Error::DataValidation(
                    "Invalid name: It has invalid characters".into(),
                ));
            }
        }
        Ok(())
    }
}
