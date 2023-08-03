pub mod model;
mod repo;

use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use std::ops::Deref;

pub use repo::DepartmentsRepo;
use sqlx::FromRow;

// * DepartmentsLength
// TODO: Review and add comments
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentsLength(u32);

impl DepartmentsLength {
    pub fn take(self) -> u32 {
        self.0
    }
}
impl From<u32> for DepartmentsLength {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Deref for DepartmentsLength {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SQLx SQLite `SELECT COUNT(*)` returns i32;
impl TryFrom<i32> for DepartmentsLength {
    type Error = design_scaffold::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}

// * Department
#[derive(Debug, Serialize, Deserialize)]
pub struct Department {
    oid: model::DepartmentOid,
    name: model::DepartmentName,
    created_at: model::DepartmentCreatedAt,
    updated_at: model::DepartmentUpdatedAt,
}
impl Department {
    pub fn get_oid(&self) -> &model::DepartmentOid {
        &self.oid
    }
    pub fn take(
        self,
    ) -> (
        model::DepartmentOid,
        model::DepartmentName,
        model::DepartmentCreatedAt,
        model::DepartmentUpdatedAt,
    ) {
        (self.oid, self.name, self.created_at, self.updated_at)
    }
}

// * DepartmentFromSqlx
#[derive(Debug, FromRow)]
pub struct DepartmentFromSqlx {
    oid: String,
    name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
impl Into<Department> for DepartmentFromSqlx {
    fn into(self) -> Department {
        Department {
            oid: model::DepartmentOid::from(self.oid),
            name: model::DepartmentName::from(self.name),
            created_at: model::DepartmentCreatedAt::from(self.created_at),
            updated_at: model::DepartmentUpdatedAt::from(self.updated_at),
        }
    }
}

// * NewDepartment
#[derive(Debug, Serialize, Deserialize)]
pub struct NewDepartment {
    name: model::DepartmentName,
}
impl NewDepartment {
    pub fn new(name: model::DepartmentName) -> Self {
        Self { name }
    }
}

#[async_trait]
impl DataValidator for NewDepartment {
    async fn validate(&self) -> design_scaffold::Result<()> {
        self.name.validate().await?;
        Ok(())
    }
}

// * DepartmentToUpdate
#[derive(Debug, Serialize, Deserialize)]
pub struct DepartmentToUpdate {
    #[serde(default, skip_deserializing)]
    oid: Option<model::DepartmentOid>,
    name: Option<model::DepartmentName>,
}

impl DepartmentToUpdate {
    pub fn get_oid(&self) -> &Option<model::DepartmentOid> {
        &self.oid
    }
    pub fn set_oid(&mut self, department_oid: model::DepartmentOid) {
        self.oid = Some(department_oid);
    }
    pub fn take(self) -> (Option<model::DepartmentOid>, Option<model::DepartmentName>) {
        (self.oid, self.name)
    }
}

#[async_trait]
impl DataValidator for DepartmentToUpdate {
    async fn validate(&self) -> design_scaffold::Result<()> {
        if let Some(department_oid) = &self.oid {
            department_oid.validate().await?;
        }

        if let Some(department_name) = &self.name {
            department_name.validate().await?;
        }

        Ok(())
    }
}
