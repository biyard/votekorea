ENV ?= local
BASE_DOMAIN ?= votekorea.org
DOMAIN ?= $(ENV).$(BASE_DOMAIN)

PROJECT ?= $(shell basename `git rev-parse --show-toplevel`)
SERVICE ?= main-ui
COMMIT ?= $(shell git rev-parse --short HEAD)

ACCESS_KEY_ID ?= $(shell aws configure get aws_access_key_id $(AWS_FLAG))
SECRET_ACCESS_KEY ?= $(shell aws configure get aws_secret_access_key $(AWS_FLAG))
REGION ?= $(shell aws configure get region $(AWS_FLAG))
CDN_ID ?= $(shell aws cloudfront list-distributions $(AWS_FLAG) --query "DistributionList.Items[*].{id:Id,test:AliasICPRecordals[?CNAME=='$(DOMAIN)']}" --output json |jq '. | map(select(.test | length > 0))[0] | .id' | tr -d \")
WORKSPACE_ROOT ?= $(PWD)
AWS_ACCOUNT_ID ?= $(shell aws sts get-caller-identity --query "Account" --output text $(AWS_FLAG))
VPC_ID ?= $(shell aws ec2 describe-vpcs --query "Vpcs[0].VpcId" --output json $(AWS_FLAG) | tr -d \")

STACK ?= $(PROJECT)-$(SERVICE)-$(ENV)-stack

BUILD_CDK_ENV ?= AWS_ACCESS_KEY_ID=$(ACCESS_KEY_ID) AWS_SECRET_ACCESS_KEY=$(SECRET_ACCESS_KEY) AWS_REGION=$(REGION) DOMAIN=$(DOMAIN) WORKSPACE_ROOT=$(WORKSPACE_ROOT) SERVICE=$(SERVICE) VPC_ID=$(VPC_ID) AWS_ACCOUNT_ID=$(AWS_ACCOUNT_ID) COMMIT=$(COMMIT) ENV=$(ENV) BASE_DOMAIN=$(BASE_DOMAIN) PROJECT=$(PROJECT) STACK=$(STACK)

run:
	cd packages/$(SERVICE) && make run

serve:
	cd packages/$(SERVICE) && make serve

clean:
	rm -rf .build/$(SERVICE)

.PHONY: deploy-if-needed
deploy-if-needed:
	$(eval DEPLOYED_VERSION := $(shell curl https://$(DOMAIN)/version | tr -d \" | cut -d'-' -f1))
	$(eval CURRENT_VERSION := $(shell toml get packages/$(SERVICE)/Cargo.toml package.version | tr -d \"))
	$(eval CMD := $(shell if [ "$(DEPLOYED_VERSION)" != "$(CURRENT_VERSION)" ] ; then echo "OLD_VERSION=\"$(DEPLOYED_VERSION)\" NEW_VERSION=\"$(CURRENT_VERSION)\" make deploy"; else echo "echo \"deployed version: $(DEPLOYED_VERSION), current version: $(CURRENT_VERSION), already latest version\""; fi))
	$(CMD)

.PHONY: deploy-web-if-needed
deploy-web-if-needed:
	$(eval DEPLOYED_VERSION := $(shell curl https://$(DOMAIN)/api/version | tr -d \" | cut -d'-' -f1))
	$(eval CURRENT_VERSION := $(shell toml get packages/$(SERVICE)/Cargo.toml package.version | tr -d \"))
	$(eval CMD := $(shell if [ "$(DEPLOYED_VERSION)" != "$(CURRENT_VERSION)" ] ; then echo "OLD_VERSION=\"$(DEPLOYED_VERSION)\" NEW_VERSION=\"$(CURRENT_VERSION)\" make deploy-web"; else echo "echo \"deployed version: $(DEPLOYED_VERSION), current version: $(CURRENT_VERSION), already latest version\""; fi))
	$(CMD)

.PHONY: deploy deploy-web
deploy: build cdk-deploy
deploy-web: build cdk-deploy s3-deploy

.PHONY: build
build: clean
	mkdir -p .build
	cd packages/$(SERVICE) && ENV=$(ENV) ARTIFACT_DIR=$(PWD)/.build/$(SERVICE) make build

deps/rust-sdk/cdk/node_modules:
	cd deps/rust-sdk/cdk && npm install

cdk-deploy: deps/rust-sdk/cdk/node_modules
	cd deps/rust-sdk/cdk && $(BUILD_CDK_ENV) CODE_PATH=$(PWD)/.build/$(SERVICE) npm run build
	cd deps/rust-sdk/cdk && $(BUILD_CDK_ENV) CODE_PATH=$(PWD)/.build/$(SERVICE) cdk synth
	cd deps/rust-sdk/cdk && $(BUILD_CDK_ENV) CODE_PATH=$(PWD)/.build/$(SERVICE) cdk deploy --require-approval never $(AWS_FLAG) --all

s3-deploy:
	cp -r packages/$(SERVICE)/public/* .build/$(SERVICE)/public
	aws s3 sync .build/$(SERVICE)/public s3://$(DOMAIN) $(AWS_FLAG)
	aws cloudfront create-invalidation --distribution-id $(CDN_ID) --paths "/*" $(AWS_FLAG) > /dev/null

.PHONY: postgres
postgres:
	docker-compose up -d
