/// Macro to generate a CRUD repository with database operations.
///
/// Generates a repository struct with PostgreSQL database operations using dynamic SQL queries.
/// Uses `sqlx::query_as` for runtime query construction, allowing flexible field configurations.
///
/// # Usage
///
/// ```rust
/// crud_repository!(
///     RepositoryName,
///     ModelType,
///     RowType,
///     "table_name",
///     insert_field1, insert_field2, ...;  // Fields for INSERT
///     select_field1, select_field2, ...;  // Fields for SELECT  
///     update_field1, update_field2, ...;  // Fields for UPDATE (exclude id, created)
/// );
/// ```
///
/// # Parameters
///
/// - **RepositoryName**: Name of the repository struct (e.g., `NoteRepository`)
/// - **ModelType**: Domain model type (e.g., `Note`)
/// - **RowType**: Database row type that implements `sqlx::FromRow` (e.g., `NoteRow`)
/// - **"table_name"**: Database table name as string literal
/// - **insert_fields**: Comma-separated fields for INSERT (include all columns)
/// - **select_fields**: Comma-separated fields for SELECT (usually same as insert)
/// - **update_fields**: Comma-separated fields for UPDATE (exclude `id` and `created`)
///
/// # Generated Methods
///
/// - `new(pool: sqlx::PgPool) -> Self` - Constructor
/// - `create<T>(model: T) -> Result<Model, AppError>` - Insert new record
/// - `find_by_id(id: Uuid) -> Result<Option<Model>, AppError>` - Find by ID
/// - `find_all() -> Result<Vec<Model>, AppError>` - Get all records (ordered by created DESC)
/// - `update<T>(model: T) -> Result<Model, AppError>` - Update existing record
/// - `delete(id: Uuid) -> Result<bool, AppError>` - Delete record, returns true if deleted
///
/// # Requirements
///
/// - `ModelType` must implement `From<RowType>`
/// - `RowType` must derive `sqlx::FromRow`
/// - `ModelType` must have an `id` field of type `uuid::Uuid`
/// - Field names must exactly match database column names
///
/// # Example
///
/// ```rust
/// use super::model::{Note, NoteRow};
/// use crate::crud_repository;
///
/// crud_repository!(
///     NoteRepository,
///     Note,
///     NoteRow,
///     "notes",
///     id, user_id, title, content, created, updated;  // INSERT
///     id, user_id, title, content, created, updated;  // SELECT
///     user_id, title, content, updated;               // UPDATE
/// );
/// ```
///
/// **Generated SQL operations:**
/// - **INSERT**: All fields including id, created, updated timestamps
/// - **SELECT**: All fields to retrieve complete records
/// - **UPDATE**: Mutable fields only (excludes id and created, includes updated)
/// - **Queries**: Dynamic SQL using `sqlx::query_as` with runtime parameter binding
#[macro_export]
macro_rules! crud_repository {
    ($repo_name:ident, $model:ty, $row:ty, $table:expr, $( $insert_field:ident ),* ; $( $select_field:ident ),* ; $( $update_field:ident ),* ; $( $enum_field:ident ),* ) => {
         #[derive(Clone)]
         pub struct $repo_name {
             pool: sqlx::PgPool,
         }

         impl $repo_name {
             pub fn new(pool: sqlx::PgPool) -> Self {
                 Self { pool }
             }

             pub async fn create<T: crate::domain::Transformer<$model>>(&self, to_model: T) -> Result<$model, crate::domain::error::AppError> {
                 let model = to_model.transform()?;

                 let placeholders: Vec<String> = (1..=vec![$( stringify!($insert_field) ),*].len()).map(|i| format!("${}", i)).collect();
                 let placeholders_str = placeholders.join(", ");

                 let query = format!(
                     "INSERT INTO {} ({}) VALUES ({}) RETURNING {}",
                     $table,
                     vec![$( stringify!($insert_field) ),*].join(", "),
                     placeholders_str,
                     vec![$( stringify!($select_field) ),*].join(", ")
                 );

                 let mut query_builder = sqlx::query_as::<_, $row>(&query);
                 $(
                     query_builder = query_builder.bind(&model.$insert_field);
                 )*
                 let result = query_builder
                     .fetch_one(&self.pool)
                     .await
                     .map_err(crate::domain::error::AppError::from)?;

                 Ok(result.into())
             }

             pub async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<$model>, crate::domain::error::AppError> {
                 let query = format!("SELECT {} FROM {} WHERE id = $1", vec![$( stringify!($select_field) ),*].join(", "), $table);
                 let user = sqlx::query_as::<_, $row>(&query)
                     .bind(id)
                     .fetch_optional(&self.pool)
                     .await
                     .map_err(crate::domain::error::AppError::from)?;
                 Ok(user.map(|u| u.into()))
             }

             pub async fn find_all(&self) -> Result<Vec<$model>, crate::domain::error::AppError> {
                 let query = format!("SELECT {} FROM {} ORDER BY created DESC", vec![$( stringify!($select_field) ),*].join(", "), $table);
                 let rows = sqlx::query_as::<_, $row>(&query)
                     .fetch_all(&self.pool)
                     .await
                     .map_err(crate::domain::error::AppError::from)?;
                 Ok(rows.into_iter().map(|u| u.into()).collect())
             }

             pub async fn update<T: crate::domain::Transformer<$model>>(&self, to_model: T) -> Result<$model, crate::domain::error::AppError> {
                 let model = to_model.transform()?;

                 let set_parts: Vec<String> = vec![$( stringify!($update_field) ),*].iter().enumerate().map(|(i, f)| {
                     format!("{} = ${}", f, i + 2)
                 }).collect();
                 let set_str = set_parts.join(", ");

                 let query = format!(
                     "UPDATE {} SET {} WHERE id = $1 RETURNING {}",
                     $table, set_str, vec![$( stringify!($select_field) ),*].join(", ")
                 );

                 let mut query_builder = sqlx::query_as::<_, $row>(&query);
                 query_builder = query_builder.bind(model.id);
                 $(
                     query_builder = query_builder.bind(&model.$update_field);
                 )*
                 let result = query_builder
                     .fetch_one(&self.pool)
                     .await
                     .map_err(crate::domain::error::AppError::from)?;

                 Ok(result.into())
             }

             pub async fn delete(&self, id: uuid::Uuid) -> Result<bool, crate::domain::error::AppError> {
                 let query = format!("DELETE FROM {} WHERE id = $1", $table);
                 let result = sqlx::query(&query)
                     .bind(id)
                     .execute(&self.pool)
                     .await
                     .map_err(crate::domain::error::AppError::from)?;
                 Ok(result.rows_affected() > 0)
             }
         }
    };
}
