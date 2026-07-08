.PHONY: setup-debian setup-alpine build test run all

setup-debian:
	 sudo bash scripts/setup-debian.sh

setup-alpine:
	 sudo sh scripts/setup-alpine.sh

build:
	 cargo build --workspace

test:
	 cargo test --workspace

run:
	 cargo run -p toy_pake

all: build test run
