# cluedOS

## Requirements

A high level overview of the main requirements:

* Rust 2018 flavour *nightly*
* qemu (**qemu-system-x86_64** emulation)
* nasm
* grub (for `boot/` experiments only)
* bootimage (`cargo install bootimage --version "^0.5.0"`)
* cargo x-build (`cargo install cargo-xbuild`)

## Usage

To run the rust version
```bash
make run
```

To experiment with GRUB and multiboot:
```bash
cd boot/
make run
```

See Makefiles for the rest
