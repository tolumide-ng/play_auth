.PHONY: help
help:
	@echo "----------------------------------------"
	@echo "make help: wil print this message"
	@echo "make migrate: will the auth workspace's migrations"
	@echo "----------------------------------------"

.PHONY: Run auth migrations on the database
migrate:
	cd auth && sqlx migrate run

.PHONY: Start the auth workspace
run_auth:
	cargo run -p auth

.PHONY: Update sqlx-data.json for the workspace
update_migrations:
	cargo sqlx prepare --merged

.PHONY: Run all tests (adding the arguments --features "tests" allows all integration test features to be implemnted)
test_all:
	cargo test -p auth --features "test"
