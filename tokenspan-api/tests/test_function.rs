use anyhow::Result;
use axum_test::http::{HeaderName, HeaderValue};
use axum_test::TestServer;
use googletest::matchers::{anything, eq, some};
use googletest::prelude::*;
use graphql_client::{GraphQLQuery, Response};
use tokenspan_api::domains::dto::FunctionCreateInput;

use tokenspan_api::domains::models::UserRole;
use tokenspan_api::state::AppState;

use crate::graphql::{
    create_function_mutation, delete_function_mutation, get_function_query, get_functions_query,
    update_function_mutation, CreateFunctionMutation, DeleteFunctionMutation, GetFunctionQuery,
    GetFunctionsQuery, UpdateFunctionMutation,
};

mod common;
mod graphql;

macro_rules! create_function {
    ($state: ident, name = $name: literal, user_id = $user_id: expr) => {
        $state
            .function_service
            .create(
                FunctionCreateInput {
                    name: $name.to_string(),
                    description: $name.to_string(),
                    parameters: serde_json::json!({}),
                    response: None,
                },
                $user_id,
            )
            .await?
    };
}

macro_rules! make_request {
    ($server: ident, $token: expr, $variables: expr) => {{
        let req_body = GetFunctionsQuery::build_query($variables);
        let resp = $server
            .post("graphql")
            .add_header(
                HeaderName::from_static("authorization"),
                HeaderValue::from_str(format!("Bearer {}", $token).as_str())?,
            )
            .json(&req_body)
            .await;

        resp.json::<Response<get_functions_query::ResponseData>>()
    }};
}

#[tokio::test]
async fn test_paginate_forward_function() -> Result<()> {
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

    create_function!(state, name = "test", user_id = auth_fixture.user.id);
    create_function!(state, name = "test1", user_id = auth_fixture.user.id);

    // Create provider
    let variables = get_functions_query::Variables {
        args: get_functions_query::FunctionArgs {
            first: None,
            after: None,
            last: Some(1),
            before: None,
        },
    };
    let resp = make_request!(server, auth_fixture.token, variables);
    assert_that!(
        resp.data,
        some(pat!(get_functions_query::ResponseData {
            functions: pat!(get_functions_query::GetFunctionsQueryFunctions {
                nodes: contains_each![pat!(get_functions_query::GetFunctionsQueryFunctionsNodes {
                    id: anything(),
                    name: eq("test1".to_string()),
                    owner_id: anything(),
                    created_at: anything(),
                }),],
                total_nodes: eq(2),
                page_info: pat!(get_functions_query::GetFunctionsQueryFunctionsPageInfo {
                    has_next_page: eq(false),
                    has_previous_page: eq(true),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    // Create provider
    let cursor = resp.data.unwrap().functions.page_info.end_cursor;
    let variables = get_functions_query::Variables {
        args: get_functions_query::FunctionArgs {
            first: None,
            after: None,
            last: Some(1),
            before: cursor,
        },
    };
    let resp = make_request!(server, auth_fixture.token, variables);
    assert_that!(
        resp.data,
        some(pat!(get_functions_query::ResponseData {
            functions: pat!(get_functions_query::GetFunctionsQueryFunctions {
                nodes: contains_each![pat!(get_functions_query::GetFunctionsQueryFunctionsNodes {
                    id: anything(),
                    name: eq("test".to_string()),
                    owner_id: anything(),
                    created_at: anything(),
                }),],
                total_nodes: eq(2),
                page_info: pat!(get_functions_query::GetFunctionsQueryFunctionsPageInfo {
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
async fn test_paginate_backward_function() -> Result<()> {
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

    create_function!(state, name = "test", user_id = auth_fixture.user.id);
    create_function!(state, name = "test1", user_id = auth_fixture.user.id);

    // Create provider
    let variables = get_functions_query::Variables {
        args: get_functions_query::FunctionArgs {
            first: None,
            after: None,
            last: Some(1),
            before: None,
        },
    };
    let resp = make_request!(server, auth_fixture.token, variables);
    println!("resp: {:?}", resp);
    assert_that!(
        resp.data,
        some(pat!(get_functions_query::ResponseData {
            functions: pat!(get_functions_query::GetFunctionsQueryFunctions {
                nodes: contains_each![pat!(get_functions_query::GetFunctionsQueryFunctionsNodes {
                    id: anything(),
                    name: eq("test1".to_string()),
                    owner_id: anything(),
                    created_at: anything(),
                }),],
                total_nodes: eq(2),
                page_info: pat!(get_functions_query::GetFunctionsQueryFunctionsPageInfo {
                    has_next_page: eq(false),
                    has_previous_page: eq(true),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    // Create provider
    let cursor = resp.data.unwrap().functions.page_info.end_cursor;
    let variables = get_functions_query::Variables {
        args: get_functions_query::FunctionArgs {
            first: None,
            after: None,
            last: Some(1),
            before: cursor,
        },
    };
    let resp = make_request!(server, auth_fixture.token, variables);
    assert_that!(
        resp.data,
        some(pat!(get_functions_query::ResponseData {
            functions: pat!(get_functions_query::GetFunctionsQueryFunctions {
                nodes: contains_each![pat!(get_functions_query::GetFunctionsQueryFunctionsNodes {
                    id: anything(),
                    name: eq("test".to_string()),
                    owner_id: anything(),
                    created_at: anything(),
                }),],
                total_nodes: eq(2),
                page_info: pat!(get_functions_query::GetFunctionsQueryFunctionsPageInfo {
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
async fn test_get_function_by_id() -> Result<()> {
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

    let function = create_function!(state, name = "test1", user_id = auth_fixture.user.id);

    // Get provider
    let variables = get_function_query::Variables {
        function_id: function.id,
    };
    let req_body = GetFunctionQuery::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<get_function_query::ResponseData>>();

    assert_that!(
        resp.data,
        some(pat!(get_function_query::ResponseData {
            function: some(pat!(get_function_query::GetFunctionQueryFunction {
                id: anything(),
                name: eq("test1".to_string()),
                description: eq("test1".to_string()),
                parameters: anything(),
                response: anything(),
                owner_id: anything(),
                created_at: anything(),
            }))
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_create_function() -> Result<()> {
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

    // Create function
    let variables = create_function_mutation::Variables {
        input: create_function_mutation::FunctionCreateInput {
            name: "test".to_string(),
            description: "test".to_string(),
            parameters: serde_json::json!({}),
            response: None,
        },
    };
    let req_body = CreateFunctionMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<create_function_mutation::ResponseData>>();

    assert_that!(
        resp.data,
        some(pat!(create_function_mutation::ResponseData {
            create_function: pat!(
                create_function_mutation::CreateFunctionMutationCreateFunction {
                    id: anything(),
                    name: eq("test".to_string()),
                    created_at: anything(),
                    updated_at: anything(),
                }
            )
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_update_function() -> Result<()> {
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

    let function = create_function!(state, name = "test", user_id = auth_fixture.user.id);

    // Update function
    let variables = update_function_mutation::Variables {
        update_function_id: function.id,
        input: update_function_mutation::FunctionUpdateInput {
            name: Some("test1".to_string()),
            description: Some("test1".to_string()),
            parameters: Some(serde_json::json!({
                "type": "object",
            })),
            response: Some(serde_json::json!({
                "type": "object",
            })),
        },
    };
    let req_body = UpdateFunctionMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<update_function_mutation::ResponseData>>();

    assert_that!(
        resp.data,
        some(pat!(update_function_mutation::ResponseData {
            update_function: pat!(
                update_function_mutation::UpdateFunctionMutationUpdateFunction {
                    id: anything(),
                    name: eq("test1".to_string()),
                    description: eq("test1".to_string()),
                    parameters: anything(),
                    response: anything(),
                    owner_id: anything(),
                    created_at: anything(),
                    updated_at: anything(),
                }
            )
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_delete_function() -> Result<()> {
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

    let function = create_function!(state, name = "test", user_id = auth_fixture.user.id);

    // Update function
    let variables = delete_function_mutation::Variables {
        delete_function_id: function.id,
    };
    let req_body = DeleteFunctionMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<delete_function_mutation::ResponseData>>();

    assert_that!(
        resp.data,
        some(pat!(delete_function_mutation::ResponseData {
            delete_function: pat!(
                delete_function_mutation::DeleteFunctionMutationDeleteFunction {
                    id: anything(),
                    name: eq("test".to_string()),
                    description: eq("test".to_string()),
                    parameters: anything(),
                    response: anything(),
                    owner_id: anything(),
                    created_at: anything(),
                    updated_at: anything(),
                }
            )
        }))
    );

    Ok(())
}
