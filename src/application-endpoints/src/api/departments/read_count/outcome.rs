use application_models::departments::DepartmentsLength;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(DepartmentsLength);
impl ModelToViewMessage {
    pub(super) fn new(data: DepartmentsLength) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> DepartmentsLength {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = DepartmentsLength;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
