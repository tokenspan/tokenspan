pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20231216_063415_create_provider_table;
mod m20231216_064026_create_model_table;
mod m20231216_070209_create_api_key_table;
mod m20231216_070852_create_task_table;
mod m20231216_072112_create_task_version_table;
mod m20231216_072606_create_parameter_table;
mod m20231216_073624_create_execution_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20231216_063415_create_provider_table::Migration),
            Box::new(m20231216_064026_create_model_table::Migration),
            Box::new(m20231216_070209_create_api_key_table::Migration),
            Box::new(m20231216_070852_create_task_table::Migration),
            Box::new(m20231216_072112_create_task_version_table::Migration),
            Box::new(m20231216_072606_create_parameter_table::Migration),
            Box::new(m20231216_073624_create_execution_table::Migration),
        ]
    }
}
