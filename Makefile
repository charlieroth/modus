# Check to see if we can use ash, in Alpine images, or default to BASH.
SHELL_PATH = /bin/ash
SHELL = $(if $(wildcard $(SHELL_PATH)),/bin/ash,/bin/bash)

# ==============================================================================
# Define Dependencies

ALPINE          := alpine:3.20
POSTGRES        := postgres:16.4
GENI            := ghcr.io/emilpriver/geni:v1.1.4

REMINDERS_APP   := reminders
BASE_IMAGE_NAME := localhost/charlieroth
VERSION         := 0.1.0
REMINDERS_IMAGE := $(BASE_IMAGE_NAME)/$(REMINDERS_APP):$(VERSION)

# ==============================================================================
# Dependencies

dev-brew:
	brew update
	brew list pgcli || brew install pgcli
	brew list watch || brew install watch
	brew list geni || brew install geni

dev-docker:
	docker pull $(ALPINE) & \
	docker pull $(POSTGRES) & \
	docker pull $(GENI) & \
	wait;

# ==============================================================================
# Docker Compose

compose-dev-up:
	docker compose --profile dev up

compose-db-up:
	docker compose --profile db up

compose-dev-down:
	docker compose --profile dev down

compose-db-down:
	docker compose --profile db down

# ==============================================================================
# Admin

pgcli:
	pgcli postgresql://postgres:postgres@localhost:5432/reminders_dev

liveness:
	curl -il http://localhost:8080/liveness

readiness:
	curl -il http://localhost:8080/readiness

# ==============================================================================
# Cargo

lint:
	cargo run clippy

test:
	cargo test

clean:
	cargo clean