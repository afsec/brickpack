use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{NaiveDateTime, Utc};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct DepartmentUpdatedAt(i64);
impl DepartmentUpdatedAt {
    pub fn now() -> Self {
        Self(Utc::now().timestamp())
    }
}

impl PartialEq for DepartmentUpdatedAt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for DepartmentUpdatedAt {}

impl Deref for DepartmentUpdatedAt {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SQLx record type
impl From<NaiveDateTime> for DepartmentUpdatedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value.timestamp())
    }
}
impl Into<NaiveDateTime> for DepartmentUpdatedAt {
    fn into(self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.0, 0)
    }
}

#[async_trait]
impl DataValidator for DepartmentUpdatedAt {
    async fn validate(&self) -> design_scaffold::Result<()> {
        let timestamp = &self.0;
        if *timestamp < 0 {
            return Err(design_scaffold::Error::DataValidation(format!(
                "Invalid data [{timestamp}] < `0`",
            )));
        }
        Ok(())
    }
}
