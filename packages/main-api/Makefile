PROJECT ?= referendum
SERVICE ?= main-api
ENV ?= local
BASE_DOMAIN ?= localhost
DOMAIN ?= localhost
DATABASE_TYPE ?= postgres
DATABASE_URL ?= postgres://postgres:postgres@localhost/referendum
AWS_FLAG ?= --profile vox

WORKSPACE_ROOT ?= $(PWD)/../..
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text)
COMMIT ?= $(shell git rev-parse --short HEAD)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")

ifeq ("$(ENV)","prod")
	RUST_LOG ?= info
endif

ifeq ("$(ENV)","dev")
	RUST_LOG ?= debug
endif

RUST_LOG ?= debug
AWS_ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
AWS_SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
AWS_REGION ?= $(shell aws configure get region)

BUILD_ENV ?= ENV=$(ENV) RUST_LOG=$(RUST_LOG) AWS_ACCESS_KEY_ID=$(AWS_ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(AWS_SECRET_ACCESS_KEY) AWS_REGION=$(AWS_REGION) VERSION=$(VERSION) COMMIT=$(COMMIT) SECRET_TOKEN=$(SECRET_TOKEN) DOMAIN=$(DOMAIN) DATABASE_TYPE=$(DATABASE_TYPE) DATABASE_URL=$(DATABASE_URL) AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) BASE_DOMAIN=$(BASE_DOMAIN)

run:
	$(BUILD_ENV) cargo watch -x run -w src

build:
	$(BUILD_ENV) cargo build --release -p $(SERVICE) --features lambda
	mkdir -p $(WORKSPACE_ROOT)/.build/$(SERVICE)
	cp $(WORKSPACE_ROOT)/target/release/$(SERVICE) $(WORKSPACE_ROOT)/.build/$(SERVICE)/bootstrap
