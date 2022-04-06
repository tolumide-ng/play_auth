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

.PHONY: Build docker image for non-production environment (development, testing, staging)
play_build_dev:
	docker build . -t authey_dev --target=dev

# .PHONY: Docker compose application in non-production env
# play_compose_dev:
# 	docker compose up --build

.PHONY: Docker compose application in non-production environment (development, testing, staging)
play_dev:
	docker compose up

.PHONY: Build docker image for production environment
play_build_prod:
	# docker build 
	docker compose -f docker-compose.yml -f docker-compose.prod.yml up -d

.PHONY: Run application with docker compose in production
play_prod:
	docker compose -f docker-compose.prod.yml up

.PHONY: Run test locally (docker compose in not up)
test_play:
	docker compose run --rm web cargo test

.PHONY: Run test locally (docker compose is already up)
test_play_up:
	docker compose exec web cargo test