build:
	@if [ -d "target" ]; then rm -rf target; fi
	cargo build
	clear
	./target/debug/Burst

run:
	clear
	cargo run

build-release:
	clear
	@if [ -d "target" ]; then rm -rf target; fi
	cargo build --release
	clear
	./target/release/Burst

run-release:
	clear
	cargo run --release

watch:
	clear
	cargo watch -x run

clean:
	clear
	cargo clean

test:
	clear
	cargo test --bin Burst -- --exact --show-output
