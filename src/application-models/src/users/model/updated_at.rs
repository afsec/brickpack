use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{NaiveDateTime, Utc};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserUpdatedAt(i64);
impl UserUpdatedAt {
    pub fn now() -> Self {
        Self(Utc::now().timestamp())
    }
}

impl PartialEq for UserUpdatedAt {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for UserUpdatedAt {}

impl Deref for UserUpdatedAt {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SQLx record type
impl From<NaiveDateTime> for UserUpdatedAt {
    fn from(value: NaiveDateTime) -> Self {
        Self(value.timestamp())
    }
}
impl Into<NaiveDateTime> for UserUpdatedAt {
    fn into(self) -> NaiveDateTime {
        NaiveDateTime::from_timestamp(self.0, 0)
    }
}

#[async_trait]
impl DataValidator for UserUpdatedAt {
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
