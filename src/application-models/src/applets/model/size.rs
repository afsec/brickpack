use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppletSize(u64);

impl PartialEq for AppletSize {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for AppletSize {}

impl Deref for AppletSize {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SQLx record type
impl From<i64> for AppletSize {
    fn from(value: i64) -> Self {
        Self(u64::try_from(value).unwrap_or_default())
    }
}
impl Into<i64> for AppletSize {
    fn into(self) -> i64 {
        i64::try_from(self.0).unwrap_or_default()
    }
}
// String.len()
impl From<usize> for AppletSize {
    fn from(value: usize) -> Self {
        Self(u64::try_from(value).unwrap_or_default())
    }
}

// TODO: Add comment about the aim of this Impl
impl TryFrom<u32> for AppletSize {
    type Error = design_scaffold::Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(Self(u64::try_from(value)?))
        } else {
            Err(design_scaffold::Error::DataConversion(
                "AppletSize should be greater than 0.".into(),
            ))
        }
    }
}

// TODO: Add comment about the aim of this Impl
impl TryFrom<i32> for AppletSize {
    type Error = design_scaffold::Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(Self(u64::try_from(value)?))
        } else {
            Err(design_scaffold::Error::DataConversion(
                "AppletSize should be greater than 0.".into(),
            ))
        }
    }
}

#[async_trait]
impl DataValidator for AppletSize {
    async fn validate(&self) -> design_scaffold::Result<()> {
        let id = *(*self);
        if id == 0 {
            return Err(design_scaffold::Error::DataValidation(format!(
                "Invalid data [{:?}] == `0`",
                id
            )));
        }

        Ok(())
    }
}
