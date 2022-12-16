For build & run need:
- cargo & rustc
- linker
- qemu

Installing rust-nightly
`rustup override set nightly`

Creating a cargo config:
`path - PROJECT_PATH/.cargo/config.toml`
config.toml:
```json
[unstable]
build-std-features = ["compiler-builtins-mem"]
build-std = ["core", "compiler_builtins"]

[build]
target = "target.json"

```

Creating a bootimage:
`cargo install bootimage`
for compilation use `cargo bootimage`

Run:
`qemu-system-x86_64 -drive format=raw,file=target/target/debug/bootimage-swan_kernel.bin`
or use grub/limine to compile kernel to iso file