#!/bin/sh
tmpdir=$(mktemp -d)
mkdir -p third_party/edk2

curl -L  http://downloads.sourceforge.net/project/edk2/OVMF/OVMF-X64-r15214.zip > $tmpdir/OVMF.zip
unzip -d $tmpdir $tmpdir/OVMF.zip

cp $tmpdir/OVMF.fd third_party/edk2/
