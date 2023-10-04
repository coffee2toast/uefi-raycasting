1. Cargo config.toml anlegen:

```bash
$ cat > .cargo/config.toml << EOF
[build]
target = "x86_64-unknown-uefi"

[target.x86_64-unknown-uefi]
runner = "./run.sh"
EOF
```

2. run.sh anlegen:

```bash
$ cat > run.sh << EOF
#!/bin/bash

mkdir -p $(dirname $1)/esp/efi/boot
cp $1 $(dirname $1)/esp/efi/boot/bootx64.efi
qemu-system-x86_64 -enable-kvm -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE.fd -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_VARS.fd -drive format=raw,file=fat:rw:$(dirname $1)/esp
EOF
$ chmod +x run.sh
```

Achtung: Möglicherweise muss der Pfad zu den OVMF-Dateien angepasst werden.

3. Profit!

```bash
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `/home/frederik/uefi-raycasting/./run.sh target/x86_64-unknown-uefi/debug/bootx64.efi`
```
