run:
	cargo run

run-nix:
	nix-shell shell.nix --run '$(MAKE) run'

nix-shell:
	nix-shell shell.nix
