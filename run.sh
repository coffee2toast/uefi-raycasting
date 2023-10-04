#!/bin/bash

mkdir -p $(dirname $1)/esp/efi/boot
cp $1 $(dirname $1)/esp/efi/boot/bootx64.efi
qemu-system-x86_64 -enable-kvm -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE.fd -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_VARS.fd -drive format=raw,file=fat:rw:$(dirname $1)/esp
