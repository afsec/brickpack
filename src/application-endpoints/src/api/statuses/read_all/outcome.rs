use application_models::statuses::Status;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(Vec<Status>);
impl ModelToViewMessage {
    pub(super) fn new(data: Vec<Status>) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> Vec<Status> {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = Vec<Status>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
