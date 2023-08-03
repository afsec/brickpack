pub mod model;
mod repo;

use async_trait::async_trait;
use design_scaffold::validators::DataValidator;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::FromRow;
use std::ops::Deref;

pub use repo::AppletsRepo;

// * AppletsLength
// TODO: Review and add comments
#[derive(Debug, Serialize, Deserialize)]
pub struct AppletsLength(u32);

impl AppletsLength {
    pub fn take(self) -> u32 {
        self.0
    }
}
impl From<u32> for AppletsLength {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl Deref for AppletsLength {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// SQLx SQLite `SELECT COUNT(*)` returns i32;
impl TryFrom<i32> for AppletsLength {
    type Error = design_scaffold::Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(Self(u32::try_from(value)?))
    }
}

// * Applet
#[derive(Debug, Serialize, Deserialize)]
pub struct Applet {
    oid: model::AppletOid,
    filename: model::AppletFilename,
    code: model::AppletCode,
    size: model::AppletSize,
    created_at: model::AppletCreatedAt,
    updated_at: model::AppletUpdatedAt,
}
impl Applet {
    pub fn new(
        oid: model::AppletOid,
        filename: model::AppletFilename,
        code: model::AppletCode,
    ) -> Self {
        let size = code.len();
        Self {
            oid,
            filename,
            code: model::AppletCode::from(code),
            size: model::AppletSize::from(size),
            created_at: model::AppletCreatedAt::now(),
            updated_at: model::AppletUpdatedAt::now(),
        }
    }
    pub fn get_oid(&self) -> &model::AppletOid {
        &self.oid
    }
    pub fn get_filename(&self) -> &model::AppletFilename {
        &self.filename
    }
    pub fn get_code(&self) -> &model::AppletCode {
        &self.code
    }
    pub fn take(
        self,
    ) -> (
        model::AppletOid,
        model::AppletFilename,
        model::AppletCode,
        model::AppletSize,
        model::AppletCreatedAt,
        model::AppletUpdatedAt,
    ) {
        let Self { oid, filename, code, size, created_at, updated_at } = self;
        (oid, filename, code, size, created_at, updated_at)
    }
}

// * AppletMetadata
#[derive(Debug, Serialize, Deserialize)]
pub struct AppletMetadata {
    oid: model::AppletOid,
    filename: model::AppletFilename,
    size: model::AppletSize,
    created_at: model::AppletCreatedAt,
    updated_at: model::AppletUpdatedAt,
}

// * AppletMetadataFromSqlx
#[derive(Debug, FromRow)]
pub struct AppletMetadataFromSqlx {
    oid: String,
    filename: String,
    size: i64,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
impl Into<AppletMetadata> for AppletMetadataFromSqlx {
    fn into(self) -> AppletMetadata {
        AppletMetadata {
            oid: model::AppletOid::from(self.oid),
            filename: model::AppletFilename::from(self.filename),
            size: model::AppletSize::from(self.size),
            created_at: model::AppletCreatedAt::from(self.created_at),
            updated_at: model::AppletUpdatedAt::from(self.updated_at),
        }
    }
}

// * NewApplet
#[derive(Debug, Serialize, Deserialize)]
pub struct NewApplet {
    filename: model::AppletFilename,
    code: model::AppletCode,
}
impl NewApplet {
    pub fn get_code(&self) -> &model::AppletCode {
        &self.code
    }
}
impl From<(model::AppletFilename, model::AppletCode)> for NewApplet {
    fn from((filename, code): (model::AppletFilename, model::AppletCode)) -> Self {
        Self { filename, code }
    }
}

#[async_trait]
impl DataValidator for NewApplet {
    async fn validate(&self) -> design_scaffold::Result<()> {
        self.filename.validate().await?;
        self.code.validate().await?;
        Ok(())
    }
}

// * AppletToUpdate
#[derive(Debug, Serialize, Deserialize)]
pub struct AppletToUpdate {
    #[serde(default, skip_deserializing)]
    oid: Option<model::AppletOid>,
    filename: Option<model::AppletFilename>,
    code: Option<model::AppletCode>,
}
impl AppletToUpdate {
    pub fn get_oid(&self) -> &Option<model::AppletOid> {
        &self.oid
    }
    pub fn set_oid(&mut self, applet_oid: model::AppletOid) {
        self.oid = Some(applet_oid);
    }
    pub fn get_code(&self) -> &Option<model::AppletCode> {
        &self.code
    }
    pub fn take(
        self,
    ) -> (Option<model::AppletOid>, Option<model::AppletFilename>, Option<model::AppletCode>) {
        (self.oid, self.filename, self.code)
    }
}
#[async_trait]
impl DataValidator for AppletToUpdate {
    async fn validate(&self) -> design_scaffold::Result<()> {
        if let Some(applet_oid) = &self.oid {
            applet_oid.validate().await?;
        }

        if let Some(applet_filename) = &self.filename {
            applet_filename.validate().await?;
        }

        if let Some(applet_code) = &self.code {
            applet_code.validate().await?;
        }

        Ok(())
    }
}
