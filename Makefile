all: build

build:
	cargo build
test:
	cargo test -- --nocapture 
