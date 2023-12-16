use std::fs;

use async_trait::async_trait;
use serde::de::DeserializeOwned;

use tokenspan_api::configs::AppConfig;
use tokenspan_api::state::AppState;

mod model_seed;
mod provider_seed;
mod task_seed;
mod user_seed;

pub mod prelude {
    pub use crate::seed::model_seed::*;
    pub use crate::seed::provider_seed::*;
    pub use crate::seed::task_seed::*;
    pub use crate::seed::user_seed::*;
    pub use crate::seed::Seed;
}

#[async_trait]
pub trait Seed {
    async fn new(config: AppConfig, state: AppState) -> anyhow::Result<Self>
    where
        Self: Sized;

    async fn load<T>() -> anyhow::Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let paths = fs::read_dir(Self::path())?;
        let mut data = vec![];
        for path in paths {
            let path = path?.path();
            let content = fs::read_to_string(path.clone())?;
            let provider: T = serde_yaml::from_str(&content)?;
            data.push(provider);
        }

        Ok(data)
    }

    async fn save(&self) -> anyhow::Result<()>;

    fn path() -> &'static str;
}
