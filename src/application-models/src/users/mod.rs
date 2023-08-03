pub mod model;
mod repo;

use std::ops::Deref;

use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};

pub use repo::UsersRepo;
use sqlx::{types::chrono::NaiveDateTime, FromRow};

// * UsersLength
// TODO: Review and add comments
#[derive(Debug, Serialize, Deserialize)]
pub struct UsersLength(u32);

impl UsersLength {
    pub fn take(self) -> u32 {
        self.0
    }
}
impl From<u32> for UsersLength {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Deref for UsersLength {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SQLx SQLite `SELECT COUNT(*)` returns i32;
impl TryFrom<i32> for UsersLength {
    type Error = design_scaffold::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}

// * User
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    oid: model::UserOid,
    name: model::UserName,
    email: model::UserEmail,
    department: model::UserDepartment,
    permission: model::UserPermission,
    status: model::UserStatus,
    created_at: model::UserCreatedAt,
    updated_at: model::UserUpdatedAt,
}
impl User {
    pub fn get_oid(&self) -> &model::UserOid {
        &self.oid
    }
    pub fn take(
        self,
    ) -> (
        model::UserOid,
        model::UserName,
        model::UserEmail,
        model::UserDepartment,
        model::UserPermission,
        model::UserStatus,
    ) {
        (self.oid, self.name, self.email, self.department, self.permission, self.status)
    }
}

// * UserFromSqlx
#[derive(Debug, FromRow)]
pub struct UserFromSqlx {
    oid: String,
    name: String,
    email: String,
    department: String,
    permission: String,
    status: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Into<User> for UserFromSqlx {
    fn into(self) -> User {
        User {
            oid: model::UserOid::from(self.oid),
            name: model::UserName::from(self.name),
            email: model::UserEmail::from(self.email),
            department: model::UserDepartment::from(self.department),
            permission: model::UserPermission::from(self.permission),
            status: model::UserStatus::from(self.status),
            created_at: model::UserCreatedAt::from(self.created_at),
            updated_at: model::UserUpdatedAt::from(self.updated_at),
        }
    }
}

// * NewUser
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    name: model::UserName,
    email: model::UserEmail,
    department: model::UserDepartment,
    permission: model::UserPermission,
    status: model::UserStatus,
}

impl NewUser {
    pub fn take(
        self,
    ) -> (
        model::UserName,
        model::UserEmail,
        model::UserDepartment,
        model::UserPermission,
        model::UserStatus,
    ) {
        let Self { name, email, department, permission, status } = self;
        (name, email, department, permission, status)
    }
}

impl
    From<(
        model::UserName,
        model::UserEmail,
        model::UserDepartment,
        model::UserPermission,
        model::UserStatus,
    )> for NewUser
{
    fn from(
        (name, email, department, permission, status): (
            model::UserName,
            model::UserEmail,
            model::UserDepartment,
            model::UserPermission,
            model::UserStatus,
        ),
    ) -> Self {
        Self { name, email, department, permission, status }
    }
}
#[async_trait]
impl DataValidator for NewUser {
    async fn validate(&self) -> design_scaffold::Result<()> {
        self.name.validate().await?;
        self.email.validate().await?;
        self.department.validate().await?;
        self.permission.validate().await?;
        self.status.validate().await?;
        Ok(())
    }
}

// * UserToUpdate
#[derive(Debug, Serialize, Deserialize)]
pub struct UserToUpdate {
    #[serde(default, skip_deserializing)]
    oid: Option<model::UserOid>,
    name: Option<model::UserName>,
    email: Option<model::UserEmail>,
    department: Option<model::UserDepartment>,
    permission: Option<model::UserPermission>,
    status: Option<model::UserStatus>,
}
impl UserToUpdate {
    pub fn get_oid(&self) -> &Option<model::UserOid> {
        &self.oid
    }
    pub fn set_oid(&mut self, user_oid: model::UserOid) {
        self.oid = Some(user_oid);
    }
    pub fn take(
        self,
    ) -> (
        Option<model::UserOid>,
        Option<model::UserName>,
        Option<model::UserEmail>,
        Option<model::UserDepartment>,
        Option<model::UserPermission>,
        Option<model::UserStatus>,
    ) {
        (self.oid, self.name, self.email, self.department, self.permission, self.status)
    }
}
#[async_trait]
impl DataValidator for UserToUpdate {
    async fn validate(&self) -> design_scaffold::Result<()> {
        if let Some(user_oid) = &self.oid {
            user_oid.validate().await?;
        }

        if let Some(user_name) = &self.name {
            user_name.validate().await?;
        }
        if let Some(user_email) = &self.email {
            user_email.validate().await?;
        }
        if let Some(user_department) = &self.department {
            user_department.validate().await?;
        }
        if let Some(user_permission) = &self.permission {
            user_permission.validate().await?;
        }
        if let Some(user_status) = &self.status {
            user_status.validate().await?;
        }

        Ok(())
    }
}
