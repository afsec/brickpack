use application_models::statuses::StatusesLength;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(StatusesLength);
impl ModelToViewMessage {
    pub(super) fn new(data: StatusesLength) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> StatusesLength {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = StatusesLength;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
