build:
	cargo +nightly build -Z build-std=std,panic_abort --target x86_64-apple-darwin --release
	./target/x86_64-apple-darwin/release/burst

dev:
	
run:
	clear
	cargo run

watch:
	clear
	cargo watch -x run

clean:
	clear
	cargo clean

test:
	clear
	cargo test --bin Burst -- --exact --show-output
