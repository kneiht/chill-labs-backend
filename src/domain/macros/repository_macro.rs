#[macro_export]
macro_rules! impl_crud_repository {
    ($repo_name:ident, $model_type:ty, $row_type:ty, $table_name:expr, [$($field:ident),* $(,)?]) => {


        #[derive(Clone)]
        pub struct $repo_name {
            pool: PgPool,
        }

        impl $repo_name {
            pub fn new(pool: PgPool) -> Self {
                Self { pool }
            }

            pub async fn create(&self, entity: &$model_type) -> Result<$model_type, AppError> {
                let fields = vec![$(stringify!($field)),*].join(", ");
                let placeholders = (1..=vec![$(stringify!($field)),*].len())
                    .map(|i| format!("${}", i))
                    .collect::<Vec<_>>()
                    .join(", ");

                let sql = format!(
                    "INSERT INTO {} ({}) VALUES ({}) RETURNING *",
                    $table_name, fields, placeholders
                );

                let mut query = sqlx::query_as::<_, $row_type>(&sql);
                $(
                    query = query.bind(&entity.$field);
                )*

                let row = query
                    .fetch_one(&self.pool)
                    .await
                    .map_err(AppError::from)?;

                Ok(row.into())
            }

            pub async fn find_by_id(&self, id: Uuid) -> Result<Option<$model_type>, AppError> {
                let sql = format!("SELECT * FROM {} WHERE id = $1", $table_name);
                let row = sqlx::query_as::<_, $row_type>(&sql)
                    .bind(id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(AppError::from)?;
                Ok(row.map(|r| r.into()))
            }

            pub async fn find_all(&self) -> Result<Vec<$model_type>, AppError> {
                let sql = format!("SELECT * FROM {} ORDER BY created DESC", $table_name);
                let rows = sqlx::query_as::<_, $row_type>(&sql)
                    .fetch_all(&self.pool)
                    .await
                    .map_err(AppError::from)?;
                Ok(rows.into_iter().map(|r| r.into()).collect())
            }

            pub async fn update(&self, entity: &$model_type) -> Result<$model_type, AppError> {
                // bỏ qua id
                let mut index = 1;
                let assignments = vec![
                    $(
                        {
                            index += 1;
                            format!("{} = ${}", stringify!($field), index)
                        }
                    ),*
                ].join(", ");

                let sql = format!(
                    "UPDATE {} SET {} WHERE id = $1 RETURNING *",
                    $table_name, assignments
                );

                let mut query = sqlx::query_as::<_, $row_type>(&sql);
                query = query.bind(&entity.id);
                $(
                    query = query.bind(&entity.$field);
                )*

                let row = query
                    .fetch_one(&self.pool)
                    .await
                    .map_err(AppError::from)?;

                Ok(row.into())
            }

            pub async fn delete(&self, id: Uuid) -> Result<bool, AppError> {
                let sql = format!("DELETE FROM {} WHERE id = $1", $table_name);
                let result = sqlx::query(&sql)
                    .bind(id)
                    .execute(&self.pool)
                    .await
                    .map_err(AppError::from)?;
                Ok(result.rows_affected() > 0)
            }
        }
    };
}
