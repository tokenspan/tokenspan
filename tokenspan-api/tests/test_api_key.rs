use anyhow::Result;
use axum_test::http::{HeaderName, HeaderValue};
use axum_test::TestServer;
use googletest::matchers::{anything, eq, some};
use googletest::prelude::*;
use graphql_client::{GraphQLQuery, Response};

use tokenspan_api::domains::dto::{ApiKeyCreateInput, ProviderCreateInput};
use tokenspan_api::domains::models::UserRole;
use tokenspan_api::state::AppState;

use crate::graphql::{
    create_api_key_mutation, delete_api_key_mutation, get_api_key_query, get_api_keys_query,
    update_api_key_mutation, CreateApiKeyMutation, DeleteApiKeyMutation, GetApiKeyQuery,
    GetApiKeysQuery, UpdateApiKeyMutation,
};

mod common;
mod graphql;

macro_rules! create_api_key {
    ($state: ident, name = $name: literal, provider_id = $provider_id: expr, user_id = $user_id: expr) => {
        $state
            .api_key_service
            .create(
                ApiKeyCreateInput {
                    name: $name.to_string(),
                    key: $name.to_string(),
                    provider_id: $provider_id,
                },
                $user_id,
            )
            .await?;
    };
}

macro_rules! make_request {
    ($server: ident, $token: expr, $variables: expr) => {{
        let req_body = GetApiKeysQuery::build_query($variables);
        let resp = $server
            .post("graphql")
            .add_header(
                HeaderName::from_static("authorization"),
                HeaderValue::from_str(format!("Bearer {}", $token).as_str())?,
            )
            .json(&req_body)
            .await;

        resp.json::<Response<get_api_keys_query::ResponseData>>()
    }};
}

#[tokio::test]
async fn test_paginate_forward_api_keys() -> Result<()> {
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

    // Create provider
    let provider_fixture = state
        .provider_service
        .create(ProviderCreateInput {
            name: "OpenAI".to_string(),
            slug: "openai".to_string(),
            base_url: "https://api.openai.com".to_string(),
        })
        .await?;

    // Create api_key
    create_api_key!(
        state,
        name = "test",
        provider_id = provider_fixture.id,
        user_id = auth_fixture.user.id
    );
    create_api_key!(
        state,
        name = "test1",
        provider_id = provider_fixture.id,
        user_id = auth_fixture.user.id
    );

    // Get api keys
    let variables = get_api_keys_query::Variables {
        args: get_api_keys_query::ApiKeyArgs {
            first: None,
            after: None,
            last: Some(1),
            before: None,
            where_: None,
        },
    };
    let resp = make_request!(server, auth_fixture.token, variables);

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_api_keys_query::ResponseData {
            api_keys: pat!(get_api_keys_query::GetApiKeysQueryApiKeys {
                nodes: contains_each![pat!(get_api_keys_query::GetApiKeysQueryApiKeysNodes {
                    id: anything(),
                    name: eq("test1".to_string()),
                    provider_id: eq(provider_fixture.id),
                    created_at: anything(),
                }),],
                total_nodes: eq(2),
                page_info: pat!(get_api_keys_query::GetApiKeysQueryApiKeysPageInfo {
                    has_next_page: eq(false),
                    has_previous_page: eq(true),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    // Get api keys
    let variables = get_api_keys_query::Variables {
        args: get_api_keys_query::ApiKeyArgs {
            first: None,
            after: None,
            last: Some(1),
            before: None,
            where_: None,
        },
    };
    let resp = make_request!(server, auth_fixture.token, variables);

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_api_keys_query::ResponseData {
            api_keys: pat!(get_api_keys_query::GetApiKeysQueryApiKeys {
                nodes: contains_each![pat!(get_api_keys_query::GetApiKeysQueryApiKeysNodes {
                    id: anything(),
                    name: eq("test1".to_string()),
                    provider_id: eq(provider_fixture.id),
                    created_at: anything(),
                }),],
                total_nodes: eq(2),
                page_info: pat!(get_api_keys_query::GetApiKeysQueryApiKeysPageInfo {
                    has_next_page: eq(false),
                    has_previous_page: eq(true),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_paginate_backward_api_keys() -> Result<()> {
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

    // Create provider
    let provider_fixture = state
        .provider_service
        .create(ProviderCreateInput {
            name: "OpenAI".to_string(),
            slug: "openai".to_string(),
            base_url: "https://api.openai.com".to_string(),
        })
        .await?;

    // Create api_key
    state
        .api_key_service
        .create(
            ApiKeyCreateInput {
                name: "test".to_string(),
                key: "test".to_string(),
                provider_id: provider_fixture.id,
            },
            auth_fixture.user.id,
        )
        .await?;

    state
        .api_key_service
        .create(
            ApiKeyCreateInput {
                name: "test1".to_string(),
                key: "test1".to_string(),
                provider_id: provider_fixture.id,
            },
            auth_fixture.user.id,
        )
        .await?;

    // Get api keys
    let variables = get_api_keys_query::Variables {
        args: get_api_keys_query::ApiKeyArgs {
            first: Some(2),
            after: None,
            last: None,
            before: None,
            where_: None,
        },
    };
    let req_body = GetApiKeysQuery::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<get_api_keys_query::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_api_keys_query::ResponseData {
            api_keys: pat!(get_api_keys_query::GetApiKeysQueryApiKeys {
                nodes: contains_each![
                    pat!(get_api_keys_query::GetApiKeysQueryApiKeysNodes {
                        id: anything(),
                        name: eq("test".to_string()),
                        provider_id: eq(provider_fixture.id),
                        created_at: anything(),
                    }),
                    pat!(get_api_keys_query::GetApiKeysQueryApiKeysNodes {
                        id: anything(),
                        name: eq("test1".to_string()),
                        provider_id: eq(provider_fixture.id),
                        created_at: anything(),
                    }),
                ],
                total_nodes: eq(2),
                page_info: pat!(get_api_keys_query::GetApiKeysQueryApiKeysPageInfo {
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
async fn test_get_api_keys() -> Result<()> {
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

    // Create provider
    let provider_fixture = state
        .provider_service
        .create(ProviderCreateInput {
            name: "OpenAI".to_string(),
            slug: "openai".to_string(),
            base_url: "https://api.openai.com".to_string(),
        })
        .await?;

    // Create api_key
    state
        .api_key_service
        .create(
            ApiKeyCreateInput {
                name: "test".to_string(),
                key: "test".to_string(),
                provider_id: provider_fixture.id,
            },
            auth_fixture.user.id,
        )
        .await?;

    state
        .api_key_service
        .create(
            ApiKeyCreateInput {
                name: "test1".to_string(),
                key: "test1".to_string(),
                provider_id: provider_fixture.id,
            },
            auth_fixture.user.id,
        )
        .await?;

    // Get api keys
    let variables = get_api_keys_query::Variables {
        args: get_api_keys_query::ApiKeyArgs {
            first: None,
            after: None,
            last: None,
            before: None,
            where_: None,
        },
    };
    let req_body = GetApiKeysQuery::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<get_api_keys_query::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_api_keys_query::ResponseData {
            api_keys: pat!(get_api_keys_query::GetApiKeysQueryApiKeys {
                nodes: contains_each![
                    pat!(get_api_keys_query::GetApiKeysQueryApiKeysNodes {
                        id: anything(),
                        name: eq("test".to_string()),
                        provider_id: eq(provider_fixture.id),
                        created_at: anything(),
                    }),
                    pat!(get_api_keys_query::GetApiKeysQueryApiKeysNodes {
                        id: anything(),
                        name: eq("test1".to_string()),
                        provider_id: eq(provider_fixture.id),
                        created_at: anything(),
                    }),
                ],
                total_nodes: eq(2),
                page_info: pat!(get_api_keys_query::GetApiKeysQueryApiKeysPageInfo {
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
async fn test_get_api_key_by_id() -> Result<()> {
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

    // Create provider
    let provider_fixture = state
        .provider_service
        .create(ProviderCreateInput {
            name: "OpenAI".to_string(),
            slug: "openai".to_string(),
            base_url: "https://api.openai.com".to_string(),
        })
        .await?;

    // Create api_key
    let api_key_fixture = state
        .api_key_service
        .create(
            ApiKeyCreateInput {
                name: "test".to_string(),
                key: "test".to_string(),
                provider_id: provider_fixture.id,
            },
            auth_fixture.user.id,
        )
        .await?;

    // Get api_key
    let variables = get_api_key_query::Variables {
        id: api_key_fixture.id,
    };
    let req_body = GetApiKeyQuery::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<get_api_key_query::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_api_key_query::ResponseData {
            api_key: some(pat!(get_api_key_query::GetApiKeyQueryApiKey {
                id: anything(),
                name: eq("test".to_string()),
                owner_id: eq(auth_fixture.user.id),
                provider_id: eq(provider_fixture.id),
                provider: some(pat!(get_api_key_query::GetApiKeyQueryApiKeyProvider {
                    id: eq(provider_fixture.id),
                    name: eq("OpenAI".to_string()),
                    slug: eq("openai".to_string()),
                })),
                owner: some(pat!(get_api_key_query::GetApiKeyQueryApiKeyOwner {
                    id: eq(auth_fixture.user.id),
                    username: eq("linh".to_string()),
                })),
                created_at: anything(),
                updated_at: anything(),
            }))
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_create_api_key() -> Result<()> {
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

    // Create provider
    let provider_fixture = state
        .provider_service
        .create(ProviderCreateInput {
            name: "OpenAI".to_string(),
            slug: "openai".to_string(),
            base_url: "https://api.openai.com".to_string(),
        })
        .await?;

    // Create api_key
    let variables = create_api_key_mutation::Variables {
        input: create_api_key_mutation::ApiKeyCreateInput {
            name: "test".to_string(),
            key: "test".to_string(),
            provider_id: provider_fixture.id,
        },
    };
    let req_body = CreateApiKeyMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<create_api_key_mutation::ResponseData>>();

    assert_that!(
        resp.data,
        some(pat!(create_api_key_mutation::ResponseData {
            create_api_key: pat!(create_api_key_mutation::CreateApiKeyMutationCreateApiKey {
                id: anything(),
                name: eq("test".to_string()),
                owner_id: eq(auth_fixture.user.id),
                provider_id: eq(provider_fixture.id),
                provider: some(pat!(
                    create_api_key_mutation::CreateApiKeyMutationCreateApiKeyProvider {
                        id: eq(provider_fixture.id),
                        name: eq("OpenAI".to_string()),
                        slug: eq("openai".to_string()),
                    }
                )),
                owner: some(pat!(
                    create_api_key_mutation::CreateApiKeyMutationCreateApiKeyOwner {
                        id: eq(auth_fixture.user.id),
                        username: eq("linh".to_string()),
                    }
                )),
                created_at: anything(),
                updated_at: anything(),
            })
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_update_api_key() -> Result<()> {
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

    // Create provider
    let provider_fixture = state
        .provider_service
        .create(ProviderCreateInput {
            name: "OpenAI".to_string(),
            slug: "openai".to_string(),
            base_url: "https://api.openai.com".to_string(),
        })
        .await?;

    // Create api_key
    let api_key_fixture = state
        .api_key_service
        .create(
            ApiKeyCreateInput {
                name: "test".to_string(),
                key: "test".to_string(),
                provider_id: provider_fixture.id,
            },
            auth_fixture.user.id,
        )
        .await?;

    // Update api_key
    let variables = update_api_key_mutation::Variables {
        update_api_key_id: api_key_fixture.id,
        input: update_api_key_mutation::ApiKeyUpdateInput {
            name: Some("test1".to_string()),
        },
    };
    let req_body = UpdateApiKeyMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<update_api_key_mutation::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(update_api_key_mutation::ResponseData {
            update_api_key: pat!(update_api_key_mutation::UpdateApiKeyMutationUpdateApiKey {
                id: anything(),
                name: eq("test1".to_string()),
            })
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_delete_api_key() -> Result<()> {
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

    // Create provider
    let provider_fixture = state
        .provider_service
        .create(ProviderCreateInput {
            name: "OpenAI".to_string(),
            slug: "openai".to_string(),
            base_url: "https://api.openai.com".to_string(),
        })
        .await?;

    // Create api_key
    let api_key_fixture = state
        .api_key_service
        .create(
            ApiKeyCreateInput {
                name: "test".to_string(),
                key: "test".to_string(),
                provider_id: provider_fixture.id,
            },
            auth_fixture.user.id,
        )
        .await?;

    // Update api_key
    let variables = delete_api_key_mutation::Variables {
        delete_api_key_id: api_key_fixture.id,
    };
    let req_body = DeleteApiKeyMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<delete_api_key_mutation::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(delete_api_key_mutation::ResponseData {
            delete_api_key: pat!(delete_api_key_mutation::DeleteApiKeyMutationDeleteApiKey {
                id: eq(api_key_fixture.id),
                name: eq("test".to_string()),
            })
        }))
    );

    Ok(())
}
