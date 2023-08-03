use application_models::applets::model::AppletOid;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(AppletOid);
impl ModelToViewMessage {
    pub(super) fn new(data: AppletOid) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> AppletOid {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = AppletOid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
