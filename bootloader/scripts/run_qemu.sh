#!/bin/sh
echo $@
mkdir -p build/EFI/BOOT
cp $1 build/EFI/BOOT/BOOTx64.EFI
qemu-system-x86_64 --bios ../third_party/edk2/OVMF.fd -drive format=raw,file=fat:rw:build
