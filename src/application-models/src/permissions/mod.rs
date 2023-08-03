pub mod model;
mod repo;

use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use std::ops::Deref;

pub use repo::PermissionsRepo;
use sqlx::FromRow;

// * PermissionsLength
// TODO: Review and add comments
#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionsLength(u32);

impl PermissionsLength {
    pub fn take(self) -> u32 {
        self.0
    }
}
impl From<u32> for PermissionsLength {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Deref for PermissionsLength {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SQLx SQLite `SELECT COUNT(*)` returns i32;
impl TryFrom<i32> for PermissionsLength {
    type Error = design_scaffold::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}

// * Permission
#[derive(Debug, Serialize, Deserialize)]
pub struct Permission {
    oid: model::PermissionOid,
    name: model::PermissionName,
    created_at: model::PermissionCreatedAt,
    updated_at: model::PermissionUpdatedAt,
}
impl Permission {
    pub fn get_oid(&self) -> &model::PermissionOid {
        &self.oid
    }
    pub fn take(
        self,
    ) -> (
        model::PermissionOid,
        model::PermissionName,
        model::PermissionCreatedAt,
        model::PermissionUpdatedAt,
    ) {
        (self.oid, self.name, self.created_at, self.updated_at)
    }
}

// * PermissionFromSqlx
#[derive(Debug, FromRow)]
pub struct PermissionFromSqlx {
    oid: String,
    name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
impl Into<Permission> for PermissionFromSqlx {
    fn into(self) -> Permission {
        Permission {
            oid: model::PermissionOid::from(self.oid),
            name: model::PermissionName::from(self.name),
            created_at: model::PermissionCreatedAt::from(self.created_at),
            updated_at: model::PermissionUpdatedAt::from(self.updated_at),
        }
    }
}

// * NewPermission
#[derive(Debug, Serialize, Deserialize)]
pub struct NewPermission {
    name: model::PermissionName,
}
impl NewPermission {
    pub fn new(name: model::PermissionName) -> Self {
        Self { name }
    }
}

#[async_trait]
impl DataValidator for NewPermission {
    async fn validate(&self) -> design_scaffold::Result<()> {
        self.name.validate().await?;
        Ok(())
    }
}

// * PermissionToUpdate
#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionToUpdate {
    #[serde(default, skip_deserializing)]
    oid: Option<model::PermissionOid>,
    name: Option<model::PermissionName>,
}
impl PermissionToUpdate {
    pub fn get_oid(&self) -> &Option<model::PermissionOid> {
        &self.oid
    }
    pub fn set_oid(&mut self, permission_oid: model::PermissionOid) {
        self.oid = Some(permission_oid);
    }
    pub fn take(self) -> (Option<model::PermissionOid>, Option<model::PermissionName>) {
        (self.oid, self.name)
    }
}
#[async_trait]
impl DataValidator for PermissionToUpdate {
    async fn validate(&self) -> design_scaffold::Result<()> {
        if let Some(permission_oid) = &self.oid {
            permission_oid.validate().await?;
        }

        if let Some(permission_name) = &self.name {
            permission_name.validate().await?;
        }

        Ok(())
    }
}
