use application_models::departments::Department;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(Department);
impl ModelToViewMessage {
    pub(super) fn new(data: Department) -> Self {
        Self(data)
    }
    pub(super) fn take(self) -> Department {
        self.0
    }
}
impl Deref for ModelToViewMessage {
    type Target = Department;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Outcome for ModelToViewMessage {}
