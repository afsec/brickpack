use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

use crate::departments::model::DepartmentOid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDepartment(String);
impl UserDepartment {
    pub fn get(&self) -> &String {
        &self.0
    }
    pub fn take(self) -> String {
        self.0
    }
}

impl PartialEq for UserDepartment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for UserDepartment {}

impl Deref for UserDepartment {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SQLx record type
impl From<String> for UserDepartment {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Into<DepartmentOid> for UserDepartment {
    fn into(self) -> DepartmentOid {
        DepartmentOid::from(self.0)
    }
}

impl From<DepartmentOid> for UserDepartment {
    fn from(department_oid: DepartmentOid) -> Self {
        Self(department_oid.take())
    }
}

#[async_trait]
impl DataValidator for UserDepartment {
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
