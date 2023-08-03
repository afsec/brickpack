use application_models::users::UserToUpdate;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(UserToUpdate);
impl ModelToViewMessage {
    pub(super) fn new(data: UserToUpdate) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> UserToUpdate {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = UserToUpdate;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
