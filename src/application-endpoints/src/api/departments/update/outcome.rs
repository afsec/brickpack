use application_models::departments::DepartmentToUpdate;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(DepartmentToUpdate);
impl ModelToViewMessage {
    pub(super) fn new(data: DepartmentToUpdate) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> DepartmentToUpdate {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = DepartmentToUpdate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
