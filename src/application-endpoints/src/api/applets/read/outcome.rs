use application_models::applets::Applet;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(Applet);
impl ModelToViewMessage {
    pub(super) fn new(data: Applet) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> Applet {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = Applet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
