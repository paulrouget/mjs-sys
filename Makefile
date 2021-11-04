test:
	cargo +nightly fmt -- --check
	cargo clippy --features "platform-unix" --all -- -W clippy::all -D warnings
	cargo run --features "platform-unix" --example basic                                                                                                                                                 ~/git/mjs-sys#master
