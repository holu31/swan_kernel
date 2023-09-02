ARCH = x86_64
TARGETS_PATH = targets/
BIN_PATH = target/$(ARCH)/release/bootimage-swan_os.bin
QEMU_FLAGS = -cdrom build.iso -device isa-debug-exit,iobase=0xf4,iosize=0x04 -serial stdio

all: build.iso

build.iso: $(BIN_PATH)

	cargo build --release --target $(TARGETS_PATH)$(ARCH).json

	cp $(BIN_PATH) isodir/boot/
	grub-mkrescue -o build.iso isodir/

run_iso:
	qemu-system-$(ARCH) $(QEMU_FLAGS)