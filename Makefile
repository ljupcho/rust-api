.DEFAULT_GOAL := help
.PHONY: help build

help: ## Help information
	@echo "------------------------------------------------------------------------"
	@echo "Shop project"
	@echo "------------------------------------------------------------------------"
	@echo " Get started:"
	@echo "  - make build            # Build project with all containers"
	@echo "------------------------------------------------------------------------"
	@echo " Target list:"
	@echo "------------------------------------------------------------------------"
	@grep -E '^[a-zA-Z0-9_/%\-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-25s\033[0m %s\n", $$1, $$2}'
	@printf "\nDocs:\n\n"
	@printf "See docs folder for detailed documentation and guides.\n\n"

all: run

build: ## Build docker containers - full application
	docker-compose up -d --build

up: ## Bring down the docker app
	docker-compose up -d

stop: ## Bring down the docker app
	docker-compose stop

stop-rm: ## Bring down the docker app and clean up
	docker-compose down --rmi all

migrate: # Run database migrations
	cd migration & sea-orm-cli migrate up

build-image-api: ## Build the image for api to be used in k8s.
	docker build -t rust-api:latest . -f ./docker/api/Dockerfile

build-image-migration: ## Build the image for migration to be used in k8s.
	docker build -t rust-migration:latest . -f ./docker/migration/Dockerfile
