use application_models::applets::AppletsLength;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(AppletsLength);
impl ModelToViewMessage {
    pub(super) fn new(data: AppletsLength) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> AppletsLength {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = AppletsLength;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
