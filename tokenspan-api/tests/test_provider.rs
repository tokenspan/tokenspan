use anyhow::Result;
use axum_test::http::{HeaderName, HeaderValue};
use axum_test::TestServer;
use googletest::matchers::{anything, eq, some};
use googletest::prelude::*;
use googletest::{assert_that, pat};
use graphql_client::{GraphQLQuery, Response};

use tokenspan_api::api::dto::ProviderCreateInput;
use tokenspan_api::api::models::UserRole;
use tokenspan_api::state::AppState;

use crate::graphql::{
    create_provider_mutation, delete_provider_mutation, get_provider_query, get_providers_query,
    update_provider_mutation, CreateProviderMutation, DeleteProviderMutation, GetProviderQuery,
    GetProvidersQuery, UpdateProviderMutation,
};

mod common;
mod graphql;

#[tokio::test]
async fn test_get_providers() -> Result<()> {
    // Setup
    let state: AppState;
    let server: TestServer;
    setup!(state, server);

    // Create new user
    let auth_payload = state
        .auth_service
        .sign_up_with_role(
            "linh@gmail.com".to_string(),
            "linh".to_string(),
            "123".to_string(),
            UserRole::Admin,
        )
        .await?;

    // Create provider
    state
        .provider_service
        .create(ProviderCreateInput {
            name: "OpenAI".to_string(),
            slug: "openai".to_string(),
        })
        .await?;

    state
        .provider_service
        .create(ProviderCreateInput {
            name: "Cohere".to_string(),
            slug: "cohere".to_string(),
        })
        .await?;

    // Create provider
    let variables = get_providers_query::Variables {};
    let req_body = GetProvidersQuery::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_payload.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<get_providers_query::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_providers_query::ResponseData {
            providers: pat!(get_providers_query::GetProvidersQueryProviders {
                nodes: contains_each![
                    pat!(get_providers_query::GetProvidersQueryProvidersNodes {
                        id: anything(),
                        name: eq("OpenAI".to_string()),
                        slug: eq("openai".to_string()),
                        created_at: anything(),
                        updated_at: anything(),
                    }),
                    pat!(get_providers_query::GetProvidersQueryProvidersNodes {
                        id: anything(),
                        name: eq("Cohere".to_string()),
                        slug: eq("cohere".to_string()),
                        created_at: anything(),
                        updated_at: anything(),
                    }),
                ],
                page_info: pat!(get_providers_query::GetProvidersQueryProvidersPageInfo {
                    has_next_page: eq(false),
                    has_previous_page: eq(false),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_get_provider_by_id() -> Result<()> {
    // Setup
    let state: AppState;
    let server: TestServer;
    setup!(state, server);

    // Create new user
    let auth_payload = state
        .auth_service
        .sign_up_with_role(
            "linh@gmail.com".to_string(),
            "linh".to_string(),
            "123".to_string(),
            UserRole::Admin,
        )
        .await?;

    // Create provider
    let provider_fixture = state
        .provider_service
        .create(ProviderCreateInput {
            name: "OpenAI".to_string(),
            slug: "openai".to_string(),
        })
        .await?;

    // Cet provider
    let variables = get_provider_query::Variables {
        provider_id: provider_fixture.id,
    };
    let req_body = GetProviderQuery::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_payload.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<get_provider_query::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_provider_query::ResponseData {
            provider: some(pat!(get_provider_query::GetProviderQueryProvider {
                id: eq(provider_fixture.id),
                name: eq("OpenAI".to_string()),
                slug: eq("openai".to_string()),
                created_at: anything(),
                updated_at: anything(),
            }))
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_create_provider() -> Result<()> {
    // Setup
    let state: AppState;
    let server: TestServer;
    setup!(state, server);

    // create new user
    let auth_payload = state
        .auth_service
        .sign_up_with_role(
            "linh@gmail.com".to_string(),
            "linh".to_string(),
            "123".to_string(),
            UserRole::Admin,
        )
        .await?;

    // Create provider
    let variables = create_provider_mutation::Variables {
        input: create_provider_mutation::ProviderCreateInput {
            name: "OpenAI".to_string(),
            slug: "openai".to_string(),
        },
    };
    let req_body = CreateProviderMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_payload.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<create_provider_mutation::ResponseData>>();

    assert_that!(
        resp.data,
        some(pat!(create_provider_mutation::ResponseData {
            create_provider: pat!(
                create_provider_mutation::CreateProviderMutationCreateProvider {
                    id: anything(),
                    name: eq("OpenAI".to_string()),
                    slug: eq("openai".to_string()),
                    created_at: anything(),
                    updated_at: anything(),
                }
            )
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_update_provider() -> Result<()> {
    // Setup
    let state: AppState;
    let server: TestServer;
    setup!(state, server);

    // Create new user
    let auth_payload = state
        .auth_service
        .sign_up_with_role(
            "linh@gmail.com".to_string(),
            "linh".to_string(),
            "123".to_string(),
            UserRole::Admin,
        )
        .await?;

    // Create provider
    let provider_fixture = state
        .provider_service
        .create(ProviderCreateInput {
            name: "OpenAI".to_string(),
            slug: "openai".to_string(),
        })
        .await?;

    // Update provider
    let variables = update_provider_mutation::Variables {
        id: provider_fixture.id,
        input: update_provider_mutation::ProviderUpdateInput {
            name: Some("Cohere".to_string()),
            slug: Some("cohere".to_string()),
        },
    };
    let req_body = UpdateProviderMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_payload.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<update_provider_mutation::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(update_provider_mutation::ResponseData {
            update_provider: some(pat!(
                update_provider_mutation::UpdateProviderMutationUpdateProvider {
                    id: eq(provider_fixture.id),
                    name: eq("Cohere".to_string()),
                    slug: eq("cohere".to_string()),
                    created_at: anything(),
                    updated_at: anything(),
                }
            ))
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_delete_provider() -> Result<()> {
    // Setup
    let state: AppState;
    let server: TestServer;
    setup!(state, server);

    // Create new user
    let auth_payload = state
        .auth_service
        .sign_up_with_role(
            "linh@gmail.com".to_string(),
            "linh".to_string(),
            "123".to_string(),
            UserRole::Admin,
        )
        .await?;

    // Create provider
    let provider_fixture = state
        .provider_service
        .create(ProviderCreateInput {
            name: "OpenAI".to_string(),
            slug: "openai".to_string(),
        })
        .await?;

    // Update provider
    let variables = delete_provider_mutation::Variables {
        id: provider_fixture.id,
    };
    let req_body = DeleteProviderMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_payload.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<delete_provider_mutation::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(delete_provider_mutation::ResponseData {
            delete_provider: some(pat!(
                delete_provider_mutation::DeleteProviderMutationDeleteProvider {
                    id: eq(provider_fixture.id),
                    name: eq("OpenAI".to_string()),
                    slug: eq("openai".to_string()),
                    created_at: anything(),
                    updated_at: anything(),
                }
            ))
        }))
    );

    Ok(())
}
