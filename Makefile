api:
	@echo "Starting API server..."
	@cargo run --package tokenspan-api --bin tokenspan-api

cli:
	@echo "Starting CLI..."
	@cargo run --package tokenspan-cli --bin tokenspan-cli