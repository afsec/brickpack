use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusIsDeleted(bool);
impl From<bool> for StatusIsDeleted {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Default for StatusIsDeleted {
    fn default() -> Self {
        // * StatusIsDeleted(false) -> False
        Self(false)
    }
}

impl StatusIsDeleted {
    pub fn get(&self) -> bool {
        self.0
    }
    pub fn validate(&self) -> design_scaffold::Result<()> {
        // Just a boolean
        Ok(())
    }
}
