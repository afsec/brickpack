use application_models::statuses::model::StatusOid;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(StatusOid);
impl ModelToViewMessage {
    pub(super) fn new(data: StatusOid) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> StatusOid {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = StatusOid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
