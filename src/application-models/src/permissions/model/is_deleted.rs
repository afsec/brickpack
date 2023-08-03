use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionIsDeleted(bool);
impl From<bool> for PermissionIsDeleted {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Default for PermissionIsDeleted {
    fn default() -> Self {
        // * PermissionIsDeleted(false) -> False
        Self(false)
    }
}

impl PermissionIsDeleted {
    pub fn get(&self) -> bool {
        self.0
    }
    pub fn validate(&self) -> design_scaffold::Result<()> {
        // Just a boolean
        Ok(())
    }
}
