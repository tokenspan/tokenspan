use axum_test::TestServer;

use tokenspan_api::api::models::UserRole;
use tokenspan_api::state::AppState;

mod common;
mod graphql;

macro_rules! create_model {
    ($state: ident, name = $name: literal) => {
        let provider_fixture = $state
            .provider_service
            .create(tokenspan_api::api::dto::ProviderCreateInput {
                name: "OpenAI".to_string(),
                slug: "openai".to_string(),
            })
            .await?;

        $state
            .model_service
            .create(tokenspan_api::api::dto::ModelCreateInput {
                name: $name.to_string(),
                description: $name.to_string(),
                slug: $name.to_string(),
                context: 256,
                input_pricing: tokenspan_api::api::dto::PricingInput {
                    currency: "USD".to_string(),
                    price: 0.06,
                    tokens: 1,
                },
                output_pricing: tokenspan_api::api::dto::PricingInput {
                    currency: "USD".to_string(),
                    price: 0.06,
                    tokens: 1,
                },
                training_at: Default::default(),
                provider_id: provider_fixture.id,
            })
            .await?;
    };
}

macro_rules! create_thread {
    ($state: ident, name = $name: literal, slug = $slug: literal, user_id = $user_id: expr) => {{
        $state
            .thread_service
            .create(
                tokenspan_api::api::dto::ThreadCreateInput {
                    name: $name.to_string(),
                    slug: $slug.to_string(),
                },
                $user_id,
            )
            .await?
    }};
}

macro_rules! make_request {
    ($server: ident, $token: expr, $variables: ident) => {{
        let req_body = GetThreadsQuery::build_query($variables);
        let resp = $server
            .post("graphql")
            .add_header(
                HeaderName::from_static("authorization"),
                HeaderValue::from_str(format!("Bearer {}", $token).as_str())?,
            )
            .json(&req_body)
            .await;
        resp.json::<Response<get_threads_query::ResponseData>>()
    }};
}

#[tokio::test]
async fn test_create_thread_version() -> anyhow::Result<()> {
    // Setup
    let state: AppState;
    let server: TestServer;
    setup!(state, server);

    // create new user
    let auth_fixture = state
        .auth_service
        .sign_up_with_role(
            "linh@gmail.com".to_string(),
            "linh".to_string(),
            "123".to_string(),
            UserRole::Admin,
        )
        .await?;

    // Create thread
    create_model!(state, name = "GPT-3");
    let thread_fixture = create_thread!(
        state,
        name = "GPT-3",
        slug = "gpt-3",
        user_id = auth_fixture.user.id
    );
    println!("thread_fixture: {:#?}", thread_fixture);

    // Create thread version
    // let req_body = CreateThreadVersionMutation::build_query(variables);
    // let resp = server
    //     .post("graphql")
    //     .add_header(
    //         HeaderName::from_static("authorization"),
    //         HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
    //     )
    //     .json(&req_body)
    //     .await;
    // let resp = resp.json::<Response<create_thread_mutation::ResponseData>>();

    Ok(())
}
