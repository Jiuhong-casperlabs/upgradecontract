prepare:
	rustup target add wasm32-unknown-unknown

build-contract:
	cargo build -p messenger --release --target wasm32-unknown-unknown
	wasm-strip contract/target/wasm32-unknown-unknown/release/messenger_v1_install.wasm 2>/dev/null | true
	wasm-strip contract/target/wasm32-unknown-unknown/release/messenger_v1_disable.wasm 2>/dev/null | true
	wasm-strip contract/target/wasm32-unknown-unknown/release/messenger_v2_upgrade.wasm 2>/dev/null | true
	wasm-strip contract/target/wasm32-unknown-unknown/release/assert_message.wasm 2>/dev/null | true

test: build-contract
	mkdir -p tests/wasm
	cp target/wasm32-unknown-unknown/release/*.wasm tests/wasm
	cd tests && cargo test

clippy:
	cargo clippy -p messenger --all-targets -- -D warnings
	cd tests && cargo clippy --all-targets -- -D warnings

check-lint: clippy
	cargo fmt -- --check
	cd tests && cargo fmt -- --check

lint: clippy
	cargo fmt
	cd tests && cargo fmt

clean:
	cargo clean
	cd tests && cargo clean
	rm -rf tests/wasm
