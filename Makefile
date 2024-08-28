format:
	cargo fmt --quiet

build:
	cargo build --quiet

docker-build:
	docker build -t rust-app .

docker-run:
	docker run -it --rm -p 8080:8080 rust-app

lint:
	cargo clippy --quiet

test:
	cargo test --quiet

run:
	cargo run

all: format lint test run