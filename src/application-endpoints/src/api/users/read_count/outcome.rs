use application_models::users::UsersLength;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(UsersLength);
impl ModelToViewMessage {
    pub(super) fn new(data: UsersLength) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> UsersLength {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = UsersLength;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
