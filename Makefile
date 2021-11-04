check:
	rustup toolchain install nightly --component rustfmt
	cargo +nightly fmt -- --check
	cargo run --features "platform-unix" --example basic                                                                                                                                                 ~/git/mjs-sys#master
	cargo clippy --features "platform-unix" --all -- -W clippy::all -D warnings

doc:
	cargo doc --features "platform-unix"

fmt:
	cargo +nightly fmt

public:
	cargo publish --features "platform-unix"
