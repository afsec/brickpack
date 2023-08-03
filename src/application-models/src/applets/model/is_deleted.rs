use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppletIsDeleted(bool);
impl From<bool> for AppletIsDeleted {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Default for AppletIsDeleted {
    fn default() -> Self {
        // * AppletIsDeleted(false) -> False
        Self(false)
    }
}

impl AppletIsDeleted {
    pub fn get(&self) -> bool {
        self.0
    }
    pub fn validate(&self) -> design_scaffold::Result<()> {
        // Just a boolean
        Ok(())
    }
}
