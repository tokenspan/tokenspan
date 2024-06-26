use anyhow::Result;
use axum_test::http::{HeaderName, HeaderValue};
use axum_test::TestServer;
use googletest::matchers::{anything, eq, some};
use googletest::prelude::*;
use graphql_client::{GraphQLQuery, Response};

use tokenspan_api::domains::dto::{ModelCreateInput, PricingInput, ProviderCreateInput};
use tokenspan_api::domains::models::UserRole;
use tokenspan_api::state::AppState;

use crate::graphql::{
    create_model_mutation, delete_model_mutation, get_model_query, get_models_query,
    update_model_mutation, CreateModelMutation, DeleteModelMutation, GetModelQuery, GetModelsQuery,
    UpdateModelMutation,
};

mod common;
mod graphql;

macro_rules! create_model {
    ($state: ident, name = $name: literal, provider_id = $provider_id: expr) => {
        $state
            .model_service
            .create(ModelCreateInput {
                name: $name.to_string(),
                description: $name.to_string(),
                slug: $name.to_string(),
                context: 256,
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
                training_at: Default::default(),
                provider_id: $provider_id,
            })
            .await?;
    };
}

macro_rules! make_request {
    ($server: ident, $token: expr, $variables: ident) => {{
        let req_body = GetModelsQuery::build_query($variables);
        let resp = $server
            .post("graphql")
            .add_header(
                HeaderName::from_static("authorization"),
                HeaderValue::from_str(format!("Bearer {}", $token).as_str())?,
            )
            .json(&req_body)
            .await;
        resp.json::<Response<get_models_query::ResponseData>>()
    }};
}

#[tokio::test]
async fn test_paginate_forward_models() -> Result<()> {
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
            base_url: "http://localhost:8080".to_string(),
        })
        .await?;

    // Create models
    create_model!(state, name = "gpt-3.5", provider_id = provider_fixture.id);
    create_model!(state, name = "gpt-4", provider_id = provider_fixture.id);
    create_model!(state, name = "gpt-5", provider_id = provider_fixture.id);

    // Get models
    let variables = get_models_query::Variables {
        args: get_models_query::ModelArgs {
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
        some(pat!(get_models_query::ResponseData {
            models: pat!(get_models_query::GetModelsQueryModels {
                nodes: contains_each![pat!(get_models_query::GetModelsQueryModelsNodes {
                    id: anything(),
                    name: eq("gpt-3.5".to_string()),
                }),],
                total_nodes: eq(3),
                page_info: pat!(get_models_query::GetModelsQueryModelsPageInfo {
                    has_next_page: eq(true),
                    has_previous_page: eq(false),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    // Get models
    let cursor = resp.data.unwrap().models.page_info.end_cursor;
    let variables = get_models_query::Variables {
        args: get_models_query::ModelArgs {
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
        some(pat!(get_models_query::ResponseData {
            models: pat!(get_models_query::GetModelsQueryModels {
                nodes: contains_each![pat!(get_models_query::GetModelsQueryModelsNodes {
                    id: anything(),
                    name: eq("gpt-4".to_string()),
                }),],
                total_nodes: eq(3),
                page_info: pat!(get_models_query::GetModelsQueryModelsPageInfo {
                    has_next_page: eq(true),
                    has_previous_page: eq(false),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    // Get models
    let cursor = resp.data.unwrap().models.page_info.end_cursor;
    let variables = get_models_query::Variables {
        args: get_models_query::ModelArgs {
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
        some(pat!(get_models_query::ResponseData {
            models: pat!(get_models_query::GetModelsQueryModels {
                nodes: contains_each![pat!(get_models_query::GetModelsQueryModelsNodes {
                    id: anything(),
                    name: eq("gpt-5".to_string()),
                }),],
                total_nodes: eq(3),
                page_info: pat!(get_models_query::GetModelsQueryModelsPageInfo {
                    has_next_page: eq(false),
                    has_previous_page: eq(false),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    // Get models
    let cursor = resp.data.unwrap().models.page_info.end_cursor;
    let variables = get_models_query::Variables {
        args: get_models_query::ModelArgs {
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
        some(pat!(get_models_query::ResponseData {
            models: pat!(get_models_query::GetModelsQueryModels {
                nodes: empty(),
                total_nodes: eq(3),
                page_info: pat!(get_models_query::GetModelsQueryModelsPageInfo {
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
async fn test_paginate_backward_models() -> Result<()> {
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
            base_url: "http://localhost:8080".to_string(),
        })
        .await?;

    // Create models
    create_model!(state, name = "gpt-3.5", provider_id = provider_fixture.id);
    create_model!(state, name = "gpt-4", provider_id = provider_fixture.id);
    create_model!(state, name = "gpt-5", provider_id = provider_fixture.id);

    // Get models
    let variables = get_models_query::Variables {
        args: get_models_query::ModelArgs {
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
        some(pat!(get_models_query::ResponseData {
            models: pat!(get_models_query::GetModelsQueryModels {
                nodes: contains_each![pat!(get_models_query::GetModelsQueryModelsNodes {
                    id: anything(),
                    name: eq("gpt-5".to_string()),
                }),],
                total_nodes: eq(3),
                page_info: pat!(get_models_query::GetModelsQueryModelsPageInfo {
                    has_next_page: eq(false),
                    has_previous_page: eq(true),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    // Get models
    let cursor = resp.data.unwrap().models.page_info.end_cursor;
    let variables = get_models_query::Variables {
        args: get_models_query::ModelArgs {
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
        some(pat!(get_models_query::ResponseData {
            models: pat!(get_models_query::GetModelsQueryModels {
                nodes: contains_each![pat!(get_models_query::GetModelsQueryModelsNodes {
                    id: anything(),
                    name: eq("gpt-4".to_string()),
                }),],
                total_nodes: eq(3),
                page_info: pat!(get_models_query::GetModelsQueryModelsPageInfo {
                    has_next_page: eq(false),
                    has_previous_page: eq(true),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    // Get models
    let cursor = resp.data.unwrap().models.page_info.end_cursor;
    let variables = get_models_query::Variables {
        args: get_models_query::ModelArgs {
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
        some(pat!(get_models_query::ResponseData {
            models: pat!(get_models_query::GetModelsQueryModels {
                nodes: contains_each![pat!(get_models_query::GetModelsQueryModelsNodes {
                    id: anything(),
                    name: eq("gpt-3.5".to_string()),
                }),],
                total_nodes: eq(3),
                page_info: pat!(get_models_query::GetModelsQueryModelsPageInfo {
                    has_next_page: eq(false),
                    has_previous_page: eq(false),
                    start_cursor: anything(),
                    end_cursor: anything(),
                }),
            })
        }))
    );

    // Get models
    let cursor = resp.data.unwrap().models.page_info.end_cursor;
    let variables = get_models_query::Variables {
        args: get_models_query::ModelArgs {
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
        some(pat!(get_models_query::ResponseData {
            models: pat!(get_models_query::GetModelsQueryModels {
                nodes: empty(),
                total_nodes: eq(3),
                page_info: pat!(get_models_query::GetModelsQueryModelsPageInfo {
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
async fn test_get_model_by_id() -> Result<()> {
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
            base_url: "http://localhost:8080".to_string(),
        })
        .await?;

    // Create models
    let model_fixture = state
        .model_service
        .create(ModelCreateInput {
            name: "test".to_string(),
            description: "test".to_string(),
            slug: "test".to_string(),
            context: 256,
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
            training_at: Default::default(),
            provider_id: provider_fixture.id,
        })
        .await?;

    // Cet model
    let variables = get_model_query::Variables {
        model_id: model_fixture.id,
    };
    let req_body = GetModelQuery::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<get_model_query::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(get_model_query::ResponseData {
            model: some(pat!(get_model_query::GetModelQueryModel {
                id: anything(),
                name: eq("test".to_string()),
                slug: eq("test".to_string()),
                provider_id: eq(provider_fixture.id),
                description: eq("test".to_string()),
                output_pricing: pat!(get_model_query::GetModelQueryModelOutputPricing {
                    price: eq(0.06),
                    tokens: eq(1),
                    currency: eq("USD".to_string()),
                }),
                input_pricing: pat!(get_model_query::GetModelQueryModelInputPricing {
                    price: eq(0.06),
                    tokens: eq(1),
                    currency: eq("USD".to_string()),
                }),
                context: eq(256),
                training_at: anything(),
                created_at: anything(),
                updated_at: anything(),
            }))
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_create_model() -> Result<()> {
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
            base_url: "http://localhost:8080".to_string(),
        })
        .await?;

    // Create model
    let variables = create_model_mutation::Variables {
        input: create_model_mutation::ModelCreateInput {
            name: "test".to_string(),
            slug: "test".to_string(),
            description: "test".to_string(),
            context: 1,
            input_pricing: create_model_mutation::PricingInput {
                currency: "USD".to_string(),
                price: 0.06,
                tokens: 1,
            },
            output_pricing: create_model_mutation::PricingInput {
                currency: "USD".to_string(),
                price: 0.06,
                tokens: 1,
            },
            training_at: Default::default(),
            provider_id: provider_fixture.id,
        },
    };
    let req_body = CreateModelMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<create_model_mutation::ResponseData>>();

    assert_that!(
        resp.data,
        some(pat!(create_model_mutation::ResponseData {
            create_model: pat!(create_model_mutation::CreateModelMutationCreateModel {
                id: anything(),
                name: eq("test".to_string()),
                slug: eq("test".to_string()),
                provider_id: eq(provider_fixture.id),
                description: eq("test".to_string()),
                output_pricing: pat!(
                    create_model_mutation::CreateModelMutationCreateModelOutputPricing {
                        price: eq(0.06),
                        tokens: eq(1),
                        currency: eq("USD".to_string()),
                    }
                ),
                input_pricing: pat!(
                    create_model_mutation::CreateModelMutationCreateModelInputPricing {
                        price: eq(0.06),
                        tokens: eq(1),
                        currency: eq("USD".to_string()),
                    }
                ),
                context: eq(1),
                training_at: anything(),
                created_at: anything(),
                updated_at: anything(),
            })
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_update_model() -> Result<()> {
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
            base_url: "http://localhost:8080".to_string(),
        })
        .await?;

    // Create model
    let model_fixture = state
        .model_service
        .create(ModelCreateInput {
            name: "test".to_string(),
            description: "test".to_string(),
            slug: "test".to_string(),
            context: 256,
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
            training_at: Default::default(),
            provider_id: provider_fixture.id,
        })
        .await?;

    // Update model
    let variables = update_model_mutation::Variables {
        update_model_id: model_fixture.id,
        input: update_model_mutation::ModelUpdateInput {
            name: Some("test1".to_string()),
            slug: Some("test1".to_string()),
            description: Some("test1".to_string()),
            context: Some(256),
            input_pricing: Some(update_model_mutation::PricingInput {
                currency: "JYP".to_string(),
                price: 0.07,
                tokens: 1,
            }),
            output_pricing: Some(update_model_mutation::PricingInput {
                currency: "JYP".to_string(),
                price: 0.07,
                tokens: 1,
            }),
            training_at: Default::default(),
        },
    };
    let req_body = UpdateModelMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<update_model_mutation::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(update_model_mutation::ResponseData {
            update_model: pat!(update_model_mutation::UpdateModelMutationUpdateModel {
                id: eq(model_fixture.id),
                name: eq("test1".to_string()),
                slug: eq("test1".to_string()),
                provider_id: eq(provider_fixture.id),
                description: eq("test1".to_string()),
                output_pricing: pat!(
                    update_model_mutation::UpdateModelMutationUpdateModelOutputPricing {
                        price: eq(0.07),
                        tokens: eq(1),
                        currency: eq("JYP".to_string()),
                    }
                ),
                input_pricing: pat!(
                    update_model_mutation::UpdateModelMutationUpdateModelInputPricing {
                        price: eq(0.07),
                        tokens: eq(1),
                        currency: eq("JYP".to_string()),
                    }
                ),
                created_at: anything(),
                updated_at: anything(),
            })
        }))
    );

    Ok(())
}

#[tokio::test]
async fn test_delete_model() -> Result<()> {
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
            base_url: "http://localhost:8080".to_string(),
        })
        .await?;

    // Create model
    let model_fixture = state
        .model_service
        .create(ModelCreateInput {
            name: "test".to_string(),
            description: "test".to_string(),
            slug: "test".to_string(),
            context: 256,
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
            training_at: Default::default(),
            provider_id: provider_fixture.id,
        })
        .await?;

    // Update model
    let variables = delete_model_mutation::Variables {
        delete_model_id: model_fixture.id,
    };
    let req_body = DeleteModelMutation::build_query(variables);
    let resp = server
        .post("graphql")
        .add_header(
            HeaderName::from_static("authorization"),
            HeaderValue::from_str(format!("Bearer {}", auth_fixture.token).as_str())?,
        )
        .json(&req_body)
        .await;
    let resp = resp.json::<Response<delete_model_mutation::ResponseData>>();

    // Assert
    assert_that!(
        resp.data,
        some(pat!(delete_model_mutation::ResponseData {
            delete_model: pat!(delete_model_mutation::DeleteModelMutationDeleteModel {
                id: eq(model_fixture.id),
                name: eq("test".to_string()),
                slug: eq("test".to_string()),
                provider_id: eq(provider_fixture.id),
                description: eq("test".to_string()),
                output_pricing: pat!(
                    delete_model_mutation::DeleteModelMutationDeleteModelOutputPricing {
                        price: eq(0.06),
                        tokens: eq(1),
                        currency: eq("USD".to_string()),
                    }
                ),
                input_pricing: pat!(
                    delete_model_mutation::DeleteModelMutationDeleteModelInputPricing {
                        price: eq(0.06),
                        tokens: eq(1),
                        currency: eq("USD".to_string()),
                    }
                ),
                created_at: anything(),
                updated_at: anything(),
            })
        }))
    );

    Ok(())
}
