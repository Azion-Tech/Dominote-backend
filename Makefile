

DOC_FLAGS = --target-dir $(DOC_OUTDIR) --no-deps --workspace
DOC_OUTDIR = ./doc

dev:
	cargo run

doc:
	cargo doc ${DOC_FLAGS}

run:
	cargo run --release

test:
	cargo test