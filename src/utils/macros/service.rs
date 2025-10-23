/// Macro to generate a basic CRUD service.
///
/// Usage:
/// ```rust
/// crud_service!(
///     ServiceName,
///     ModelType,
///     RepositoryType,
///     CreateInputType,
///     UpdateInputType,
///     "ModelName",
///     create_logic: |input| { ModelType::new(input.field1, input.field2) },
///     update_logic: |model, input| {
///         if let Some(field) = input.field { model.field = field; }
///     }
/// );
/// ```
///
/// Generates:
/// - Service struct with repository field
/// - new() constructor
/// - create() - uses provided create_logic closure
/// - get_by_id() - finds by ID or returns NotFound error
/// - get_all() - returns all records
/// - update() - uses provided update_logic closure
/// - delete() - deletes by ID
#[macro_export]
macro_rules! crud_service {
    (
        $service_name:ident,
        $model:ty,
        $repo:ty,
        $create_input:ty,
        $update_input:ty,
        $model_name:expr,
        create_logic: |$create_param:ident| $create_body:expr,
        update_logic: |$model_param:ident, $update_param:ident| $update_body:block
    ) => {
        #[derive(Clone)]
        pub struct $service_name {
            repository: $repo,
        }

        impl $service_name {
            pub fn new(repository: $repo) -> Self {
                Self { repository }
            }

            pub async fn create<T: crate::domain::Transformer<$create_input>>(
                &self,
                to_create: T,
            ) -> Result<$model, crate::domain::error::AppError> {
                let $create_param = to_create.transform()?;
                let model = $create_body;
                self.repository.create(model).await
            }

            pub async fn get_by_id(
                &self,
                id: uuid::Uuid,
            ) -> Result<$model, crate::domain::error::AppError> {
                self.repository
                    .find_by_id(id)
                    .await?
                    .ok_or_else(|| {
                        crate::domain::error::AppError::NotFound(format!(
                            "{} with id {} not found",
                            $model_name, id
                        ))
                    })
            }

            pub async fn get_all(&self) -> Result<Vec<$model>, crate::domain::error::AppError> {
                self.repository.find_all().await
            }

            pub async fn update<T: crate::domain::Transformer<$update_input>>(
                &self,
                to_update: T,
            ) -> Result<$model, crate::domain::error::AppError> {
                let $update_param = to_update.transform()?;
                let mut $model_param = self.get_by_id($update_param.id).await?;

                $update_body

                $model_param.updated = chrono::Utc::now();
                self.repository.update($model_param).await
            }

            pub async fn delete(
                &self,
                id: uuid::Uuid,
            ) -> Result<(), crate::domain::error::AppError> {
                self.get_by_id(id).await?;
                if !self.repository.delete(id).await? {
                    return Err(crate::domain::error::AppError::NotFound(format!(
                        "{} with id {} not found",
                        $model_name, id
                    )));
                }
                Ok(())
            }
        }
    };
}
