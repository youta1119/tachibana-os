.PHONY: run build

build:
	cargo build --release

run: third_party/edk2/OVMF.fd
	cargo run

third_party/edk2/OVMF.fd:
	./scripts/dl_ovmf.sh
