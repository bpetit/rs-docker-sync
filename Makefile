MAKEFILE := $(abspath $(lastword $(MAKEFILE_LIST)))
PROJECT := $(dir $(MAKEFILE))
CARGO := $(PROJECT)/Cargo.toml

export CARGO

default: build

build:
	@cargo build --manifest-path $(CARGO)

run:
	cargo run

clean:
	cargo clean

publish:
	cargo publish
