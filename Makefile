.PHONY: build_client build_server run_client run_server

build_client:
	cargo build --package client

build_server:
	cargo build --package server

run_client: build_client
	cargo run --package client

run_server: build_server
	cargo run --package server
