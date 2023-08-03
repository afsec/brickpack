use application_models::users::User;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(Vec<User>);
impl ModelToViewMessage {
    pub(super) fn new(data: Vec<User>) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> Vec<User> {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = Vec<User>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
