
all:
	cd bambu-farm-client && make shared
	cd bambu-farm-server && cargo build --target aarch64-apple-darwin

run: all
	cd bambu-farm-server && RUST_LOG=warn cargo run

