use application_models::applets::AppletMetadata;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(Vec<AppletMetadata>);
impl ModelToViewMessage {
    pub(super) fn new(data: Vec<AppletMetadata>) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> Vec<AppletMetadata> {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = Vec<AppletMetadata>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
