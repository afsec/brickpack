use async_trait::async_trait;
use design_scaffold::oid::OidPool;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppletOid(String);

impl AppletOid {
    pub async fn new(option_oid_pool: Option<&OidPool>) -> design_scaffold::Result<Self> {
        let oid_pool = match option_oid_pool {
            Some(inner_rwlock) => inner_rwlock,
            None => {
                return Err(design_scaffold::Error::ImpossibleState("No option_oid_pool".into()))
            }
        };
        let mut oid_reactor = oid_pool.write().await;
        let new_oid = oid_reactor.generate().await?;
        Ok(Self(new_oid))
    }
    pub fn get(&self) -> &String {
        &self.0
    }
    pub fn take(self) -> String {
        self.0
    }
}

impl PartialEq for AppletOid {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for AppletOid {}

impl Deref for AppletOid {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SQLx record type
impl From<String> for AppletOid {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[async_trait]
impl DataValidator for AppletOid {
    async fn validate(&self) -> design_scaffold::Result<()> {
        const SHA256_FIXED_LENGTH: usize = 64;
        // TODO: Check algorithm and create tests to validate `sha256` string
        let applet_oid = &self.0;
        if applet_oid.is_empty() {
            return Err(design_scaffold::Error::DataValidation("Invalid oid: It's empty".into()));
        }
        if applet_oid.len() != SHA256_FIXED_LENGTH {
            return Err(design_scaffold::Error::DataValidation(format!(
                "Invalid oid: More than {} Bytes",
                SHA256_FIXED_LENGTH
            )));
        }

        // * Check if is a valid ascii string
        for c in applet_oid.chars() {
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
