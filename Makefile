test-all: test-node test-web

test-node:
	wasm-pack test --node

test-web:
	wasm-pack test --headless --chrome --firefox
