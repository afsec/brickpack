use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserIsDeleted(bool);
impl From<bool> for UserIsDeleted {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Default for UserIsDeleted {
    fn default() -> Self {
        // * UserIsDeleted(false) -> False
        Self(false)
    }
}

impl UserIsDeleted {
    pub fn get(&self) -> bool {
        self.0
    }
    pub fn validate(&self) -> design_scaffold::Result<()> {
        // Just a boolean
        Ok(())
    }
}
