SERVICE:= ipython-diff
SRC_DIR:= /work/ipython_diff/src

PyTorch:=/work/ipython_diff/data/avilen/PyTorch
TenserFlow:=/work/ipython_diff/data/avilen/TensorFlow

TEST_DATA_NAME:= /work/ipython_diff/data/test-data
TEST_NUMBER:=04
ISPY:= false

help:
	@echo cat Makefile

# for user
ISPYFile = 
ifeq ($(ISPY),true)
	ISPYFile = $(PyTorch)
else
	ISPYFile = $(TenserFlow)
endif

# converter
.PHONY: serve
build:
	cargo check
	cargo build
serve:
	@cargo run $(TEST_DATA_NAME) $(ISPYFile) $(TEST_NUMBER)

naitive-build:
	@rustc src/main.rs

native-run:
	./main

# env
.PHONY: up
up:
	docker-compose up

.PHONY: up/d
up/d:
	docker-compose up -d
	docker exec -it $(SERVICE) /bin/bash

.PHONY: stop
stop:
	docker-compose stop

.PHONY: down
down:
	docker-compose down

.PHONY: rm
rm: 
	docker-compose down --rmi all --volumes --remove-orphans

.PHONY: logs
logs:
	docker-compose logs -f

# health check
.PHONY: docker-ichekc
docker-ichekc:
	docker image ls
.PHONY: docker-chekc
docker-chekc:
	docker ps -a

# dokcer native
.PHONY: docker-build
docker-build:
	docker build -t $(IMAGE_NAME):$(TAG) .

.PHONY: docker-up
docker-up: 
	# this command need arg = IID
	# chekc IID: make docker-ichekc
	docker run --name $(COTEINA_NAME) -itd $(IID)
	docker exec -it $(COTEINA_NAME) bash

.PHONY: docker-stop
docker-stop:
	docker stop $(COTEINA_NAME)

.PHONY: docker-down
docker-down:
	docker stop $(COTEINA_NAME)
	docker rm $(COTEINA_NAME)
	docker rmi $(IMAGE_NAME):$(TAG)