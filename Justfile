build:
    nix build .

check:
  contrib/check.sh

gateway:
  contrib/run-local.py cargo run -p tulpje-gateway
handler:
  # specify METRICS_LISTEN_ADDR to avoid clashing with gateway
  METRICS_LISTEN_ADDR=0.0.0.0:9001 \
    contrib/run-local.py cargo run -p tulpje-handler

release *args:
  uv --project tools/release-tulpje run release-tulpje {{ args }}

sqlx-migrate: database-up
  contrib/run-local.py sqlx migrate run --source migrations

sqlx-prepare: database-up
  contrib/run-local.py cargo sqlx prepare --workspace

reset-db: database-up
  contrib/run-local.py sqlx database reset

up: build-docker
  docker compose --profile=full up

database-up:
  docker compose up -d postgres

services-up: (build-docker ".#docker-nirn-proxy" ".#docker-gateway-queue")
  docker compose up -d

services-down:
  docker compose down

build-docker *packages:
    contrib/build-docker.sh {{ packages }}
