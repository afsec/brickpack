use crate::TABLE_MAX_ROW_ID;
use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct StatusId(i64);
impl StatusId {
    pub fn take(self) -> i64 {
        self.0
    }
}
impl PartialEq for StatusId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for StatusId {}

impl Deref for StatusId {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SQLx record type
impl From<i64> for StatusId {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

// Tests
impl TryFrom<String> for StatusId {
    type Error = design_scaffold::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(value.parse::<i64>()?))
    }
}

// TODO: Add comment about the aim of this Impl
impl TryFrom<u32> for StatusId {
    type Error = design_scaffold::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(Self(i64::try_from(value)?))
        } else {
            Err(design_scaffold::Error::DataConversion("StatusId should be greater than 0.".into()))
        }
    }
}

// TODO: Add comment about the aim of this Impl
impl TryFrom<i32> for StatusId {
    type Error = design_scaffold::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(Self(i64::try_from(value)?))
        } else {
            Err(design_scaffold::Error::DataConversion("StatusId should be greater than 0.".into()))
        }
    }
}

#[async_trait]
impl DataValidator for StatusId {
    async fn validate(&self) -> design_scaffold::Result<()> {
        let id = *(*self);
        if id == 0 {
            return Err(design_scaffold::Error::DataValidation(format!(
                "Invalid data [{id}] == `0`",
            )));
        }

        if id < 0 {
            return Err(design_scaffold::Error::DataValidation(format!(
                "Invalid data [{id}] < `0`",
            )));
        }

        if id > TABLE_MAX_ROW_ID.into() {
            return Err(design_scaffold::Error::DataValidation(format!(
                "Invalid data [{id}] > {TABLE_MAX_ROW_ID}",
            )));
        }
        Ok(())
    }
}
