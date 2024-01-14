use axum_test::TestServer;
use chrono::Utc;
use httpmock::prelude::*;
use httpmock::MockServer;
use tokenspan_api::api::dto::{
    ApiKeyCreateInput, ModelCreateInput, PricingInput, ProviderCreateInput, ThreadCreateInput,
};
use tokenspan_api::api::models::UserRole;
use tokenspan_api::state::AppState;

mod common;

#[tokio::test]
async fn test_task_execute() -> anyhow::Result<()> {
    let mock_server = MockServer::start();
    mock_server.mock(|when, then| {
        when.method(POST).path("/v1/chat/completions");

        then.status(200)
            .header("content-type", "application/json")
            .body(
                r#"{
                "id": "chatcmpl-8gnxs0sqo53wEWkxkIOchb78jRIf4",
                "object": "chat.completion",
                "created": 1705212532,
                "model": "gpt-3.5-turbo-0613",
                "choices": [
                    {
                        "index": 0,
                        "message": {
                            "role": "assistant",
                            "content": "She did not go to the market."
                        },
                        "logprobs": null,
                        "finish_reason": "stop"
                    }
                ],
                "usage": {
                    "prompt_tokens": 36,
                    "completion_tokens": 8,
                    "total_tokens": 44
                },
                "system_fingerprint": null
            }"#,
            );
    });

    // Setup
    let state: AppState;
    let server: TestServer;
    setup!(state, server);

    // Create new user
    let auth_fixture = state
        .auth_service
        .sign_up_with_role(
            "linh@gmail.com".to_string(),
            "linh".to_string(),
            "123".to_string(),
            UserRole::Admin,
        )
        .await?;

    let provider_fixture = state
        .provider_service
        .create(ProviderCreateInput {
            name: "OpenAI".to_string(),
            slug: "openai".to_string(),
            base_url: mock_server.base_url(),
        })
        .await?;

    state
        .model_service
        .create(ModelCreateInput {
            name: "gpt-3.5-turbo".to_string(),
            slug: "gpt-3.5-turbo".to_string(),
            description: "GPT-3.5 Turbo is a language model that can generate text from a prompt."
                .to_string(),
            provider_id: provider_fixture.id,
            context: 256,
            training_at: Utc::now().naive_utc(),
            input_pricing: PricingInput {
                currency: "USD".to_string(),
                price: 0.06,
                tokens: 1,
            },
            output_pricing: PricingInput {
                currency: "USD".to_string(),
                price: 0.06,
                tokens: 1,
            },
        })
        .await?;

    let thread_fixture = state
        .thread_service
        .new(
            ThreadCreateInput {
                name: "thread".to_string(),
                slug: "thread".to_string(),
            },
            auth_fixture.user.id,
        )
        .await?;

    let thread_version_fixture = state
        .thread_version_service
        .find_latest(&thread_fixture.id)
        .await?
        .ok_or(anyhow::anyhow!("Thread version not found"))?;

    let parameter_fixture = state
        .parameter_service
        .find_by_thread_version_id(&thread_version_fixture.id)
        .await?
        .first()
        .cloned()
        .ok_or(anyhow::anyhow!("Parameter not found"))?;

    let api_key_fixture = state
        .api_key_service
        .create(
            ApiKeyCreateInput {
                name: "OpenAI".to_string(),
                key: "sk-123".to_string(),
                provider_id: provider_fixture.id,
            },
            auth_fixture.user.id,
        )
        .await?;

    let resp = server
        .post("/api/v1/threads/execute")
        .json(&serde_json::json!({
            "thread_version_id": thread_version_fixture.id,
            "parameter_id": parameter_fixture.id,
            "api_key_id": api_key_fixture.id,
            "variables": {
                "sentence": "She did not go to the market."
            }
        }))
        .await;
    println!("{:?}", resp);

    Ok(())
}
