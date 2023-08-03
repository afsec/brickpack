use application_models::departments::model::DepartmentOid;
use design_scaffold::endpoint::Outcome;
use std::ops::Deref;

pub(super) struct ModelToViewMessage(DepartmentOid);
impl ModelToViewMessage {
    pub(super) fn take(self) -> DepartmentOid {
        self.0
    }
}

impl Deref for ModelToViewMessage {
    type Target = DepartmentOid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<DepartmentOid> for ModelToViewMessage {
    fn from(department_oid: DepartmentOid) -> Self {
        Self(department_oid)
    }
}

impl Outcome for ModelToViewMessage {}
