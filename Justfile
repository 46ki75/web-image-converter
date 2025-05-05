install:
	cargo install cargo-license cargo-about

build:
	wasm-pack build --target web
	cargo about generate about.hbs > ./pkg/license.html
