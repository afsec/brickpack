use design_scaffold::{oid::OidPool, repo::Repository, validators::DataValidator};
use sqlx::{ConnectOptions, SqlitePool};
use tracing::log::LevelFilter;

use crate::applets::{
    model::{AppletCode, AppletFilename},
    AppletsRepo, NewApplet,
};

pub struct DatabaseConnection(SqlitePool);
impl DatabaseConnection {
    pub async fn new(
        option_oid_pool: Option<&OidPool>,
        filename_path: &str,
    ) -> design_scaffold::Result<Self> {
        use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
        use std::str::FromStr;
        use std::time::Duration;

        let busy_timeout = Duration::from_secs(2);

        let sqlite_path = if filename_path == ":memory:" {
            format!("sqlite:{}", filename_path)
        } else {
            format!("sqlite://{}", filename_path)
        };

        let mut connect_options = SqliteConnectOptions::from_str(sqlite_path.as_str())?
            .busy_timeout(busy_timeout)
            // Why we set to `Delete`: https://www.sqlite.org/pragma.html#pragma_journal_mode
            // > "The DELETE journaling mode is the normal behavior".
            .journal_mode(SqliteJournalMode::Delete)
            .create_if_missing(true);
        connect_options.log_statements(LevelFilter::Debug);

        tracing::debug!("Database file opened at {}", filename_path);

        let db_conn_pool =
            match SqlitePoolOptions::new().max_connections(5).connect_with(connect_options).await {
                Ok(pool) => {
                    tracing::debug!("Database connection Ok");
                    pool
                }
                Err(error) => {
                    tracing::error!("Database connection is not possible! Reason: {error:?}");
                    panic!("Impossible state reached!")
                }
            };

        // * Bootstraping
        if let Err(bootstrap_error) =
            DatabaseConnection::db_bootstrap(option_oid_pool, &db_conn_pool).await
        {
            tracing::error!("Database bootstraping is not possible! Reason: {bootstrap_error:?}");
            return Err(design_scaffold::Error::from(bootstrap_error));
        }

        Ok(Self(db_conn_pool))
    }
    async fn db_bootstrap(
        option_oid_pool: Option<&OidPool>,
        db_conn_pool: &SqlitePool,
    ) -> design_scaffold::Result<()> {
        tracing::info!("Bootstraping database...");

        create_tables(db_conn_pool).await?;
        populate_tables(option_oid_pool, db_conn_pool).await?;

        tracing::info!("Database bootstrapped!");
        Ok(())
    }

    pub fn take(self) -> SqlitePool {
        self.0
    }
}

async fn create_tables(db_conn_pool: &SqlitePool) -> design_scaffold::Result<()> {
    let _ = sqlx::query(include_str!("../migrations/20211231034234_init.sql"))
        .execute(db_conn_pool)
        .await?;
    Ok(())
}

async fn populate_tables(
    option_oid_pool: Option<&OidPool>,
    db_conn_pool: &SqlitePool,
) -> design_scaffold::Result<()> {
    use crate::departments::{model::DepartmentName, DepartmentsRepo, NewDepartment};
    use crate::permissions::{model::PermissionName, NewPermission, PermissionsRepo};
    use crate::statuses::{model::StatusName, NewStatus, StatusesRepo};
    use crate::users::{
        model::{UserDepartment, UserEmail, UserName, UserPermission, UserStatus},
        NewUser, UsersRepo,
    };
    let statuses_count = StatusesRepo::count(db_conn_pool).await?;

    if !(*statuses_count > 0) {
        // TODO: Build some kind of results counter (?Statistics?)
        let mut total_rows_affected: usize = 0;
        let it_department_oid = DepartmentsRepo::create(
            option_oid_pool,
            db_conn_pool,
            &NewDepartment::new(DepartmentName::from("IT".to_string())),
        )
        .await?;
        total_rows_affected += 1;

        let accounting_department_oid = DepartmentsRepo::create(
            option_oid_pool,
            db_conn_pool,
            &NewDepartment::new(DepartmentName::from("Accounting".to_string())),
        )
        .await?;
        total_rows_affected += 1;

        let marketing_department_oid = DepartmentsRepo::create(
            option_oid_pool,
            db_conn_pool,
            &NewDepartment::new(DepartmentName::from("Marketing".to_string())),
        )
        .await?;
        total_rows_affected += 1;

        let admin_permission_oid = PermissionsRepo::create(
            option_oid_pool,
            db_conn_pool,
            &NewPermission::new(PermissionName::from("Administrator".to_string())),
        )
        .await?;
        total_rows_affected += 1;

        let tech_permission_oid = PermissionsRepo::create(
            option_oid_pool,
            db_conn_pool,
            &NewPermission::new(PermissionName::from("Technical".to_string())),
        )
        .await?;
        total_rows_affected += 1;

        let basic_permission_oid = PermissionsRepo::create(
            option_oid_pool,
            db_conn_pool,
            &NewPermission::new(PermissionName::from("Basic".to_string())),
        )
        .await?;
        total_rows_affected += 1;

        let activated_status_oid = StatusesRepo::create(
            option_oid_pool,
            db_conn_pool,
            &NewStatus::new(StatusName::from("Activated".to_string())),
        )
        .await?;
        total_rows_affected += 1;

        let deactivated_status_oid = StatusesRepo::create(
            option_oid_pool,
            db_conn_pool,
            &NewStatus::new(StatusName::from("Deactivated".to_string())),
        )
        .await?;
        total_rows_affected += 1;

        let locked_status_oid = StatusesRepo::create(
            option_oid_pool,
            db_conn_pool,
            &NewStatus::new(StatusName::from("Locked".to_string())),
        )
        .await?;
        total_rows_affected += 1;

        let user_oid = UsersRepo::create(
            option_oid_pool,
            db_conn_pool,
            &NewUser::from((
                UserName::from("Charlie Root".to_string()),
                UserEmail::from("root@example.net".to_string()),
                UserDepartment::from(it_department_oid),
                UserPermission::from(admin_permission_oid),
                UserStatus::from(activated_status_oid),
            )),
        )
        .await?;
        user_oid.validate().await?;
        total_rows_affected += 1;

        let user_oid = UsersRepo::create(
            option_oid_pool,
            db_conn_pool,
            &NewUser::from((
                UserName::from("Administrator".to_string()),
                UserEmail::from("admin@example.net".to_string()),
                UserDepartment::from(accounting_department_oid),
                UserPermission::from(tech_permission_oid),
                UserStatus::from(deactivated_status_oid),
            )),
        )
        .await?;
        user_oid.validate().await?;
        total_rows_affected += 1;

        let user_oid = UsersRepo::create(
            option_oid_pool,
            db_conn_pool,
            &NewUser::from((
                UserName::from("Staff".to_string()),
                UserEmail::from("staff@example.net".to_string()),
                UserDepartment::from(marketing_department_oid),
                UserPermission::from(basic_permission_oid),
                UserStatus::from(locked_status_oid),
            )),
        )
        .await?;
        user_oid.validate().await?;
        total_rows_affected += 1;

        let applet_oid = AppletsRepo::create(
            option_oid_pool,
            db_conn_pool,
            &NewApplet::from((
                AppletFilename::from("hello.lua".to_string()),
                AppletCode::from(
                    "ZWNobygiPHN0cm9uZz5IZWxsbzwvc3Ryb25nPiB3b3JsZCEiKQ==".to_string(),
                ),
            )),
        )
        .await?;
        applet_oid.validate().await?;
        total_rows_affected += 1;

        let applet_oid = AppletsRepo::create(
            option_oid_pool,
            db_conn_pool,
            &NewApplet::from((
                AppletFilename::from("fibonacci.lua".to_string()),
                AppletCode::from(
                    "ZnVuY3Rpb24gZmliKG4pCiAgICBpZiBuIDwgMiAKICAgICAgICB0aGVuIHJldHVybiAxIAogICAgICAgIGVsc2UgcmV0dXJuIGZpYihuLTIpICsgZmliKG4tMSkKICAgIGVuZAplbmQKCgpmdW5jdGlvbiBtYWluKCkKICAgIGxvY2FsIHggPSAxMAogICAgZWNobygiPGgxPkZpYm9uYWNjaTwvaDE+IikKICAgIGVjaG8oIjxwPkZpYm9uYWNjaSA8c3Ryb25nPigiKQogICAgbG9jYWwgb3V0cHV0ID0gdG9zdHJpbmcoeCkKICAgIGVjaG8ob3V0cHV0KQogICAgZWNobygiKTwvc3Ryb25nPjwvcD4iKQogICAgZWNobygiPGhyLz4iKQogICAgbG9jYWwgcmVzdWx0PWZpYih4KQogICAgZWNobygiPHA+PHN0cm9uZz5SZXN1bHQ6PC9zdHJvbmc+PHNwYW4gc3R5bGU9XCJjb2xvcjogcmVkO1wiPiIpCiAgICBsb2NhbCBvdXRwdXQgPSB0b3N0cmluZyhyZXN1bHQpCiAgICBlY2hvKG91dHB1dCkKICAgIGVjaG8oIjwvc3Bhbj48L3A+IikKZW5kCgoKbWFpbigpCg==".to_string(),
                ),
            )),
        )
        .await?;
        applet_oid.validate().await?;
        total_rows_affected += 1;

        tracing::info!("Inserted {total_rows_affected} rows into some tables");
    }
    Ok(())
}
