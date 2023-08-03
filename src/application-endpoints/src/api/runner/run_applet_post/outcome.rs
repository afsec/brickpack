use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(String);
impl ModelToViewMessage {
    pub(super) fn new(data: String) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> String {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
