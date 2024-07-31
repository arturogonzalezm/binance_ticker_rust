# Default target
.PHONY: help
help:  ## Show this help.
	@echo "Usage:"
	@echo "  make <target>"
	@echo ""
	@echo "Targets:"
	@awk 'BEGIN {FS = ":.*##"; printf "\033[36m%-15s\033[0m %s\n", "help", "Show this help."} /^[a-zA-Z_-]+:.*##/ { printf "\033[36m%-15s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

.PHONY: all build up down clean logs db-shell

# Default target
all: build up

rebuild: clean build up

# Build the Docker images without using cache
build:
	@echo "Building Docker images..."
	docker compose build --no-cache

# Start the Docker containers in detached mode
up:
	@echo "Starting Docker containers..."
	docker compose up -d

# Stop the Docker containers
down:
	@echo "Stopping Docker containers..."
	docker compose down

# Remove Docker volumes
clean:
	@echo "Removing Docker volumes..."
	docker compose down -v
	docker volume rm binanceticker_pgdata || true

# Show logs for all containers
logs:
	@echo "Showing logs for all containers..."
	docker compose logs -f

db-shell:
	docker exec -it binanceticker_pgdata psql -U postgres -d postgres
