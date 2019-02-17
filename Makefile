.PHONY: build run clean

build:
	bootimage build

run: build
	qemu-system-x86_64 -drive format=raw,file=target/x86_64/debug/bootimage-small_os.bin

clean:
	rm -rf target/x86_64/debug/bootimage-small_os.bin
