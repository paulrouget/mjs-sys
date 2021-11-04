check:
	rustup toolchain install nightly --component rustfmt
	cargo +nightly fmt -- --check
	cargo run --example basic                                                                                                                                                 ~/git/mjs-sys#master
	cargo clippy --all -- -W clippy::all -D warnings

doc:
	cargo doc

fmt:
	cargo +nightly fmt

public:
	cargo publish
