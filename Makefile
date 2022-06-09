run:
	#cargo run --features bevy/dynamic
	cargo run --target wasm32-unknown-unknown

run-nix:
	nix-shell shell.nix --run '$(MAKE) run'

nix-shell:
	nix-shell shell.nix

wasm-out:
	cargo build --release --target wasm32-unknown-unknown
	wasm-bindgen --out-dir ./wasm/ --target web ./target/wasm32-unknown-unknown/release/card-game.wasm
