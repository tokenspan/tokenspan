api:
	@echo "Starting API server..."
	@cargo run --package tokenspan-api --bin tokenspan-api

cli:
	@echo "Starting CLI..."
	@cargo run --package tokenspan-cli --bin tokenspan-cli

test-api:
	cargo test --package tokenspan-api --test $(FILE) $(NAME) -- --exact