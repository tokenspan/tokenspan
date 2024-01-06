#[macro_export]
macro_rules! setup {
    ($state: ident, $server: ident) => {
        // Setup
        std::env::set_var("APP__ENV", "test");

        let docker = testcontainers_modules::testcontainers::clients::Cli::default();
        let node = docker.run(testcontainers_modules::postgres::Postgres::default());

        let conn_url = &format!(
            "postgres://postgres:postgres@localhost:{}/postgres",
            node.get_host_port_ipv4(5432)
        );

        let mut config = tokenspan_api::configs::AppConfig::new().expect("Failed to load config");
        config.database.url = conn_url.to_string();

        $state = tokenspan_api::state::AppState::new(&config).await?;
        let app = tokenspan_api::app::make_app_with_state(config, $state.clone()).await?;
        $server = axum_test::TestServer::new(app)?;
    };
}
