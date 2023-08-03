use application_models::statuses::Status;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(Status);
impl ModelToViewMessage {
    pub(super) fn new(data: Status) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> Status {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = Status;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
