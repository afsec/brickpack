use application_models::permissions::PermissionToUpdate;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(PermissionToUpdate);
impl ModelToViewMessage {
    pub(super) fn new(data: PermissionToUpdate) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> PermissionToUpdate {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = PermissionToUpdate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
