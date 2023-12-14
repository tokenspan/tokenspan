use std::collections::BTreeMap;
use std::fmt::Display;
use std::sync::Arc;

use comfy_table::Table;
use console::{style, Emoji};
use indicatif::{MultiProgress, ProgressBar};
use tokio::sync::Mutex;
use tokio_stream::StreamExt;

use tokenspan_api::api::dto::ModelCreateInput;
use tokenspan_api::api::models::{Model, Provider, User};
use tokenspan_api::state::AppState;

use crate::configs::{ModelSeed, ProviderSeed, UserSeed};

mod configs;

static CLIP: Emoji<'_, '_> = Emoji("🔗  ", "");

enum CacheValue {
    Provider(Action, Provider),
    Model(Action, Model),
    User(Action, User),
}

enum Action {
    Created,
    Existed,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Created => write!(f, "Created"),
            Action::Existed => write!(f, "Existed"),
        }
    }
}

#[tokio::main]
async fn main() {
    let app_config = tokenspan_api::configs::AppConfig::new().unwrap();
    let app_config = Arc::new(app_config);

    let seed_config = configs::SeedConfig::new().unwrap();

    let cache = Arc::new(Mutex::new(BTreeMap::new()));

    let app_state = AppState::new(app_config).await;

    let progress = Arc::new(MultiProgress::new());

    migrate_providers(
        app_state.clone(),
        cache.clone(),
        progress.clone(),
        seed_config.providers,
    )
    .await;
    migrate_models(
        app_state.clone(),
        cache.clone(),
        progress.clone(),
        seed_config.models,
    )
    .await;
    migrate_users(
        app_state.clone(),
        cache.clone(),
        progress.clone(),
        seed_config.users,
    )
    .await;
    progress.clear().unwrap();
    print_table(cache).await;
}

async fn print_table(cache: Arc<Mutex<BTreeMap<String, CacheValue>>>) {
    let mut table = Table::new();
    table.set_header(vec!["Collection", "Key", "Id", "Action"]);

    for (key, value) in cache.lock().await.iter() {
        match value {
            CacheValue::Provider(action, provider) => table.add_row(vec![
                "Provider",
                key,
                &provider.id.to_string(),
                &action.to_string(),
            ]),
            CacheValue::Model(action, model) => table.add_row(vec![
                "Model",
                key,
                &model.id.to_string(),
                &action.to_string(),
            ]),
            CacheValue::User(action, user) => {
                table.add_row(vec!["User", key, &user.id.to_string(), &action.to_string()])
            }
        };
    }

    println!("{table}");
}

async fn migrate_providers(
    app_state: AppState,
    cache: Arc<Mutex<BTreeMap<String, CacheValue>>>,
    progress: Arc<MultiProgress>,
    providers: Vec<ProviderSeed>,
) {
    println!(
        "{} {}Migrate providers...",
        style("[1/3]").bold().dim(),
        CLIP
    );
    let pb = Arc::new(progress.add(ProgressBar::new(providers.len() as u64)));
    let mut stream = tokio_stream::iter(providers);
    let mut cache = cache.lock().await;

    while let Some(provider) = stream.next().await {
        let provider_name = provider.name.clone();
        let result = app_state
            .provider_service
            .get_provider_by_name(provider_name.clone())
            .await
            .unwrap();

        let value = if let Some(provider) = result {
            (Action::Existed, provider)
        } else {
            let provider = app_state
                .provider_service
                .create_provider(tokenspan_api::api::dto::ProviderCreateInput {
                    name: provider_name.clone(),
                })
                .await
                .unwrap();
            (Action::Created, provider)
        };

        cache.insert(provider_name, CacheValue::Provider(value.0, value.1));
        pb.inc(1);
    }
}

async fn migrate_models(
    app_state: AppState,
    cache: Arc<Mutex<BTreeMap<String, CacheValue>>>,
    progress: Arc<MultiProgress>,
    models: Vec<ModelSeed>,
) {
    println!("{} {}Migrate models...", style("[2/3]").bold().dim(), CLIP);
    let pb = Arc::new(progress.add(ProgressBar::new(models.len() as u64)));
    let mut stream = tokio_stream::iter(models);
    let mut cache = cache.lock().await;

    while let Some(model) = stream.next().await {
        let provider_name = model.provider.name;
        let provider = if let CacheValue::Provider(_, provider) = cache
            .get(&provider_name)
            .expect("provider should be in cache")
        {
            provider
        } else {
            panic!("provider should be in cache")
        };

        let model_name = model.name.clone();
        let result = app_state
            .model_service
            .find_by_name(model_name.clone())
            .await
            .unwrap();

        let value = if let Some(model) = result {
            (Action::Existed, model)
        } else {
            let model = app_state
                .model_service
                .create(ModelCreateInput {
                    provider_id: provider.id.clone(),
                    name: model.name,
                    description: model.description,
                    context: model.context,
                    input_pricing: model.input_pricing.into(),
                    output_pricing: model.output_pricing.into(),
                    training_at: model.training_at,
                })
                .await
                .unwrap();
            (Action::Created, model)
        };

        cache.insert(model_name, CacheValue::Model(value.0, value.1));
        pb.inc(1);
    }
}

async fn migrate_users(
    app_state: AppState,
    cache: Arc<Mutex<BTreeMap<String, CacheValue>>>,
    progress: Arc<MultiProgress>,
    models: Vec<UserSeed>,
) {
    println!("{} {}Migrate users...", style("[3/3]").bold().dim(), CLIP);
    let pb = Arc::new(progress.add(ProgressBar::new(models.len() as u64)));
    let mut stream = tokio_stream::iter(models);
    let mut cache = cache.lock().await;

    while let Some(user) = stream.next().await {
        let email = user.email.clone();
        let result = app_state
            .user_service
            .get_user_by_email(email.clone())
            .await
            .unwrap();

        let value = if let Some(user) = result {
            (Action::Existed, user)
        } else {
            let user = app_state
                .user_service
                .create_user_with_role(email.clone(), user.username, user.password, user.role)
                .await
                .unwrap();
            (Action::Created, user)
        };

        cache.insert(email, CacheValue::User(value.0, value.1));
        pb.inc(1);
    }
}
