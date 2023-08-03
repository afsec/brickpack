pub mod model;
mod repo;

use std::ops::Deref;

use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;

pub use repo::StatusesRepo;
use sqlx::FromRow;

// * StatusesLength
// TODO: Review and add comments
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusesLength(u32);

impl StatusesLength {
    pub fn take(self) -> u32 {
        self.0
    }
}
impl From<u32> for StatusesLength {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Deref for StatusesLength {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SQLx SQLite `SELECT COUNT(*)` returns i32;
impl TryFrom<i32> for StatusesLength {
    type Error = design_scaffold::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}

// * Status
#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    oid: model::StatusOid,
    name: model::StatusName,
    created_at: model::StatusCreatedAt,
    updated_at: model::StatusUpdatedAt,
}
impl Status {
    pub fn get_oid(&self) -> &model::StatusOid {
        &self.oid
    }
    pub fn take(
        self,
    ) -> (model::StatusOid, model::StatusName, model::StatusCreatedAt, model::StatusUpdatedAt) {
        (self.oid, self.name, self.created_at, self.updated_at)
    }
}

// * StatusFromSqlx
#[derive(Debug, FromRow)]
pub struct StatusFromSqlx {
    oid: String,
    name: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
impl Into<Status> for StatusFromSqlx {
    fn into(self) -> Status {
        Status {
            oid: model::StatusOid::from(self.oid),
            name: model::StatusName::from(self.name),
            created_at: model::StatusCreatedAt::from(self.created_at),
            updated_at: model::StatusUpdatedAt::from(self.updated_at),
        }
    }
}

// * NewStatus
#[derive(Debug, Serialize, Deserialize)]
pub struct NewStatus {
    name: model::StatusName,
}

impl NewStatus {
    pub fn new(name: model::StatusName) -> Self {
        Self { name }
    }
}

#[async_trait]
impl DataValidator for NewStatus {
    async fn validate(&self) -> design_scaffold::Result<()> {
        self.name.validate().await?;
        Ok(())
    }
}

// * StatusToUpdate
#[derive(Debug, Serialize, Deserialize)]
pub struct StatusToUpdate {
    #[serde(default, skip_deserializing)]
    oid: Option<model::StatusOid>,
    name: Option<model::StatusName>,
}

impl StatusToUpdate {
    pub fn get_oid(&self) -> &Option<model::StatusOid> {
        &self.oid
    }
    pub fn set_oid(&mut self, status_oid: model::StatusOid) {
        self.oid = Some(status_oid);
    }
    pub fn take(self) -> (Option<model::StatusOid>, Option<model::StatusName>) {
        (self.oid, self.name)
    }
}

#[async_trait]
impl DataValidator for StatusToUpdate {
    async fn validate(&self) -> design_scaffold::Result<()> {
        if let Some(status_oid) = &self.oid {
            status_oid.validate().await?;
        }

        if let Some(status_name) = &self.name {
            status_name.validate().await?;
        }

        Ok(())
    }
}
