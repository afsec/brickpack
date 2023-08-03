use application_models::permissions::Permission;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(Permission);
impl ModelToViewMessage {
    pub(super) fn new(data: Permission) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> Permission {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = Permission;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
