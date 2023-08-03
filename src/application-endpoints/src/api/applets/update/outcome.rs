use application_models::applets::AppletToUpdate;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(AppletToUpdate);
impl ModelToViewMessage {
    pub(super) fn new(data: AppletToUpdate) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> AppletToUpdate {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = AppletToUpdate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
