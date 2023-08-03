use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentIsDeleted(bool);
impl From<bool> for DepartmentIsDeleted {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl Default for DepartmentIsDeleted {
    fn default() -> Self {
        // * DepartmentIsDeleted(false) -> False
        Self(false)
    }
}

impl DepartmentIsDeleted {
    pub fn get(&self) -> bool {
        self.0
    }
    pub fn validate(&self) -> design_scaffold::Result<()> {
        // Just a boolean
        Ok(())
    }
}
