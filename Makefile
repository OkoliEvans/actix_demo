DB_DOCKER_CONTAINER=actixwebdemo_container

install:
	cargo add actix-web
	cargo add actix-cors
	cargo add serde_json
	cargo add serde --features derive
	cargo add chrono --features serde
	cargo add env_logger
	cargo add dotenv
	cargo add uuid --features "serde v4"
	cargo add sqlx --features "runtime-async-std-native-tls postgres chrono uuid"
	cargo add jsonwebtoken
	cargo add argon2
	cargo add rand_core --features "std"
# SQLX-CLI
	cargo install sqlx-cli

build:
	cargo build

create_migrations:
	sqlx migrate add -r init

migrate-up:
	sqlx migrate run

migrate-down:
	sqlx migrate revert

create_docker_container: 
	docker run --name ${DB_DOCKER_CONTAINER} -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:12-alpine

create_postgres_db:
	docker exec -it ${DB_DOCKER_CONTAINER} createdb --username=root --owner=root actixwebdemo

start_docker_db:
	docker start ${DB_DOCKER_CONTAINER}

run:
	cargo run