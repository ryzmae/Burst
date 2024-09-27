build:
	@if [ -d "target" ]; then rm -rf target; fi
	cargo build

run:
	cargo run

build-release:
	@if [ -d "target" ]; then rm -rf target; fi
	cargo build --release

run-release:
	cargo run --release

watch:
	cargo watch -x run
	
clean:
	cargo clean