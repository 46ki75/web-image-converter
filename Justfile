install:
	cargo install --locked cargo-license cargo-about wasm-pack

build:
	wasm-pack build --target web
	cargo about generate about.hbs > ./pkg/LICENSE
