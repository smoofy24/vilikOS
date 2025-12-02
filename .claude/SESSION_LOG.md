# Vilik Rust OS - Session Log

## 2024-12-02 - Project Initialization & UART

### What was done
- Created `.claude/` directory with project instructions
- Installed Rust nightly toolchain (1.93.0-nightly)
- Added `aarch64-unknown-none` target
- Added `rust-src` component for build-std
- Created project structure:
  - `Cargo.toml` - project manifest
  - `.cargo/config.toml` - build configuration (target, linker flags, build-std)
  - `linker.ld` - linker script (copied from C version, added `.text._start` section)
  - `boot/boot.S` - assembly startup code (copied from C version)
  - `src/main.rs` - kernel entry point with panic handler
  - `src/uart.rs` - UART driver (putc, puts)
- Successfully built and tested in QEMU
- Fixed UART address bug: `0x9000_0000` -> `0x0900_0000`

### Current state
- [x] Rust nightly toolchain installed
- [x] aarch64-unknown-none target added
- [x] Cargo.toml and project structure
- [x] Linker script
- [x] Main source file (src/main.rs)
- [x] Build and QEMU test
- [x] UART driver working - prints "Hello, UART!"

### Project structure
```
vilik-rust/
├── .cargo/
│   └── config.toml
├── boot/
│   └── boot.S
├── src/
│   ├── main.rs
│   └── uart.rs
├── Cargo.toml
├── linker.ld
└── README.md
```

### Key learnings
- Linker script needs `.text._start` section to ensure `_start` is at beginning
- UART address for QEMU virt: `0x0900_0000` (not `0x9000_0000`!)
- `#[no_mangle]` needed for functions called from assembly
- `unsafe` can be inside function or function itself can be `unsafe`
- `b'H'` is byte literal (u8), `'H'` is char (4 bytes Unicode)

### Next steps
- Add printk safe wrapper
- Implement more kernel features
