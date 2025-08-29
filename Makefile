

DOC_FLAGS = --target-dir $(DOC_OUTDIR) --no-deps --workspace
DOC_OUTDIR = ./doc
SCRIPTS_DIR = ./scripts
UNAME = $(shell uname -s | tr '[:upper:]' '[:lower:]')

install:
ifeq ($(UNAME),linux)
	bash ${SCRIPTS_DIR}/install_postgres.sh
	make mount_db
else ifeq ($(UNAME),darwin)
	echo "PostgreSQL installation for macOS is not implemented yet."
else
	echo "Unsupported OS: $(UNAME). Please install PostgreSQL manually."
endif

mount_db:
	bash ./scripts/create_db.sh

uninstall:
	bash ./scripts/remove_db.sh

dev:
	cargo run

doc:
	cargo doc ${DOC_FLAGS}

run:
	cargo run --release

test:
	cargo test