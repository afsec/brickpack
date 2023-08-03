use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::permissions::model::PermissionOid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPermission(String);

impl UserPermission {
    pub fn get(&self) -> &String {
        &self.0
    }
    pub fn take(self) -> String {
        self.0
    }
}

impl PartialEq for UserPermission {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for UserPermission {}

impl Deref for UserPermission {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PermissionOid> for UserPermission {
    fn from(department_oid: PermissionOid) -> Self {
        Self(department_oid.take())
    }
}

impl Into<PermissionOid> for UserPermission {
    fn into(self) -> PermissionOid {
        PermissionOid::from(self.take())
    }
}

// SQLx record type
impl From<String> for UserPermission {
    fn from(value: String) -> Self {
        Self(value)
    }
}
#[async_trait]
impl DataValidator for UserPermission {
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
