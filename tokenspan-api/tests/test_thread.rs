use anyhow::Result;
use axum_test::http::{HeaderName, HeaderValue};
use axum_test::TestServer;
use googletest::matchers::{anything, eq, some};
use googletest::prelude::*;
use graphql_client::{GraphQLQuery, Response};

use tokenspan_api::api::dto::ThreadCreateInput;
use tokenspan_api::api::models::UserRole;
use tokenspan_api::state::AppState;

use crate::graphql::{
    create_thread_mutation, delete_thread_mutation, get_thread_query, get_threads_query,
    update_thread_mutation, CreateThreadMutation, DeleteThreadMutation, GetThreadQuery,
    GetThreadsQuery, UpdateThreadMutation,
};

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
    ($state: ident, name = $name: literal, slug = $slug: literal, user_id = $user_id: expr) => {
        $state
            .thread_service
            .new(
                ThreadCreateInput {
                    name: $name.to_string(),
                    slug: $slug.to_string(),
                },
                $user_id,
            )
            .await?
    };
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
async fn test_paginate_forward_threads() -> Result<()> {
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

    // Create thread
    create_model!(state, name = "test");
    create_thread!(
        state,
        name = "test",
        slug = "test",
        user_id = auth_fixture.user.id
    );
    create_thread!(
        state,
        name = "test1",
        slug = "test1",
        user_id = auth_fixture.user.id
    );
    create_thread!(
        state,
        name = "test2",
        slug = "test2",
        user_id = auth_fixture.user.id
    );

    // Get first thread
    let variables = get_threads_query::Variables {
        args: get_threads_query::ThreadArgs {
            first: Some(1),
            after: None,
            last: None,
            before: None,
            where_: None,
        },
    };
    let resp = make_request!(server, auth_fixture.token, variables);

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_threads_query::ResponseData {
            threads: pat!(get_threads_query::GetThreadsQueryThreads {
                nodes: contains_each![pat!(get_threads_query::GetThreadsQueryThreadsNodes {
                    id: anything(),
                    name: eq("test".to_string()),
                    created_at: anything()
                }),],
                total_nodes: eq(3),
                page_info: pat!(get_threads_query::GetThreadsQueryThreadsPageInfo {
                    has_next_page: eq(true),
                    has_previous_page: eq(false),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    // Get second thread
    let cursor = resp.data.unwrap().threads.page_info.end_cursor;
    let variables = get_threads_query::Variables {
        args: get_threads_query::ThreadArgs {
            first: Some(1),
            after: cursor,
            last: None,
            before: None,
            where_: None,
        },
    };
    let resp = make_request!(server, auth_fixture.token, variables);

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_threads_query::ResponseData {
            threads: pat!(get_threads_query::GetThreadsQueryThreads {
                nodes: contains_each![pat!(get_threads_query::GetThreadsQueryThreadsNodes {
                    id: anything(),
                    name: eq("test1".to_string()),
                    created_at: anything()
                }),],
                total_nodes: eq(3),
                page_info: pat!(get_threads_query::GetThreadsQueryThreadsPageInfo {
                    has_next_page: eq(true),
                    has_previous_page: eq(false),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    // Get third thread
    let cursor = resp.data.unwrap().threads.page_info.end_cursor;
    let variables = get_threads_query::Variables {
        args: get_threads_query::ThreadArgs {
            first: Some(1),
            after: cursor,
            last: None,
            before: None,
            where_: None,
        },
    };
    let resp = make_request!(server, auth_fixture.token, variables);

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_threads_query::ResponseData {
            threads: pat!(get_threads_query::GetThreadsQueryThreads {
                nodes: contains_each![pat!(get_threads_query::GetThreadsQueryThreadsNodes {
                    id: anything(),
                    name: eq("test2".to_string()),
                    created_at: anything()
                }),],
                total_nodes: eq(3),
                page_info: pat!(get_threads_query::GetThreadsQueryThreadsPageInfo {
                    has_next_page: eq(false),
                    has_previous_page: eq(false),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    // Get fourth thread
    let cursor = resp.data.unwrap().threads.page_info.end_cursor;
    let variables = get_threads_query::Variables {
        args: get_threads_query::ThreadArgs {
            first: Some(1),
            after: cursor,
            last: None,
            before: None,
            where_: None,
        },
    };
    let resp = make_request!(server, auth_fixture.token, variables);

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_threads_query::ResponseData {
            threads: pat!(get_threads_query::GetThreadsQueryThreads {
                nodes: empty(),
                total_nodes: eq(3),
                page_info: pat!(get_threads_query::GetThreadsQueryThreadsPageInfo {
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
async fn test_paginate_backward_threads() -> Result<()> {
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

    // Create thread
    create_model!(state, name = "test");
    create_thread!(
        state,
        name = "test",
        slug = "test",
        user_id = auth_fixture.user.id
    );
    create_thread!(
        state,
        name = "test1",
        slug = "test1",
        user_id = auth_fixture.user.id
    );
    create_thread!(
        state,
        name = "test2",
        slug = "test2",
        user_id = auth_fixture.user.id
    );

    // Get first thread
    let variables = get_threads_query::Variables {
        args: get_threads_query::ThreadArgs {
            last: Some(1),
            before: None,
            first: None,
            after: None,
            where_: None,
        },
    };
    let resp = make_request!(server, auth_fixture.token, variables);

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_threads_query::ResponseData {
            threads: pat!(get_threads_query::GetThreadsQueryThreads {
                nodes: contains_each![pat!(get_threads_query::GetThreadsQueryThreadsNodes {
                    name: eq("test2".to_string()),
                }),],
                total_nodes: eq(3),
                page_info: pat!(get_threads_query::GetThreadsQueryThreadsPageInfo {
                    has_next_page: eq(false),
                    has_previous_page: eq(true),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    // Get second thread
    let cursor = resp.data.unwrap().threads.page_info.end_cursor;
    let variables = get_threads_query::Variables {
        args: get_threads_query::ThreadArgs {
            last: Some(1),
            before: cursor,
            first: None,
            after: None,
            where_: None,
        },
    };
    let resp = make_request!(server, auth_fixture.token, variables);

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_threads_query::ResponseData {
            threads: pat!(get_threads_query::GetThreadsQueryThreads {
                nodes: contains_each![pat!(get_threads_query::GetThreadsQueryThreadsNodes {
                    name: eq("test1".to_string()),
                }),],
                total_nodes: eq(3),
                page_info: pat!(get_threads_query::GetThreadsQueryThreadsPageInfo {
                    has_next_page: eq(false),
                    has_previous_page: eq(true),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    // Get third thread
    let cursor = resp.data.unwrap().threads.page_info.end_cursor;
    let variables = get_threads_query::Variables {
        args: get_threads_query::ThreadArgs {
            last: Some(1),
            before: cursor,
            first: None,
            after: None,
            where_: None,
        },
    };
    let resp = make_request!(server, auth_fixture.token, variables);

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_threads_query::ResponseData {
            threads: pat!(get_threads_query::GetThreadsQueryThreads {
                nodes: contains_each![pat!(get_threads_query::GetThreadsQueryThreadsNodes {
                    name: eq("test".to_string()),
                }),],
                total_nodes: eq(3),
                page_info: pat!(get_threads_query::GetThreadsQueryThreadsPageInfo {
                    has_next_page: eq(false),
                    has_previous_page: eq(false),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    // Get fourth thread
    let cursor = resp.data.unwrap().threads.page_info.end_cursor;
    let variables = get_threads_query::Variables {
        args: get_threads_query::ThreadArgs {
            last: Some(1),
            before: cursor,
            first: None,
            after: None,
            where_: None,
        },
    };
    let resp = make_request!(server, auth_fixture.token, variables);

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_threads_query::ResponseData {
            threads: pat!(get_threads_query::GetThreadsQueryThreads {
                nodes: empty(),
                total_nodes: eq(3),
                page_info: pat!(get_threads_query::GetThreadsQueryThreadsPageInfo {
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
async fn test_get_thread_by_id() -> Result<()> {
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

    // Create thread
    create_model!(state, name = "test");
    let thread_fixture = create_thread!(
        state,
        name = "test",
        slug = "test",
        user_id = auth_fixture.user.id
    );

    // Get thread
    let variables = get_thread_query::Variables {
        thread_id: thread_fixture.id,
    };
    let req_body = GetThreadQuery::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<get_thread_query::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_thread_query::ResponseData {
            thread: some(pat!(get_thread_query::GetThreadQueryThread {
                id: eq(thread_fixture.id),
                name: eq("test".to_string()),
                slug: eq("test".to_string()),
                owner_id: eq(auth_fixture.user.id),
                version: some(pat!(get_thread_query::GetThreadQueryThreadVersion {
                    id: anything(),
                    semver: eq("0.0.0".to_string()),
                    version: eq(0),
                    release_note: eq(None),
                    description: eq(None),
                    document: eq(None),
                    status: eq(get_thread_query::ThreadVersionStatus::DRAFT),
                    thread_id: anything(),
                    owner_id: eq(auth_fixture.user.id),
                    created_at: anything(),
                    updated_at: anything(),
                    parameters: contains_each![pat!(
                        get_thread_query::GetThreadQueryThreadVersionParameters {
                            id: anything(),
                            name: eq("untitled".to_string()),
                            temperature: eq(1.0),
                        }
                    ),],
                    messages: empty(),
                })),
                owner: some(pat!(get_thread_query::GetThreadQueryThreadOwner {
                    id: anything(),
                    email: eq("linh@gmail.com".to_string()),
                    username: eq("linh".to_string()),
                    role: eq(get_thread_query::UserRole::ADMIN),
                    created_at: anything(),
                    updated_at: anything(),
                })),
                created_at: anything(),
                updated_at: anything(),
            }))
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_create_thread() -> Result<()> {
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
    create_model!(state, name = "test");

    let variables = create_thread_mutation::Variables {
        input: create_thread_mutation::ThreadCreateInput {
            name: "test".to_string(),
            slug: "test".to_string(),
        },
    };
    let req_body = CreateThreadMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<create_thread_mutation::ResponseData>>();

    assert_that!(
        resp.data,
        some(pat!(create_thread_mutation::ResponseData {
            create_thread: pat!(create_thread_mutation::CreateThreadMutationCreateThread {
                id: anything(),
                name: eq("test".to_string()),
                slug: eq("test".to_string()),
                owner_id: eq(auth_fixture.user.id),
                created_at: anything(),
                updated_at: anything(),
            })
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_update_thread() -> Result<()> {
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

    // Create thread
    create_model!(state, name = "test");
    let thread_fixture = create_thread!(
        state,
        name = "test",
        slug = "test",
        user_id = auth_fixture.user.id
    );

    // Update thread
    let variables = update_thread_mutation::Variables {
        update_thread_id: thread_fixture.id,
        input: update_thread_mutation::ThreadUpdateInput {
            name: Some("test1".to_string()),
            slug: None,
        },
    };
    let req_body = UpdateThreadMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<update_thread_mutation::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(update_thread_mutation::ResponseData {
            update_thread: pat!(update_thread_mutation::UpdateThreadMutationUpdateThread {
                id: eq(thread_fixture.id),
                name: eq("test1".to_string()),
            })
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_delete_thread() -> Result<()> {
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

    create_model!(state, name = "test");
    let thread_fixture = create_thread!(
        state,
        name = "test",
        slug = "test",
        user_id = auth_fixture.user.id
    );

    // Update thread
    let variables = delete_thread_mutation::Variables {
        delete_thread_id: thread_fixture.id,
    };
    let req_body = DeleteThreadMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<delete_thread_mutation::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(delete_thread_mutation::ResponseData {
            delete_thread: pat!(delete_thread_mutation::DeleteThreadMutationDeleteThread {
                id: eq(thread_fixture.id),
                name: eq("test".to_string()),
            })
        }))
    );

    Ok(())
}
