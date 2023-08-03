use crate::statuses::model::StatusOid;
use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserStatus(String);
// UserStatus(0) -> unused

// UserStatus(1) -> Activated
// UserStatus(2) -> Deactivated
// UserStatus(3) -> Locked

impl From<StatusOid> for UserStatus {
    fn from(status_oid: StatusOid) -> Self {
        Self(status_oid.take())
    }
}

impl Into<StatusOid> for UserStatus {
    fn into(self) -> StatusOid {
        StatusOid::from(self.0)
    }
}

impl Deref for UserStatus {
    type Target = String;
    fn deref(&self) -> &<Self as Deref>::Target {
        &self.0
    }
}

// SQLx record type
impl From<String> for UserStatus {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[async_trait]
impl DataValidator for UserStatus {
    async fn validate(&self) -> design_scaffold::Result<()> {
        const SHA256_FIXED_LENGTH: usize = 64;
        // TODO: Check algorithm and create tests to validate `sha256` string
        let department_oid = &self.0;
        if department_oid.is_empty() {
            return Err(design_scaffold::Error::DataValidation("Invalid oid: It's empty".into()));
        }
        if department_oid.len() != SHA256_FIXED_LENGTH {
            return Err(design_scaffold::Error::DataValidation(format!(
                "Invalid oid: More than {} Bytes",
                SHA256_FIXED_LENGTH
            )));
        }

        // * Check if is a valid ascii string
        for c in department_oid.chars() {
            if !c.is_ascii() {
                return Err(design_scaffold::Error::DataValidation(
                    "Invalid oid: Must have ONLY ascii alphanumeric characters".into(),
                ));
            }

            if !(c.is_ascii_lowercase() || c.is_ascii_digit()) {
                return Err(design_scaffold::Error::DataValidation(
                    "Invalid oid: It has invalid characters".into(),
                ));
            }
        }

        Ok(())
    }
}
