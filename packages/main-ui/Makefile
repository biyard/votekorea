SERVICE ?= $(shell basename `git rev-parse --show-toplevel`)
VERSION ?= $(shell toml get Cargo.toml package.version | tr -d \")
COMMIT ?= $(shell git rev-parse --short HEAD)
ENV ?= local

ifeq ("$(ENV)","prod")
	RUST_LOG ?= info
endif

DOMAIN ?= localhost
RUST_LOG ?= debug
KAKAO_REDIRECT_URI ?= http://localhost:8080/oauth/kakao
WORKSPACE_ROOT ?= ../..
MAIN_API_ENDPOINT ?= https://api.votekorea.org

BUILD_ENV ?= ENV=$(ENV) VERSION=$(VERSION) COMMIT=$(COMMIT) RUST_LOG=$(RUST_LOG) DOMAIN=${DOMAIN} SERVICE=$(SERVICE) MAIN_API_ENDPOINT=$(MAIN_API_ENDPOINT) KAKAO_REDIRECT_URI=$(KAKAO_REDIRECT_URI)

setup.tool:
	cargo install cargo-binstall
	cargo binstall dioxus-cli
	cargo binstall toml-cli
	npm i -g tailwindcss

serve: clean public/tailwind.css
	$(BUILD_ENV) dx serve --addr 0.0.0.0 --platform web  --client-features web-only

run: clean public/tailwind.css
	$(BUILD_ENV) dx serve --fullstack

build: public/tailwind.css
	$(BUILD_ENV) dx build --release --fullstack --server-features lambda
	cp -r $(WORKSPACE_ROOT)/target/dx/$(SERVICE)/release/web $(ARTIFACT_DIR)
	mv $(ARTIFACT_DIR)/server $(ARTIFACT_DIR)/bootstrap

public/tailwind.css:
	echo "Skip building tailwind.css due to css reading bug"
	echo "" > ./public/tailwind.css
	# tailwindcss -i ./public/input.css -o ./public/tailwind.css

clean:
	rm -rf public/tailwind.css

build-docker: clean public/tailwind.css
	docker run -it --rm --name $(SERVICE) -v $(PWD)/../..:/app -w /app/packages/$(SERVICE) biyard/dioxus-docker bash -c 'source ~/.cargo/env && $(BUILD_ENV) dx build --release --fullstack --platform web --server-features lambda && cp -r /app/target/dx/$(SERVICE)/release/web /app/.build/$(SERVICE) && mv /app/.build/$(SERVICE)/server /app/.build/$(SERVICE)/bootstrap'
