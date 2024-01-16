use std::fs;

use async_trait::async_trait;
use serde::de::DeserializeOwned;

mod function_seed;
mod model_seed;
mod provider_seed;
mod thread_seed;
mod user_seed;

pub mod prelude {
    pub use crate::seed::function_seed::*;
    pub use crate::seed::model_seed::*;
    pub use crate::seed::provider_seed::*;
    pub use crate::seed::thread_seed::*;
    pub use crate::seed::user_seed::*;
    pub use crate::seed::Seed;
}

#[async_trait]
pub trait Seed {
    async fn load<T>() -> anyhow::Result<Vec<T>>
    where
        T: DeserializeOwned,
    {
        let paths = fs::read_dir(Self::path())?;
        let mut items = vec![];
        for path in paths {
            let path = path?.path();
            let content = fs::read_to_string(path.clone())?;
            let data: T = serde_yaml::from_str(&content)?;
            items.push(data);
        }

        Ok(items)
    }

    async fn save(&self) -> anyhow::Result<()>;

    fn path() -> &'static str;
}
