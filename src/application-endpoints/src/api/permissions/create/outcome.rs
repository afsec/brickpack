use application_models::permissions::model::PermissionOid;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(PermissionOid);
impl ModelToViewMessage {
    pub(super) fn new(data: PermissionOid) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> PermissionOid {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = PermissionOid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
