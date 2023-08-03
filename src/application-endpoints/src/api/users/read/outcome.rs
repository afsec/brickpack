use application_models::users::User;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(User);
impl ModelToViewMessage {
    pub(super) fn new(data: User) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> User {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
