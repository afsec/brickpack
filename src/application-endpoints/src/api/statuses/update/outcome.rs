use application_models::statuses::StatusToUpdate;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(StatusToUpdate);
impl ModelToViewMessage {
    pub(super) fn new(data: StatusToUpdate) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> StatusToUpdate {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = StatusToUpdate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
