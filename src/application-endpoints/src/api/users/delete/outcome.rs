use application_models::users::model::UserOid;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(UserOid);
impl ModelToViewMessage {
    pub(super) fn new(data: UserOid) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> UserOid {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = UserOid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
