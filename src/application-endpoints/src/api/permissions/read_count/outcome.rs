use application_models::permissions::PermissionsLength;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(PermissionsLength);
impl ModelToViewMessage {
    pub(super) fn new(data: PermissionsLength) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> PermissionsLength {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = PermissionsLength;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
