use super::outcome::ModelToViewMessage;
use super::CreateUser;
use application_models::departments::DepartmentsRepo;
use application_models::permissions::PermissionsRepo;
use application_models::statuses::StatusesRepo;
use application_models::users::model::{UserDepartment, UserPermission, UserStatus};
use application_models::users::{NewUser, UsersRepo};
use async_trait::async_trait;
use design_scaffold::endpoint::Model;
use design_scaffold::oid::OidPool;
use design_scaffold::repo::Repository;
use sqlx::sqlite::Sqlite;

#[async_trait]
impl<'endpoint> Model<'endpoint, Sqlite, NewUser, ModelToViewMessage> for CreateUser {
    async fn model(
        &'endpoint self,
        option_oid_pool: Option<&OidPool>,
        db_conn_pool: &sqlx::Pool<Sqlite>,
        new_user: NewUser,
    ) -> design_scaffold::Result<ModelToViewMessage> {
        let (user_name, user_email, user_department, user_permission, user_status) =
            new_user.take();

        // * Check if department_oid exists before create user
        let department_oid = user_department.into();
        {
            let department_data = DepartmentsRepo::read(db_conn_pool, &department_oid).await;
            let id = &*department_oid;
            if department_data.is_err() {
                return Err(design_scaffold::Error::EntityIdNotFound(format!(
                    "User department id [{id}] not exists"
                )));
            }
        }

        // * Check if permission_oid exists before create user
        let permission_oid = user_permission.into();
        {
            let permission_data = PermissionsRepo::read(db_conn_pool, &permission_oid).await;
            let id = &*permission_oid;
            if permission_data.is_err() {
                return Err(design_scaffold::Error::EntityIdNotFound(format!(
                    "User permission id [{id}] not exists"
                )));
            }
        }

        // * Check if status_oid exists before create user
        let status_oid = user_status.into();
        {
            let status_data = StatusesRepo::read(db_conn_pool, &status_oid).await;
            let id = &*status_oid;
            if status_data.is_err() {
                return Err(design_scaffold::Error::EntityIdNotFound(format!(
                    "User status id [{id}] not exists"
                )));
            }
        }

        let new_user = NewUser::from((
            user_name,
            user_email,
            UserDepartment::from(department_oid),
            UserPermission::from(permission_oid),
            UserStatus::from(status_oid),
        ));

        let outcome_data = UsersRepo::create(option_oid_pool, db_conn_pool, &new_user).await?;

        Ok(ModelToViewMessage::new(outcome_data))
    }
}
